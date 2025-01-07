use crate::parse::Puzzle;

pub fn solve(puzzle: &mut Puzzle) -> usize {
    // VV: First assign people to all seats based on their tickets
    let mut seats: Vec<u8> = vec![0; 128 * 8];
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

            let idx = col + row * 8;
            seats[idx as usize] = 1;
        })
        .count();

    // VV: return the 1st seat which is empty after a series of N empty, and then
    // a series of K full seats. Where N > 0 and K > 0
    for (seat_id, _is_full) in seats
        .iter()
        .enumerate()
        .skip_while(|(_seat_id, is_full)| **is_full == 0)
        .skip_while(|(_seat_id, is_full)| **is_full == 1)
        .take(1)
    {
        return seat_id;
    }

    panic!("oh no")
}
