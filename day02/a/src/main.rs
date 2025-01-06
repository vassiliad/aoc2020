pub mod parse;
pub mod solve;

use std::time::Instant;

use crate::parse::Puzzle;
use crate::solve::solve;
use std::env;

use anyhow::{Context, Result};

#[cfg(test)]
#[path = "../tests/puzzle_test.rs"]
mod tests;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut input_file: &str = "input.txt";
    if args.len() == 2 {
        input_file = args[1].as_str();
    }

    let contents = std::fs::read_to_string(input_file)
        .with_context(|| format!("Read input file {:?}", input_file))?;
    let mut puzzle =
        Puzzle::from_str(&contents).with_context(|| "Parse the contents of the file")?;

    let start = Instant::now();
    let solution = solve(&mut puzzle);
    let duration = Instant::now() - start;

    println!("Duration {duration:?}");
    println!("Solution is {solution}");

    Ok(())
}
