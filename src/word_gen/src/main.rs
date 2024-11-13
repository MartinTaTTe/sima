use std::collections::BTreeMap;
use std::fs::File;

use serde_yaml;

use word_gen::{command, generator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get language rule file path.
    let path = command::get_path();

    // Read and deserialize yaml file.
    let yaml = File::open(path)?;
    let rules: BTreeMap<String, BTreeMap<String, u32>> = serde_yaml::from_reader(yaml)?;

    // Print out 10 words.
    println!("{}", generator::generate_words(10, &rules)?);

    Ok(())
}
