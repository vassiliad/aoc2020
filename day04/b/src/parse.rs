use anyhow::{bail, Result};

#[derive(Debug, Clone)]
pub enum Field<'a> {
    Byr(usize),
    Iyr(usize),
    Eyr(usize),
    Pid(&'a str),
    Cid(usize),
    HgtCm(usize),
    HgtIn(usize),
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
                let field = if let Ok(byr) = sscanf::scanf!(field, "byr:{usize}") {
                    if byr < 1920 || byr > 2002 {
                        continue;
                    }
                    Field::Byr(byr)
                } else if let Ok(iyr) = sscanf::scanf!(field, "iyr:{usize}") {
                    if iyr < 2010 || iyr > 2020 {
                        continue;
                    }
                    Field::Iyr(iyr)
                } else if let Ok(eyr) = sscanf::scanf!(field, "eyr:{usize}") {
                    if eyr < 2020 || eyr > 2030 {
                        continue;
                    }
                    Field::Eyr(eyr)
                } else if let Ok(pid) = sscanf::scanf!(field, "pid:{&str}") {
                    if pid.len() != 9 {
                        continue;
                    }
                    if let Err(_) = pid.parse::<u32>() {
                        continue;
                    }
                    Field::Pid(pid)
                } else if let Ok(cid) = sscanf::scanf!(field, "cid:{usize}") {
                    Field::Cid(cid)
                } else if let Ok(hgt) = sscanf::scanf!(field, "hgt:{usize}cm") {
                    if hgt < 150 || hgt > 193 {
                        continue;
                    }
                    Field::HgtCm(hgt)
                } else if let Ok(hgt) = sscanf::scanf!(field, "hgt:{usize}in") {
                    if hgt < 59 || hgt > 76 {
                        continue;
                    }
                    Field::HgtIn(hgt)
                } else if let Ok(hcl) = sscanf::scanf!(field, "hcl:#{&str}") {
                    if hcl.len() != 6 {
                        continue;
                    }
                    Field::Hcl(hcl)
                } else if let Ok(ecl) = sscanf::scanf!(field, "ecl:{&str}") {
                    match ecl {
                        "amb" => (),
                        "blu" => (),
                        "brn" => (),
                        "gry" => (),
                        "grn" => (),
                        "hzl" => (),
                        "oth" => (),
                        _ => continue,
                    }
                    Field::Ecl(ecl)
                } else {
                    // VV: This is an invalid field, throw it away
                    continue;
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
