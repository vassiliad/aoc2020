use anyhow::{bail, Result};

#[derive(Debug, Clone)]
pub enum Field<'a> {
    Byr(&'a str),
    Iyr(&'a str),
    Eyr(&'a str),
    Pid(&'a str),
    Cid(&'a str),
    Hgt(&'a str),
    Hcl(&'a str),
    Ecl(&'a str),
}

#[derive(Debug)]
pub struct Passport<'a> {
    pub fields: Vec<Field<'a>>,
}

#[derive(Debug)]
pub struct Puzzle<'a> {
    pub passports: Vec<Passport<'a>>,
}

impl Puzzle<'_> {
    pub fn from_str<'a>(text: &'a str) -> Result<Puzzle<'a>> {
        let mut fields = vec![];
        let mut passports = vec![];

        for line in text.lines() {
            let line = line.trim();
            if line.len() == 0 && fields.len() > 0 {
                passports.push(Passport {
                    fields: fields.clone(),
                });
                fields.clear();
                continue;
            }

            for field in line.split(" ").filter(|field| field.len() > 0) {
                let field = if let Ok(byr) = sscanf::scanf!(field, "byr:{&str}") {
                    Field::Byr(byr)
                } else if let Ok(iyr) = sscanf::scanf!(field, "iyr:{&str}") {
                    Field::Iyr(iyr)
                } else if let Ok(eyr) = sscanf::scanf!(field, "eyr:{&str}") {
                    Field::Eyr(eyr)
                } else if let Ok(pid) = sscanf::scanf!(field, "pid:{&str}") {
                    Field::Pid(pid)
                } else if let Ok(cid) = sscanf::scanf!(field, "cid:{&str}") {
                    Field::Cid(cid)
                } else if let Ok(hgt) = sscanf::scanf!(field, "hgt:{&str}") {
                    Field::Hgt(hgt)
                } else if let Ok(hcl) = sscanf::scanf!(field, "hcl:{&str}") {
                    Field::Hcl(hcl)
                } else if let Ok(ecl) = sscanf::scanf!(field, "ecl:{&str}") {
                    Field::Ecl(ecl)
                } else {
                    bail!(format!("invalid field definition {field}"))
                };

                fields.push(field);
            }
        }

        if fields.len() > 0 {
            passports.push(Passport { fields });
        }

        Ok(Puzzle { passports })
    }
}
