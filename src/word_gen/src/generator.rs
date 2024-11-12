use std::collections::BTreeMap;

use rand::Rng;

use crate::verification;

// Generates amount number of words using rules.
pub fn generate_words(amount: u32, rules: &BTreeMap<String, BTreeMap<String, u32>>) -> Result<String, &str> {
    // Verify the rules are valid.
    verification::verify_rules(&rules)?;

    // Setup the language.
    let language = Language::new(&rules);
    let mut result = String::from("");

    // Generate each word individually.
    for _ in 0..amount {
        let word = language.generate_word()?;
        result.push_str(&word);
        result.push_str(" ");
    }

    Ok(result.trim().to_owned())
}

// The language object stores the rules specified in the language rules file.
struct Language {
    alphabet: String,
    min: u32,
    avg: u32,
    max: u32,
    patterns: BTreeMap<String, (u32, BTreeMap<u32, String>)>,
}

impl Language {
    fn new(rules: &BTreeMap<String, BTreeMap<String, u32>>) -> Self {
        match Self::build_language(&rules) {
            Some(s) => s,
            None => panic!("Failed to build language!"),
        }
    }

    // Creates the language object from the language rules file.
    fn build_language(rules: &BTreeMap<String, BTreeMap<String, u32>>) -> Option<Self> {
        // Create a new copy of rules without the alphabet and word length limits.
        let mut rules = rules.clone();
        let alphabet: String = rules.remove("alphabet")?.first_key_value()?.0.to_owned();
        let limits: BTreeMap<String, u32> = rules.remove("word_length")?;

        // Get the word length limits.
        let min: u32 = *limits.get("min")?;
        let avg: u32 = *limits.get("avg")?;
        let max: u32 = *limits.get("max")?;

        // Create the patterns map based on the rules.
        let mut patterns: BTreeMap<String, (u32, BTreeMap<u32, String>)> = BTreeMap::new();

        for (p, m) in &rules {
            // Cumulative weight used by rng.
            let mut sum: u32 = 0;

            // Continuation map for this pattern.
            let mut map: BTreeMap<u32, String> = BTreeMap::new();

            // Continuations with weight 0.
            let mut forbidden= Vec::new();

            // Invert the inner BTreeMap so it can be used with weighted randomness.
            for (k, v) in m {
                sum += v;
                if map.contains_key(&sum) {
                    forbidden.push(k.to_owned());
                }
                else {
                    map.insert(sum, k.to_owned());
                }
            }

            // Key of last valid continuation.
            let last = sum;

            // Insert forbidden continuations after the last valid continuation.
            for f in &forbidden {
                sum += 1;
                map.insert(sum, f.to_owned());
            }

            patterns.insert(p.to_owned(), (last, map));
        }

        Some(Self {
            alphabet: alphabet,
            min: min,
            avg: avg,
            max: max,
            patterns: patterns,
        })
    }

    fn generate_word<'a>(&self) -> Result<String, &'a str> {
        let mut candidates: Vec<(f32, String)> = vec![];
        let mut current: String = String::from("");
        let mut word: String = String::new();
        let mut l = 0;

        while l < self.max {
            for i in (0..3).rev() {
                // Find the pattern to match against.
                let pattern: String = if current == "" {
                    format!(" ")
                }
                else {
                    // Get the i last characters of current.
                    let split_pos = current.char_indices().nth_back(i).unwrap_or((current.len(), ' ')).0;
                    if split_pos == current.len() { continue }
                    current[split_pos..].to_owned()
                };

                match self.patterns.get(&pattern) {
                    Some(map) => {
                        // Get the weighted probability of the word to end on curent pattern.
                        let mut terminate: u32 = 0;
                        if let Some((k, v)) = map.1.first_key_value() {
                            if *v == " " { terminate = *k }
                        }

                        // Get a random continuation, excluding word termination.
                        let start: u32 = terminate + 1;
                        let rng = rand::thread_rng().gen_range(start..=map.0);
                        let cont = map.1.range(&rng..).next().unwrap().1;
                        let continuation = if cont.contains("_") {
                            self.replace_wildcards(&cont, &map.1)
                        }
                        else {
                            cont.to_owned()
                        };

                        current.push_str(&continuation);

                        // If the length of the current word is acceptable, add it as a candidate with relative value.
                        let len: u32 = current.len() as u32;
                        if len >= self.min && len <= self.max {
                            let mut value = if len < self.avg {
                                inverse_lerp(self.min, self.avg, len)
                            }
                            else {
                                1.0 - inverse_lerp(self.avg, self.max, len)
                            };
                            // Calculate the relative value based on distance from avg length and weighted probability of the word to end * 5.
                            value += terminate as f32 / 5.0;
                            candidates.push((value, current.clone()));
                        }

                        l += continuation.len() as u32;

                        break
                    },
                    None => continue,
                }
            }
        }

        // Get the candidate with the highest value.
        if !candidates.is_empty() {
            candidates.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
            word = match candidates.first() {
                Some(word) => word.1.clone(),
                None => return Err("No word found."),
            }
        }

        Ok(word)
    }

    // Replace all wildcard characters (_) in string.
    fn replace_wildcards(&self, string: &String, map: &BTreeMap<u32, String>) -> String {
        let mut candidate: String = String::new();
        let mut found = false;
        while !found {
            candidate = string.replace("_", &self.get_wildcard().to_string());
            found = true;
            for (_, v) in map {
                if candidate == *v { found = false; }
            }
        }
        candidate
    }

    // Get a random character that is not represented by the existing rules of a pattern.
    fn get_wildcard(&self) -> char {
        let rng = rand::thread_rng().gen_range(0..self.alphabet.len());
        self.alphabet.chars().nth(rng).unwrap()
    }
}

// Similar to lerp (linear interpolation), but instead of finding the point based on the relative distance,
// it finds the relative distance based on the point.
fn inverse_lerp(left: u32, right: u32, point: u32) -> f32 {
    assert!(left <= point);
    assert!(point <= right);

    if right == left { return 1.0 }
    (point - left) as f32 / (right - left) as f32
}
