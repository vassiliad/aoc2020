use std::cmp;
use std::collections::HashSet;

use crate::parse::Puzzle;

pub fn solve(puzzle: &mut Puzzle) -> usize {
    let mut can_contain_gold: HashSet<&str> = HashSet::new();

    // VV: Maintain a set of bags that are known to contain "shiny gold" bags.
    // Whenever a bag B contains a shiny gold bag, or a bag that can directly/indirectly contain a
    // shiny gold bag, add B to the set.
    // Keep looping over the rules, till
    loop {
        let starting_res = can_contain_gold.len();
        for rule in puzzle.rules.iter() {
            for (_num, contained_kind) in rule.contains.iter() {
                if cmp::Ordering::Equal == contained_kind.cmp(&"shiny gold")
                    || can_contain_gold.contains(contained_kind)
                {
                    can_contain_gold.insert(rule.kind);
                    break;
                }
            }
        }

        if starting_res == can_contain_gold.len() {
            break;
        }
    }
    can_contain_gold.len()
}
