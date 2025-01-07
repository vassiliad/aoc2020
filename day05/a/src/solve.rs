use crate::parse::Puzzle;

pub fn solve(puzzle: &mut Puzzle) -> i32 {
    puzzle
        .seats
        .iter()
        .map(|&seat| {
            let mut row_low = 0;
            let mut row_high = 127;

            let mut col_low = 0;
            let mut col_high = 7;

            let mut col = 0;
            let mut row = 0;

            for c in seat.chars() {
                let row_delta = ((row_high - row_low) as f32 / 2.0 + 0.5) as i32;
                let col_delta = ((col_high - col_low) as f32 / 2.0 + 0.5) as i32;
                match c {
                    'F' => {
                        row_high -= row_delta;
                        row = row_low;
                    }
                    'B' => {
                        row_low += row_delta;
                        row = row_high;
                    }

                    'R' => {
                        col_low += col_delta;
                        col = col_high;
                    }
                    'L' => {
                        col_high -= col_delta;
                        col = col_low;
                    }

                    _ => panic!("Invalid seat {seat}"),
                }
            }

            col + row * 8
        })
        .max()
        .unwrap()
}
