use crate::parse::Puzzle;

pub fn solve(puzzle: &mut Puzzle) -> u32 {
    for (i, a) in puzzle.entries.iter().enumerate() {
        for (j, b) in puzzle.entries.iter().enumerate().skip(i) {
            for c in puzzle.entries.iter().skip(j) {
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }

    panic!("oh no")
}
