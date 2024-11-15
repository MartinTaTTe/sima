use std::collections::BTreeMap;
use std::fs::File;

use rand::{rngs::StdRng, SeedableRng};
use serde_yaml;

use word_gen::{command, generator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get language rule file path.
    let path = command::get_path();

    // Read and deserialize yaml file.
    let yaml = File::open(path)?;
    let rules: BTreeMap<String, BTreeMap<String, u32>> = serde_yaml::from_reader(yaml)?;

    // Create the rng from a seed.
    let seed = rand::random::<u64>();
    println!("Used seed: {seed}");
    let mut rng = StdRng::seed_from_u64(seed);

    // Print out 10 words.
    println!("{}", generator::generate_words(&mut rng, 10, &rules)?);

    Ok(())
}
