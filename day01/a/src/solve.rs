use crate::parse::Puzzle;

pub fn solve(puzzle: &mut Puzzle) -> u32 {
    for (i, a) in puzzle.entries.iter().enumerate() {
        for b in puzzle.entries.iter().skip(i) {
            if a + b == 2020 {
                return a * b;
            }
        }
    }

    panic!("oh no")
}
