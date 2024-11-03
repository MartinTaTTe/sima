use std::collections::BTreeMap;
use std::fs::File;

use clap::{Arg, Command};
use serde_yaml;

use word_gen::verification;
use word_gen::generator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define command line argument for path
    let cmd = Command::new("rules")
        .arg(
            Arg::new("rules")
                .long("language-rules")
                .value_name("YAML_FILE")
                .help("Sets the source file for language rules")
                .required(true),
        )
        .get_matches();

    // Extract path from cmd line arg
    let path: String = cmd.get_one::<String>("rules").unwrap_or(&"".to_owned()).to_string();

    //println!("{}", path);

    // Read and deserialize yaml file
    let yaml = File::open(path)?;
    let rules: BTreeMap<String, BTreeMap<String, u32>> = serde_yaml::from_reader(yaml)?;

    //println!("{:?}", rules);

    // Verify and generate word(s)
    match verification::verify_rules(&rules) {
        Ok(_) => {
            match generator::generate_words(10, &rules) {
                Some(words) => println!("{}", words),
                None => println!("Failed to generate words."),
            };
        },
        e => e?,
        //Err(e) => println!("{}", e),
    }

    Ok(())
}

// DONE step 1: take command line argument
// DONE step 2: read yaml to map
// DONE step 3: perform read verifications
// step 4: generate words
// step 5: synthesizer
