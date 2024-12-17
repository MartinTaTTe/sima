use std::collections::BTreeMap;
use std::fs::File;

use rand::{rngs::StdRng, SeedableRng};

use word_gen::generator::generate_words;
use word_gen::verification::verify_rules;

fn get_rules(path: &str) -> BTreeMap<String, BTreeMap<String, u32>> {
    // Read and deserialize yaml file.
    let yaml = File::open(format!("./assets/testing/{path}.yaml")).expect("YAML file not found.");
    let rules: BTreeMap<String, BTreeMap<String, u32>> = serde_yaml::from_reader(yaml).expect("YAML file wrong format.");
    rules
}

fn verify_rules_of(path: &str) {
    let rules = get_rules(path);
    verify_rules(&rules).expect("");
}

#[test]
#[should_panic]
fn verify_rules_too_small_alphabet() {
    verify_rules_of("test2");
}

#[test]
#[should_panic]
fn verify_rules_no_alphabet() {
    verify_rules_of("test3");
}

#[test]
#[should_panic]
fn verify_rules_min_0() {
    verify_rules_of("test4");
}

#[test]
#[should_panic]
fn verify_rules_no_min() {
    verify_rules_of("test5");
}

#[test]
#[should_panic]
fn verify_rules_max_less_than_min() {
    verify_rules_of("test6");
}

#[test]
#[should_panic]
fn verify_rules_no_max() {
    verify_rules_of("test7");
}

#[test]
#[should_panic]
fn verify_rules_avg_outside_interval() {
    verify_rules_of("test8");
}

#[test]
#[should_panic]
fn verify_rules_no_avg() {
    verify_rules_of("test9");
}

#[test]
#[should_panic]
fn verify_rules_no_word_length() {
    verify_rules_of("test10");
}

#[test]
fn generate_words_correct_values() {
    let rules = get_rules("test1");
    let mut rng = StdRng::seed_from_u64(0);
    let amount = 10;

    let result = generate_words(&mut rng, amount, &rules).expect("Failed to generate words.");
    let as_vec = result.split(" ").collect::<Vec<&str>>();

    assert_eq!(as_vec.len(), amount as usize);
}

#[test]
#[should_panic]
fn generate_words_invalid_yaml() {
    let rules = get_rules("test0");
    let mut rng = StdRng::seed_from_u64(0);
    let amount = 10;

    generate_words(&mut rng, amount, &rules).expect("");
}
