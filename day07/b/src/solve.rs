use std::collections::HashMap;

use crate::parse::Puzzle;

pub fn solve(puzzle: &mut Puzzle) -> usize {
    let mut graph: HashMap<&str, usize> = HashMap::new();

    for (idx, rule) in puzzle.rules.iter().enumerate() {
        graph.insert(rule.kind, idx);
    }

    // VV: Starting from the shiny gold bag, cound how many bags it contains, including itself
    let mut bags = 0;
    let mut pending = vec![(1, "shiny gold")];

    while let Some((number, kind)) = pending.pop() {
        if let Some(idx) = graph.get(kind) {
            let rule = &puzzle.rules[*idx];
            bags += number;

            for (num, kind) in rule.contains.iter() {
                pending.push((num * number, kind));
            }
        } else {
            panic!();
        }
    }

    // VV: report one less bag - we don't include the shiny gold bag in the result
    bags - 1
}
