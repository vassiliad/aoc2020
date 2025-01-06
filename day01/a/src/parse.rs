use anyhow::{Context, Result};

pub struct Puzzle {
    pub entries: Vec<u32>,
}

impl Puzzle {
    pub fn from_str(text: &str) -> Result<Self> {
        let mut entries: Vec<u32> = Vec::new();

        for line in text.lines() {
            let line = line.trim();
            if line.len() == 0 {
                continue;
            }

            let entry: u32 =
                str::parse::<u32>(line).with_context(|| format!("parse integer {}", line))?;
            entries.push(entry);
        }

        Ok(Puzzle { entries })
    }
}
