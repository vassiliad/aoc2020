use std::usize;

use crate::parse::{Field, Puzzle};

pub fn solve(puzzle: &mut Puzzle) -> usize {
    puzzle
        .passports
        .iter()
        .map(|passport| {
            let required_fields: usize = passport
                .fields
                .iter()
                .map(|field| match field {
                    Field::Cid(_) => 0,
                    _ => 1,
                })
                .sum::<usize>();
            (required_fields == 7) as usize
        })
        .sum::<usize>()
}
