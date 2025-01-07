use anyhow::Result;

#[derive(Debug)]
pub struct Puzzle<'a> {
    pub seats: Vec<&'a str>,
}

impl Puzzle<'_> {
    pub fn from_str<'a>(text: &'a str) -> Result<Puzzle<'a>> {
        let mut seats = vec![];

        for line in text.lines() {
            let line = line.trim();
            if line.len() == 0 {
                continue;
            }
            seats.push(line)
        }

        Ok(Puzzle { seats })
    }
}
