use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

#[derive(Debug)]
pub struct Puzzle {
    pub operations: Vec<Operation>,
}

impl Puzzle {
    pub fn from_str(text: &str) -> Result<Puzzle> {
        let mut operations: Vec<Operation> = vec![];

        for line in text.lines() {
            let line = line.trim();
            if line.len() == 0 {
                continue;
            }
            let operation = if let Ok(arg) = sscanf::scanf!(line, "nop {i32}") {
                Operation::Nop(arg)
            } else if let Ok(arg) = sscanf::scanf!(line, "acc {i32}") {
                Operation::Acc(arg)
            } else if let Ok(arg) = sscanf::scanf!(line, "jmp {i32}") {
                Operation::Jmp(arg)
            } else {
                panic!("invalid operation {line}");
            };

            operations.push(operation);
        }

        Ok(Puzzle { operations })
    }
}
