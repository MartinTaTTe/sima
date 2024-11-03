use std::collections::BTreeMap;

use rand::Rng;

pub fn generate_words(amount: u32, rules: &BTreeMap<String, BTreeMap<String, u32>>) -> Option<String> { // rules must be verified
    let language = Language::new(&rules);
    let mut result: String = String::from("");

    for _ in 0..amount {
        let word: String = language.generate_word()?;
        result.push_str(&word);
        result.push_str(" ");
    }
    Some(result.trim().to_owned())
}

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

    fn build_language(rules: &BTreeMap<String, BTreeMap<String, u32>>) -> Option<Self> {
        let mut rules = rules.clone();
        let alphabet: String = rules.remove("alphabet")?.first_key_value()?.0.to_owned();
        let limits: BTreeMap<String, u32> = rules.remove("word_length")?;

        //println!("alphabet is {} and limits are {:?}", alphabet, limits);

        // Create the patterns map based on the rules
        let mut patterns: BTreeMap<String, (u32, BTreeMap<u32, String>)> = BTreeMap::new();

        for (p, m) in &rules {
            let mut sum: u32 = 0;
            let mut map: BTreeMap<u32, String> = BTreeMap::new();
            for (k, v) in m {
                sum += v;
                if !map.contains_key(&sum) { map.insert(sum, k.to_owned()); }
            }
            patterns.insert(p.to_owned(), (sum, map));
        }

        //println!("Complete map:\n{:?}", &patterns);

        let min: u32 = *limits.get("min")?;
        let avg: u32 = *limits.get("avg")?;
        let max: u32 = *limits.get("max")?;

        Some(Self {
            alphabet: alphabet,
            min: min,
            avg: avg,
            max: max,
            patterns: patterns,
        })
    }

    fn generate_word(&self) -> Option<String> { // rules must be verified
        let mut candidates: Vec<(f32, String)> = vec![];
        let mut current: String = String::from("");
        let mut word: String = String::new();

        for _ in 0..self.max {
            for i in (0..3).rev() {

                let pattern: String = if current == "" { format!(" ") }
                else {
                    let split_pos = current.char_indices().nth_back(i).unwrap_or((current.len(), ' ')).0;
                    if split_pos == current.len() { continue }

                    current[split_pos..].to_owned()
                };
                match self.patterns.get(&pattern) {
                    Some(map) => {
                        let mut terminate: u32 = 0;
                        if let Some((k, v)) = map.1.iter().next() {
                            if *v == " " { terminate = *k }
                        }
                        let start: u32 = terminate + 1;
                        let rng = rand::thread_rng().gen_range(start..=map.0);
                        let cont = map.1.range(&rng..).next().unwrap().1;
                        let continuation = if cont.contains("_") {
                            self.replace_wildcards(&cont, &map.1)
                        }
                        else {
                            cont.to_owned()
                        };

                        //println!("{}", continuation);

                        current.push_str(&continuation);

                        let len: u32 = current.len() as u32;

                        if len >= self.min && len <= self.max {
                            let mut value = if len < self.avg {
                                inverse_lerp(self.min, self.avg, len)
                            }
                            else {
                                1.0 - inverse_lerp(self.avg, self.max, len)
                            };
                            value += terminate as f32 / 5.0;
                            candidates.push((value, current.clone()));
                        }

                        //println!("current is {} and all are {:?}", current, candidates);

                        break
                    },
                    None => continue,
                }
            }
        }

        if !candidates.is_empty() {
            candidates.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
            word = candidates.first()?.1.clone();
        }

        Some(word)
    }

    fn replace_wildcards(&self, string: &String, map: &BTreeMap<u32, String>) -> String {
        let mut candidate: String = String::new();
        let mut found = false;
        while !found {
            candidate = string.replace("_", &self.get_wildcard().to_string());
            found = true;
            for (_, v) in map {
                if candidate == *v { found = false }
            }
        }
        candidate
    }

    fn get_wildcard(&self) -> char {
        let rng = rand::thread_rng().gen_range(0..self.alphabet.len());
        self.alphabet.chars().nth(rng).unwrap()
    }
}

fn inverse_lerp(left: u32, right: u32, point: u32) -> f32 {
    assert!(left <= right);
    assert!(left <= point);
    assert!(point <= right);

    if right == point { return 1.0 }
    (point - left) as f32 / (right - left) as f32
}



// array of candidates
// relatively closest in length to avg and highest relative chance of termination
// if no candidates, forced at length avg
