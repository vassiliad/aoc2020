use crate::parse::Puzzle;

pub fn solve(puzzle: &mut Puzzle) -> u32 {
    puzzle
        .entries
        .iter()
        .map(|policy| {
            let occurences: u8 = policy
                .password
                .chars()
                .map(|c| (c == policy.letter) as u8)
                .sum();
            (policy.min <= occurences && occurences <= policy.max) as u32
        })
        .sum::<u32>()
}
