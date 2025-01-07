use anyhow::Result;

#[derive(Debug)]
pub struct Puzzle<'a> {
    pub answers: Vec<Vec<&'a str>>,
}

impl Puzzle<'_> {
    pub fn from_str<'a>(text: &'a str) -> Result<Puzzle<'a>> {
        let mut answers = vec![];
        let mut group = vec![];

        for line in text.lines() {
            let line = line.trim();
            if line.len() == 0 {
                if group.len() > 0 {
                    answers.push(group.clone());
                    group.clear();
                }
                continue;
            }
            group.push(line)
        }

        if group.len() > 0 {
            answers.push(group)
        }

        Ok(Puzzle { answers })
    }
}
