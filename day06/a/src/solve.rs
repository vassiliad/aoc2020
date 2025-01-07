use crate::parse::Puzzle;

pub fn solve(puzzle: &mut Puzzle) -> u32 {
    puzzle
        .answers
        .iter()
        .map(|group| {
            let mut answers = 0 as u32;
            let mut unique = 0;

            group
                .iter()
                .map(|&answer| {
                    answer
                        .chars()
                        .map(|question| {
                            let idx = question as usize - 'a' as usize;
                            let answer = 1 << idx;
                            if answers & answer == 0 {
                                unique += 1;
                                answers |= answer;
                            }
                        })
                        .count()
                })
                .count();
            unique
        })
        .sum()
}
