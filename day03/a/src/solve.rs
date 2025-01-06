use crate::parse::Puzzle;

pub fn solve(puzzle: &mut Puzzle) -> u32 {
    let mut x = 0;

    (1..(puzzle.rows.len()))
        .map(|y| {
            x += 3;

            if let Some('#') = puzzle.rows[y].chars().nth(x % puzzle.width) {
                1
            } else {
                0
            }
        })
        .sum()
}
