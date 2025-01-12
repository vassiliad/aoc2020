use std::collections::HashSet;

use crate::parse::{Operation, Puzzle};

pub fn solve(puzzle: &mut Puzzle) -> i32 {
    let mut pc_exec: HashSet<usize> = HashSet::new();

    let mut pc = 0;
    let mut accumulator: i32 = 0;

    loop {
        let op = &puzzle.operations[pc];

        if pc_exec.contains(&pc) {
            return accumulator;
        }

        pc_exec.insert(pc);

        pc = match op {
            Operation::Nop(_) => pc + 1,
            Operation::Acc(num) => {
                accumulator += num;
                pc + 1
            }
            Operation::Jmp(delta) => (pc as i32 + delta) as usize,
        };
    }
}
