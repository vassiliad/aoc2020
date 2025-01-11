use std::cmp;

use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct Rule<'a> {
    pub kind: &'a str,
    pub contains: Vec<(usize, &'a str)>,
}

#[derive(Debug)]
pub struct Puzzle<'a> {
    pub rules: Vec<Rule<'a>>,
}

impl Puzzle<'_> {
    pub fn from_str<'a>(text: &'a str) -> Result<Puzzle<'a>> {
        let mut rules: Vec<Rule<'a>> = vec![];

        for line in text.lines() {
            let line = line.trim();
            if line.len() == 0 {
                continue;
            }
            let line = line.strip_suffix(".").with_context(|| "strip suffix .")?;

            let mut parts = line.split(" bags contain ");
            let kind = parts
                .next()
                .with_context(|| "extracting kind of outtermost bag")?;

            let contains_str = parts.next().with_context(|| "extracting contained bags")?;
            let contains: Vec<(usize, &'a str)> = contains_str
                .split(", ")
                .filter_map(|spec| {
                    if cmp::Ordering::Equal == spec.cmp("no other bags") {
                        None
                    } else if let Ok((number, kind)) = sscanf::scanf!(spec, "{usize} {&str} bag") {
                        Some((number, kind))
                    } else if let Ok((number, kind)) = sscanf::scanf!(spec, "{usize} {&str} bags") {
                        Some((number, kind))
                    } else {
                        panic!("line {line} contains invalid contained text: {spec}")
                    }
                })
                .collect();

            rules.push(Rule { kind, contains });
        }

        Ok(Puzzle { rules })
    }
}
