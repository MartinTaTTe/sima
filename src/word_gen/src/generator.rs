use std::collections::{btree_map, BTreeMap};

use rand::{rngs::StdRng, RngCore};

use crate::verification;

// Generates amount number of words using rules.
pub fn generate_words<'a>(rng: &mut StdRng, amount: u32, rules: &'a BTreeMap<String, BTreeMap<String, u32>>) -> Result<String, &'a str> {
    // Verify the rules are valid.
    verification::verify_rules(rules)?;

    // Setup the language.
    let language = Language::new(rules);
    let mut result = String::from("");

    // Generate each word individually.
    for _ in 0..amount {
        let word = language.generate_word(rng)?;
        result.push_str(&word);
        result.push(' ');
    }

    Ok(result.trim().to_owned())
}

// The language object stores the rules specified in the language rules file.
struct Language {
    alphabet: String,
    min: usize,
    avg: usize,
    max: usize,
    patterns: BTreeMap<String, (u32, f32, BTreeMap<u32, String>)>,
}

impl Language {
    fn new(rules: &BTreeMap<String, BTreeMap<String, u32>>) -> Self {
        match Self::build_language(rules) {
            Some(s) => s,
            None => panic!("Failed to build language!"),
        }
    }

    // Creates the language object from the language rules file.
    fn build_language(rules: &BTreeMap<String, BTreeMap<String, u32>>) -> Option<Self> {
        // Create a new copy of rules without the alphabet and word length limits.
        let mut rules = rules.clone();
        let alphabet = rules.remove("alphabet")?.first_key_value()?.0.to_owned();
        let limits: BTreeMap<String, u32> = rules.remove("word_length")?;

        // Get the word length limits.
        let min = *limits.get("min")? as usize;
        let avg = *limits.get("avg")? as usize;
        let max = *limits.get("max")? as usize;

        // Create the patterns map based on the rules.
        let mut patterns: BTreeMap<String, (u32, f32, BTreeMap<u32, String>)> = BTreeMap::new();

        for (p, m) in &rules {
            // Cumulative weight used by rng.
            let mut sum: u32 = 0;

            // Continuation map for this pattern.
            let mut map: BTreeMap<u32, String> = BTreeMap::new();

            // Continuations with weight 0.
            let mut forbidden= Vec::new();

            // Termination weight.
            let mut termination_weight: u32 = 0;

            // Invert the inner BTreeMap so it can be used with weighted randomness.
            for (k, v) in m {
                sum += v;
                if let btree_map::Entry::Vacant(e) = map.entry(sum) {
                    e.insert(k.to_owned());
                } else {
                    forbidden.push(k.to_owned());
                }

                if k == " " {
                    termination_weight = *v;
                }
            }

            // Key of last valid continuation.
            let last = sum;

            // Calculate the chance the word should end with this pattern.
            let termination = termination_weight as f32 / sum as f32;

            // Insert forbidden continuations after the last valid continuation.
            for f in &forbidden {
                sum += 1;
                map.insert(sum, f.to_owned());
            }

            patterns.insert(p.to_owned(), (last, termination, map));
        }

        Some(Self {
            alphabet,
            min,
            avg,
            max,
            patterns,
        })
    }

    fn generate_word<'a>(&self, rng: &mut StdRng) -> Result<String, &'a str> {
        let mut candidates: Vec<(f32, String)> = vec![];
        let mut current: String = String::from(" ");
        let mut l = 0;

        while l < self.max {
            for i in (0..3).rev() {
                // Find the pattern to match against, from (at most) the i last characters of current.
                let split_pos = current.char_indices().nth_back(i).unwrap_or((current.len(), ' ')).0;
                if split_pos == current.len() { continue }
                let pattern = current[split_pos..].to_owned();

                match self.patterns.get(&pattern) {
                    Some(map) => {
                        // Get the weighted probability of the word to end on curent pattern.
                        let mut terminate: u32 = 0;
                        if let Some((k, v)) = map.2.first_key_value() {
                            if *v == " " { terminate = *k }
                        }

                        // Get the start position of continuations, excluding word termination.
                        let start = terminate + 1;

                        // If the only continuation is termination, add that and stop generating word candidates.
                        if start > map.0 {
                            candidates.push((1.0, current.clone()));
                            l = self.max;
                            break
                        }

                        // Get a random continuation. If there's only one option, choose that, else, pick one at random.
                        let r = if map.0 == start { start } else { rng.next_u32() % (map.0 - start) + start };

                        // Replace potential wildcards in the continuation.
                        let raw_continuation = map.2.range(&r..).next().unwrap().1;
                        let continuation = if raw_continuation.contains('_') {
                            self.replace_wildcards(rng, raw_continuation, &map.2).to_owned()
                        }
                        else {
                            raw_continuation.to_owned()
                        };

                        current.push_str(&continuation);

                        // If the length of the current word is acceptable, add it as a candidate with relative value.
                        let len = current.len();
                        if len >= self.min && len <= self.max {
                            // Calculate the value for the candidate based on its distance from avg and the likelihood the word should end with current pattern.
                            let value = if len < self.avg {
                                inverse_lerp(self.min, self.avg, len)
                            }
                            else {
                                1.0 - inverse_lerp(self.avg, self.max, len)
                            } + map.1;
                            candidates.push((value, current.clone()));
                        }

                        l += continuation.len();

                        break
                    },
                    None => continue,
                }
            }
        }

        // Get the candidate with the highest value.
        candidates.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        match candidates.first() {
            Some(word) => Ok(word.1.trim().to_owned()),
            None => Err("No word found."),
        }
    }

