use std::collections::BTreeMap;
use std::fs::File;

use clap::{Arg, Command};
use serde_yaml;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define command line argument for path.
    let cmd = Command::new("configuration")
        .arg(
            Arg::new("rules")
                .long("language-rules")
                .short('r')
                .value_name("YAML_FILE")
                .help("Sets the source file for language rules.")
                .required(true),
        )
        .get_matches();

    // Extract path from cmd line arg.
    let path: String = cmd.get_one::<String>("rules").unwrap_or(&"".to_owned()).to_string();

    // Read and deserialize yaml file.
    let yaml = File::open(path)?;
    let rules: BTreeMap<String, BTreeMap<String, u32>> = serde_yaml::from_reader(yaml)?;

    // Verify and generate word(s).
    println!("{}", word_gen::generator::generate_words(10, &rules)?);

    Ok(())
}
