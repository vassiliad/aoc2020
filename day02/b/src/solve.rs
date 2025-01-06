use crate::parse::Puzzle;

pub fn solve(puzzle: &mut Puzzle) -> u32 {
    puzzle
        .entries
        .iter()
        .map(|policy| {
            let min_is_letter = policy
                .password
                .chars()
                .nth(policy.min as usize - 1)
                .unwrap()
                == policy.letter;
            let max_is_letter = policy
                .password
                .chars()
                .nth(policy.max as usize - 1)
                .unwrap()
                == policy.letter;

            (min_is_letter ^ max_is_letter) as u32
        })
        .sum::<u32>()
}
