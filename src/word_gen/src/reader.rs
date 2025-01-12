use std::collections::{btree_map::Entry::Vacant, BTreeMap};

// Creates the rules for a language based on a String.
pub fn rules_from_string(text: &str, depth: usize) -> BTreeMap<String, BTreeMap<String, u32>> {
    // Format the text to remove undesired characters and pad with single spaces before and after.
    let mut text: String = filter_string(text);
    assert!(!text.is_empty());
    text.insert(0, ' ');
    text.push(' ');

    // Get alphabet and word lengths.
    let alphabet = get_alphabet(&text);
    let word_len = get_word_lengths(&text);

    // The result to be returned.
    let mut result: BTreeMap<String, BTreeMap<String, u32>> = BTreeMap::new();
    for d in 1..=depth {
        // Window iterator for iterating through the text at all depths.
        let mut windows = char_windows(&text, d);
        let mut pattern = windows.next().unwrap().to_owned();

        for window in windows {
            // Avoid incrementing counts shorter than current depth.
            if pattern.len() < d {
                pattern = remove_preceding_words(window);
                continue
            }

            let continuation: char = window.chars().last().unwrap();

            // Add the continuation to the pattern map, increasing the count if it already exists.
            if let Vacant(e) = result.entry(pattern.to_owned()) {
                if !pattern.is_empty() {
                    let continuations: BTreeMap<String, u32> = BTreeMap::from([(" ".to_owned(), 0), (continuation.to_string(), 1)]);
                    e.insert(continuations);
                }
            } else {
                let inner = result.get_mut(&pattern).unwrap();
                if let Vacant(e) = inner.entry(continuation.to_string()) {
                    e.insert(1);
                } else {
                    let value = inner.get_mut(&continuation.to_string()).unwrap();
                    *value += 1;
                }
            }
            pattern = remove_preceding_words(window);
        }
    }

    // Add alphabet and word_length to the rules.
    result.insert("alphabet".to_owned(), BTreeMap::from([(alphabet, 0)]));
    result.insert("word_length".to_owned(), BTreeMap::from([
        ("min".to_owned(), word_len.0),
        ("avg".to_owned(), word_len.1),
        ("max".to_owned(), word_len.2),
    ]));

    result
}

// Borrowed from https://stackoverflow.com/questions/51257304/creating-a-sliding-window-iterator-of-slices-of-chars-from-a-string
// with some modifications by clippy.
fn char_windows(src: &str, win_size: usize) -> impl Iterator<Item = &str> {
    src.char_indices()
        .flat_map(move |(from, _)| {
            src[from ..].char_indices()
                .nth(win_size - 1)
                .map(|(to, c)| {
                    &src[from .. from + to + c.len_utf8()]
                })
    })
}

// Filters out all characters that are not alphabetic
fn filter_string(text: &str) -> String {
    let text: String = text.to_lowercase().chars().map(|c| if !c.is_alphabetic() { ' ' } else { c }).collect();
    let mut new_text = String::from("");

    // Replace whitespaces (/r, /n) with spaces (' ').
    let lines = text.lines();
    for line in lines {
        if line.is_empty() { continue }
        new_text.push_str(line);
        new_text.push(' ');
    }

    // Remove all duplicate spaces so the words are separated by single spaces only.
    let words: Vec<_> = new_text.split_whitespace().collect();
    words.join(" ")
}

// Calculate the min, avg and max word lengths of all words in text.
fn get_word_lengths(text: &str) -> (u32, u32, u32) {
    let mut words: Vec<u32> = text.split_whitespace().map(|word| word.len() as u32).collect();
    words.sort();

    let min = words[0];
    let max = *words.last().unwrap();
    let count = words.len() as u32;
    let sum: u32 = words.iter().sum();
    (min, sum / count, max)
}

// Get get all the characters that are used in the text and sort them in alphabetical order.
fn get_alphabet(text: &str) -> String {
    // Add each letter to the alphabet once.
    let mut alphabet = Vec::new();
    for c in text.chars() {
        if !alphabet.contains(&c) {
            alphabet.push(c);
        }
    }

    // Sort the alphabet.
    alphabet.retain(|c| !c.is_whitespace());
    alphabet.sort();
    alphabet.into_iter().collect()
}

// Remove any preceding words in the pattern, to not create patterns based on previous word endings.
fn remove_preceding_words(pattern: &str) -> String {
    // Take only the last word of the pattern, ignoring everything before the last whitespace.
    let mut last_word: String = pattern.chars().rev().take_while(|c| !c.is_whitespace()).collect();
    last_word = last_word.chars().rev().collect();
    if pattern.len() > last_word.len() {
        last_word.insert(0, ' ');
    }
    last_word
}

// TESTS BEGIN
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_windows_correct_values() {
        let text = "lorem ipsum";
        let mut windows = char_windows(text, 2);
        assert_eq!(windows.next().unwrap(), "lo");
        assert_eq!(windows.next().unwrap(), "or");
        assert_eq!(windows.next().unwrap(), "re");
        assert_eq!(windows.next().unwrap(), "em");
        assert_eq!(windows.next().unwrap(), "m ");
        assert_eq!(windows.next().unwrap(), " i");
        assert_eq!(windows.next().unwrap(), "ip");
        assert_eq!(windows.next().unwrap(), "ps");
        assert_eq!(windows.next().unwrap(), "su");
        assert_eq!(windows.next().unwrap(), "um");
        assert_eq!(windows.next(), None);
    }

    #[test]
    fn filter_string_correct_values() {
        assert_eq!(filter_string("123?a#,!"), "a");
        assert_eq!(filter_string(" multiple  \n  lines  \r  and  \n\n   return "), "multiple lines and return");
    }

    #[test]
    fn get_word_lengths_correct_values() {
        assert_eq!(get_word_lengths("a"), (1, 1, 1));
        assert_eq!(get_word_lengths("a aa aaa"), (1, 2, 3));
        assert_eq!(get_word_lengths("a a a aaaaa"), (1, 2, 5));
        assert_eq!(get_word_lengths("lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua"), (2, 5, 11));
    }

    #[test]
    fn get_alphabet_correct_values() {
        assert_eq!(get_alphabet("ba"), "ab");
        assert_eq!(get_alphabet("baa"), "ab");
        assert_eq!(get_alphabet("  b  a  a   "), "ab");
        assert_eq!(get_alphabet("random text"), "ademnortx");
    }

    #[test]
    fn remove_preceding_words_correct_values() {
        assert_eq!(remove_preceding_words("a"), "a");
        assert_eq!(remove_preceding_words(" a"), " a");
        assert_eq!(remove_preceding_words("a a"), " a");
        assert_eq!(remove_preceding_words(" a a"), " a");
        assert_eq!(remove_preceding_words("a a a"), " a");
        assert_eq!(remove_preceding_words("a "), " ");
        assert_eq!(remove_preceding_words(" a "), " ");
        assert_eq!(remove_preceding_words(" a"), " a");
        assert_eq!(remove_preceding_words("ab"), "ab");
        assert_eq!(remove_preceding_words(" ab"), " ab");
    }
}
// TESTS END
