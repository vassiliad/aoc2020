use anyhow::Result;

#[derive(Debug)]
pub struct Puzzle<'a> {
    pub rows: Vec<&'a str>,
    pub width: usize,
}

impl Puzzle<'_> {
    pub fn from_str<'a>(text: &'a str) -> Result<Puzzle<'a>> {
        let mut rows: Vec<&'a str> = Vec::new();
        let mut width: usize = 0;

        for line in text.lines() {
            let line = line.trim();
            if line.len() == 0 {
                continue;
            }
            width = line.len();
            rows.push(line);
        }

        Ok(Puzzle { rows, width })
    }
}
