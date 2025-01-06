use std::usize;

use crate::parse::Puzzle;

pub fn track(puzzle: &mut Puzzle, right: usize, down: usize) -> usize {
    let mut x = 0;

    (down..puzzle.rows.len())
        .step_by(down)
        .map(|y| {
            x += right;

            if let Some('#') = puzzle.rows[y].chars().nth(x % puzzle.width) {
                1
            } else {
                0
            }
        })
        .sum()
}

pub fn solve(puzzle: &mut Puzzle) -> usize {
    let deltas: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    deltas
        .iter()
        .map(|&(right, down)| track(puzzle, right, down))
        .fold(1, |trees, current_track| trees * current_track)
}
