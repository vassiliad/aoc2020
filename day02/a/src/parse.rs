use anyhow::{bail, Context, Result};
use sscanf::sscanf;

pub struct Policy<'a> {
    pub min: u8,
    pub max: u8,
    pub letter: char,
    pub password: &'a str,
}

pub struct Puzzle<'a> {
    pub entries: Vec<Policy<'a>>,
}

impl Puzzle<'_> {
    pub fn from_str<'a>(text: &'a str) -> Result<Puzzle<'a>> {
        let mut entries: Vec<Policy<'a>> = Vec::new();

        for line in text.lines() {
            let line = line.trim();
            if line.len() == 0 {
                continue;
            }

            let entry;

            if let Ok((min, max, letter, password)) = sscanf!(line, "{u8}-{u8} {char}: {&str}") {
                entry = Policy {
                    min,
                    max,
                    letter,
                    password,
                };
            } else {
                bail!(format!("illegal line {line}"));
            }

            entries.push(entry);
        }

        Ok(Puzzle { entries })
    }
}
