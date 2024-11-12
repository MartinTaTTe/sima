use std::collections::BTreeMap;
use std::fs::File;
use std::path::Path;

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
    let mut path: String = cmd.get_one::<String>("rules").unwrap_or(&"".to_owned()).to_string();

    // Prepend correct paths if needed.
    if !path.contains("assets") {
        let p = format!(".\\assets\\local\\{}", path);
        if Path::new(&p).exists() {
            path.insert_str(0, ".\\assets\\local\\");
        }
        else {
            path.insert_str(0, ".\\assets\\examples\\");
        }
    }

    // Read and deserialize yaml file.
    let yaml = File::open(path)?;
    let rules: BTreeMap<String, BTreeMap<String, u32>> = serde_yaml::from_reader(yaml)?;

    // Print out 10 words.
    println!("{}", word_gen::generator::generate_words(10, &rules)?);

    Ok(())
}
