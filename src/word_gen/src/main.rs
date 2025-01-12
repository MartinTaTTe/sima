use rand::{rngs::StdRng, SeedableRng};

use word_gen::{command, generator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rules = command::get_rules(true)?;

    // Create the rng from a seed.
    let seed = rand::random::<u64>();
    println!("Used seed: {seed}");
    let mut rng = StdRng::seed_from_u64(seed);

    // Print out 10 words.
    println!("{}", generator::generate_words(&mut rng, 10, &rules)?);

    Ok(())
}
