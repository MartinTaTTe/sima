use std::collections::BTreeMap;

pub fn verify_rules(rules: &BTreeMap<String, BTreeMap<String, u32>>) -> Result<(), String> {
    // Verify the alphabet is defined properly.
    let mut alphabet: String = match rules.get("alphabet") {
        Some(v) => {
            match v.first_key_value() {
                Some((k, _v)) => if k.len() < 1 { return Err(format!("Alphabet is too small.")) } else { k.to_owned() },
                None => return Err(format!("No alphabet in alphabet.")),
            }
        }
        None => return Err(format!("No alphabet in rules.")),
    };
    alphabet.push('_');
    alphabet.push(' ');

    // Verify the word_length is defined properly.
    match rules.get("word_length") {
        Some(v) => {
            let min = match v.get("min") {
                Some(v) => if *v == 0 { return Err(format!("Min can't be 0.")) } else { v },
                None => return Err(format!("No min in word_length.")),
            };
            let max = match v.get("max") {
                Some(v) => if *v < *min { return Err(format!("Max can't be less than min.")) } else { v },
                None => return Err(format!("No max in word_length.")),
            };
            match v.get("avg") {
                Some(v) => if *v < *min || *v > *max { return Err(format!("Avg can't be less than min or more than max.")) },
                None => return Err(format!("No avg in word_length.")),
            }
        }
        None => return Err(format!("No word_length in rules."))
    }

    // Verify every letter rule is defined properly.
    for (k, v) in rules {
        if k == "alphabet" || k == "word_length" {
            continue
        }

        in_alphabet(&k, &alphabet)?;

        for (k, _v) in v {
            in_alphabet(&k, &alphabet)?;
        }
    }
    Ok(())
}

fn in_alphabet(pattern: &String, alphabet: &String) -> Result<(), String> {
    for l in pattern.chars() {
        if !alphabet.contains(l) {
            return Err(format!("{} not in alphabet, from pattern {}", l, pattern))
        }
    }
    Ok(())
}

// Add rules to prevent:
// alphabet: a
// a:
//    a: 0
//    _: 1
// Each pattern must contain " ":
