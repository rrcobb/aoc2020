use anyhow::{Context, Result};
use std::str::FromStr;
use regex::Regex;

#[derive(Debug)]
struct Passport {
    byr: Option<String>, // (Birth Year)
    iyr: Option<String>, // (Issue Year)
    eyr: Option<String>, // (Expiration Year)
    hgt: Option<String>, // (Height)
    hcl: Option<String>, // (Hair Color)
    ecl: Option<String>, // (Eye Color)
    pid: Option<String>, // (Passport ID)
    cid: Option<String>, // (Country ID)
}

impl FromStr for Passport {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<(&str, &str)> = line
            .split_whitespace()
            .map(|part| part.split(":"))
            .map(|mut splits| {
                let key = splits.nth(0); 
                let val = splits.nth(0);
                (key.unwrap(), val.unwrap())
            })
            .collect();
            
        let mut p = Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        };

        for (name, val) in parts {
            match name {
                "byr" => p.byr = Some(val.to_string()),
                "iyr" => p.iyr = Some(val.to_string()),
                "eyr" => p.eyr = Some(val.to_string()),
                "hgt" => p.hgt = Some(val.to_string()),
                "hcl" => p.hcl = Some(val.to_string()),
                "ecl" => p.ecl = Some(val.to_string()),
                "pid" => p.pid = Some(val.to_string()),
                "cid" => p.cid = Some(val.to_string()),
                _ => (),
            }
        }
        Ok(p)
    }
}

impl Passport {
    fn valid(&self) -> bool {
        self.byr_valid() &&
            self.iyr_valid() && 
            self.eyr_valid() && 
            self.hgt_valid() && 
            self.hcl_valid() && 
            self.ecl_valid() && 
            self.pid_valid()
    }

    fn byr_valid(&self) -> bool {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        match &self.byr {
            None => false,
            Some(val) => {
                match val.parse::<i32>() {
                    Ok(year) => year >= 1920 && year <= 2002,
                    _ => false,
                }
            }
        }
    }

    fn iyr_valid(&self) -> bool {
        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        match &self.iyr {
            None => false,
            Some(val) => {
                match val.parse::<i32>() {
                    Ok(year) => year >= 2010 && year <= 2020,
                    _ => false
                }
            }
        }
    }

    fn eyr_valid(&self) -> bool {
        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        match &self.eyr {
            None => false,
            Some(val) => {
                match val.parse::<i32>() {
                    Ok(year) => year >= 2020 && year <= 2030,
                    _ => false
                }
            }
        }
    }

    fn hgt_valid(&self) -> bool {
        match &self.hgt {
            None => false,
            Some(val) => {
                // hgt (Height) - a number followed by either cm or in:
                let re = Regex::new(r"^(\d+)(cm|in)$").unwrap();
                match re.captures(&val) {
                    None => false,
                    Some(caps) => {
                        // If cm, the number must be at least 150 and at most 193.
                        // If in, the number must be at least 59 and at most 76.
                        match caps.get(2).unwrap().as_str() {
                            "in" =>  {
                                let val = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                                val >= 59 && val <= 76
                            },
                            "cm" => {
                                let val = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                                val >= 150 && val <= 193
                            },
                            _ => false,
                        }
                    }
                }
            },
        }
    }

    fn hcl_valid(&self) -> bool {
        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        match &self.hcl {
            None => false,
            Some(val) => Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(&val),
        }
    }

    fn ecl_valid(&self) -> bool {
        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        match &self.ecl {
            None => false,
            Some(val) => {
                match val.as_str() {
                    "amb" => true,
                    "blu" => true,
                    "brn" => true,
                    "gry" => true,
                    "grn" => true,
                    "hzl" => true,
                    "oth" => true,
                    _ => false
                }
            }
        }
    }

    fn pid_valid(&self) -> bool {
        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        match &self.pid {
            None => false,
            Some(val) => Regex::new(r"^\d{9}$").unwrap().is_match(&val),
        }
    }
}

pub fn four(path: Option<std::path::PathBuf>) -> Result<()> {
    let unwrapped_path = path.context("This example needs a path").unwrap();
    let content = std::fs::read_to_string(&unwrapped_path)
        .with_context(|| 
            format!("could not read from {}", unwrapped_path.display())
        ).unwrap();
    let passports = content
        .split("\n\n")
        .map(|block| block.parse::<Passport>())
        .filter_map(Result::ok)
        .filter(|p| p.valid())
        .count();

    println!("{} are valid", passports);

    Ok(())
}
