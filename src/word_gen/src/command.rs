use std::path::Path;

use clap::{Arg, Command};

pub fn get_path() -> String {
    // Define command for file path.
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
    let mut path: String = cmd.get_one::<String>("rules").unwrap_or(&"".to_owned()).to_owned();
    
    // Prepend correct paths if needed.
    if !path.contains("assets") {
        let p = format!(".\\assets\\local\\{}", &path);
        if Path::new(&p).exists() {
            path.insert_str(0, ".\\assets\\local\\");
        }
        else {
            path.insert_str(0, ".\\assets\\examples\\");
        }
    }
    path
}
