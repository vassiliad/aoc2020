use crate::parse::Puzzle;

pub fn solve(puzzle: &mut Puzzle) -> u32 {
    puzzle
        .answers
        .iter()
        .map(|group| {
            let mut common_answers = u32::MAX;

            group
                .iter()
                .map(|&answer| {
                    let mut person = 0;

                    answer
                        .chars()
                        .map(|question| {
                            let idx = question as usize - 'a' as usize;
                            let answer = 1 << idx;
                            person |= answer;
                        })
                        .count();
                    common_answers &= person;
                })
                .count();
            common_answers.count_ones()
        })
        .sum()
}