    // Replace all wildcard characters (_) in string.
    fn replace_wildcards(&self, rng: &mut StdRng, string: &str, map: &BTreeMap<u32, String>) -> String {
        let mut candidate = String::from("");
        let mut found = false;
        while !found {
            candidate = string.replace('_', &self.get_wildcard(rng).to_string());
            found = true;
            for v in map.values() {
                if candidate == *v { found = false; }
            }
        }
        candidate
    }

    // Get a random character that is not represented by the existing rules of a pattern.
    fn get_wildcard(&self, rng: &mut StdRng) -> char {
        let i = rng.next_u32() as usize % self.alphabet.len();
        self.alphabet.chars().nth(i).unwrap()
    }
}

// Similar to lerp (linear interpolation), but instead of finding the point based on the relative distance,
// it finds the relative distance based on the point.
fn inverse_lerp(left: usize, right: usize, point: usize) -> f32 {
    assert!(left <= point);
    assert!(point <= right);

    if right == left { return 1.0 }
    (point - left) as f32 / (right - left) as f32
}

// TESTS BEGIN
#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;
    use std::fs::File;

    use rand::{rngs::StdRng, SeedableRng};

    fn get_language() -> Language {
        // Read and deserialize yaml file.
        let yaml = File::open("./assets/testing/test1.yaml").expect("YAML file not found.");
        let rules: BTreeMap<String, BTreeMap<String, u32>> = serde_yaml::from_reader(yaml).expect("YAML file wrong format.");

        Language::new(&rules)
    }

    #[test]
    fn generate_word_follows_rules() {
        let language = get_language();
        let mut rng = StdRng::seed_from_u64(0);

        for _ in 0..100 {
            let word = language.generate_word(&mut rng).expect("Failed to return word.");
            if word.contains("aa") || word.contains("bbb") || word.contains("cc") {
                panic!("Word contained impossible pattern.")
            }
            if word.len() < language.min as usize {
                panic!("Word impossibly short.")
            }
            if word.len() > language.max as usize {
                panic!("Word impossibly long.")
            }
        }

    }

    #[test]
    fn replace_wildcards_does_not_replace_with_weight_0() {
        let language = get_language();
        let mut rng = StdRng::seed_from_u64(0);

        // Ensure a is never returned since it has weight 0.
        let map = &language.patterns.get("a").expect("YAML file missing pattern.").2;
        for _ in 0..100 {
            let result = language.replace_wildcards(&mut rng, "_", map);
            let result = result.as_str();
            match result {
                "a" => panic!("Impossible character returned."),
                _ => (),
            }
        }
    }

    #[test]
    fn get_wildcard_returns_all_possibilities() {
        let language = get_language();
        let mut rng = StdRng::seed_from_u64(0);

        let mut a = false;
        let mut b = false;
        let mut c = false;

        // Ensure each of a, b, c are getting returned.
        for _ in 0..100 {
            let result = language.get_wildcard(&mut rng);
            match result {
                'a' => a = true,
                'b' => b = true,
                'c' => c = true,
                _ => panic!("Impossible character returned."),
            }
        }
        assert!(a && b && c);
    }

    #[test]
    fn inverse_lerp_correct_values() {
        assert_eq!(inverse_lerp(0, 1, 0), 0.0);
        assert_eq!(inverse_lerp(0, 1, 1), 1.0);
        assert_eq!(inverse_lerp(0, 5, 3), 0.6);
        assert_eq!(inverse_lerp(5, 10, 7), 0.4);
        assert_eq!(inverse_lerp(4, 7, 6), 0.66666666);
    }

    #[test]
    #[should_panic]
    fn inverse_lerp_point_greater_than_right() {
        inverse_lerp(0, 1, 2);
    }

    #[test]
    #[should_panic]
    fn inverse_lerp_point_less_than_left() {
        inverse_lerp(1, 2, 0);
    }

    #[test]
    #[should_panic]
    fn inverse_lerp_left_greater_than_right() {
        inverse_lerp(1, 0, 0);
    }
}
// TESTS END
