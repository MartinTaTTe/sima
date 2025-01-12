use std::collections::BTreeMap;

// Verifies the BTreeMap read from the yaml file to ensure it is valid and contains necessary information.
pub fn verify_rules(rules: &BTreeMap<String, BTreeMap<String, u32>>) -> Result<(), &str> {
    // Verify the alphabet is defined properly.
    let mut alphabet: String = match rules.get("alphabet") {
        Some(v) => {
            match v.first_key_value() {
                Some((k, _)) => if k.len() < 2 { return Err("Alphabet is too small.") } else { k.to_owned() },
                None => return Err("No alphabet in alphabet."),
            }
        }
        None => return Err("No alphabet in rules."),
    };
    alphabet.push('_');
    alphabet.push(' ');

    // Verify the word_length is defined properly.
    match rules.get("word_length") {
        Some(v) => {
            let min = match v.get("min") {
                Some(v) => if *v == 0 { return Err("Min can't be 0.") } else { v },
                None => return Err("No min in word_length."),
            };
            let max = match v.get("max") {
                Some(v) => if *v < *min { return Err("Max can't be less than min.") } else { v },
                None => return Err("No max in word_length."),
            };
            match v.get("avg") {
                Some(v) => if *v < *min || *v > *max { return Err("Avg can't be less than min or more than max.") },
                None => return Err("No avg in word_length."),
            }
        }
        None => return Err("No word_length in rules.")
    }

    // Verify every letter rule is defined using only characters in the alphabet.
    for (k, v) in rules {
        if k == "alphabet" || k == "word_length" {
            continue
        }

        in_alphabet(k, &alphabet)?;

        for k in v.keys() {
            in_alphabet(k, &alphabet)?;
        }
    }
    Ok(())
}

// Helper function to verify that each character in pattern exists in alphabet.
fn in_alphabet<'a, 'b>(pattern: &'a str, alphabet: &'a str) -> Result<(), &'b str> {
    for l in pattern.chars() {
        if !alphabet.contains(l) {
            return Err("Character not in alphabet used.")
        }
    }
    Ok(())
}

//TESTS BEGIN
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_alphabet_correct_values() {
        let alphabet = "abc";
        let error = Err("Character not in alphabet used.");

        // Patterns in alphabet.
        assert_eq!(in_alphabet("aaa", alphabet), Ok(()));
        assert_eq!(in_alphabet("abc", alphabet), Ok(()));
        assert_eq!(in_alphabet("cba", alphabet), Ok(()));

        // Patterns not in alphabet.
        assert_eq!(in_alphabet("d", alphabet), error);
        assert_eq!(in_alphabet("ad", alphabet), error);
        assert_eq!(in_alphabet("da", alphabet), error);
    }
}
// TESTS END
