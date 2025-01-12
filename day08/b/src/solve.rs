use std::collections::HashSet;

use crate::parse::{Operation, Puzzle};

pub fn solve(puzzle: &mut Puzzle) -> i32 {
    let mut pc_exec: HashSet<usize> = HashSet::new();

    let mut pc = 0;
    let mut accumulator: i32 = 0;

    // VV: Information about the last swapped operation (pc, accumulator value, all executed PC)
    let mut last_swap: Option<(usize, i32, HashSet<usize>)> = None;

    loop {
        // VV: when undoing a swap we return to the opcode that was last swapped
        // therefore, we need to omit swapping that very same operation
        let mut skip_swap_this_round = false;

        if pc >= puzzle.operations.len() {
            // VV: the PC is past the end of the program, the accumulator is our solution
            return accumulator;
        } else if pc_exec.contains(&pc) {
            // VV: We're about to enter an infinite loop.
            // Undo the last jmp/nop swap, and swap a different one

            if let Some((old_pc, old_accumulator, ref old_pc_exec)) = last_swap {
                pc = old_pc;
                accumulator = old_accumulator;
                pc_exec.clear();
                pc_exec = old_pc_exec.clone();
            } else {
                panic!("there's no way to execute the same operation twice without going through a Jmp");
            }

            last_swap = None;
            skip_swap_this_round = true;
        }

        let op = &puzzle.operations[pc];
        pc_exec.insert(pc);

        let swap_op = match op {
            Operation::Nop(arg) => Some(Operation::Jmp(*arg)),
            Operation::Jmp(delta) => Some(Operation::Nop(*delta)),
            _ => None,
        };

        let op = if let Some(swap_op) = swap_op {
            // VV: if this is an operation we've neve swapped before, and we have not swapped an
            // operation since the last time we undid an operation. Then, record that we just did a
            // swap and perform the swap itself (by setting op to swap_op)
            if skip_swap_this_round == false && last_swap.is_none() {
                last_swap = Some((pc, accumulator, pc_exec.clone()));
                swap_op
            } else {
                // VV: We either just reverted from swapping the current operation OR we've already
                // swapped an operation and therefore we must assume all other operations are valid
                *op
            }
        } else {
            // VV: This operation cannot be swapped (i.e. it's an Acc)
            *op
        };

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
