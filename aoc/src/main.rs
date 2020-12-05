use structopt::StructOpt;
use anyhow::{Context, Result, bail, ensure};
use std::str::FromStr;
use regex::Regex;

fn one(path: Option<std::path::PathBuf>) -> Result<()> {
    let unwrapped_path = path.context("This example needs a path").unwrap();
    let content = std::fs::read_to_string(&unwrapped_path)
        .with_context(|| format!("could not read from {}", unwrapped_path.display()))?;

    // turn content from string to array of nums
    let nums = content
        .split("\n")
        .map(|s| s.parse::<i32>())
        .filter_map(Result::ok)
        .collect::<Vec<i32>>();

    'outer: for num in &nums {
        for other in &nums {
            for third in &nums {
                if num + other + third == 2020 {
                    println!("{}", num * other * third);
                    break 'outer;
                }
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
struct PasswordDay {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl FromStr for PasswordDay {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split(" ").collect();
        ensure!(parts.len() == 3, "must have three space separated parts to a line");

        let range = parts[0].split("-")
            .map(|s| s.parse::<usize>())
            .filter_map(Result::ok)
            .collect::<Vec<usize>>();
        let min = range[0];
        let max = range[1];

        let letter = match parts[1].chars().nth(0) {
            Some(l) => l,
            None => bail!("no letter in the line"),
        };
        let password = parts[2];

        Ok(PasswordDay { min, max, letter, password: password.to_string() })
    }
}

impl PasswordDay {
    fn valid(&self) -> bool {
        (self.password.chars().nth(self.min - 1) == Some(self.letter)) ^ 
            (self.password.chars().nth(self.max - 1) == Some(self.letter))
    }
}


fn two(path: Option<std::path::PathBuf>) -> Result<()> {
    let unwrapped_path = path.context("This example needs a path").unwrap();
    let content = std::fs::read_to_string(&unwrapped_path)
        .with_context(|| 
            format!("could not read from {}", unwrapped_path.display())
        )?;

    let valid_pwds: usize = content
        .split("\n")
        .map(|s| s.parse::<PasswordDay>())
        .filter_map(Result::ok)
        .filter(|pwd| pwd.valid())
        .count();

    println!("{}", valid_pwds);
    Ok(())
}

fn three(path: Option<std::path::PathBuf>) -> Result<()> {
    let unwrapped_path = path.context("This example needs a path").unwrap();
    let content = std::fs::read_to_string(&unwrapped_path)
        .with_context(|| 
            format!("could not read from {}", unwrapped_path.display())
        )?;
    let lines: Vec<&str> = content
        .trim_end()
        .split("\n")
        .collect();

    let total: usize = [
        (1,1),
        (3,1),
        (5,1),
        (7,1),
        (1,2),
    ]
        .iter()
        .map(|(right, down)| count_trees(&lines, *right, *down))
        .product();

    println!("{}", total);
    Ok(())
}

fn count_trees(lines: &Vec<&str>, right: usize, down: usize) -> usize {
    let line_len = lines[0].len();

    let mut count = 0;
    let mut pos = 0;
    for line in lines.into_iter().step_by(down) {
        if line.chars().nth(pos) == Some('#') {
            count += 1;
        }
        // increment with line wrapping
        pos = pos + right;
        if pos >= line_len {
            pos = pos - line_len
        }
    }
    println!("{}, {}: {}", right, down, count);
    count
}

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

fn four(path: Option<std::path::PathBuf>) -> Result<()> {
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

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: Option<std::path::PathBuf>,
}

fn main() {
    let args = Cli::from_args();
    let _ = match args.pattern.as_str() {
        "1" => one(args.path),
        "one" => one(args.path),
        "2" => two(args.path),
        "two" => two(args.path),
        "3" => three(args.path),
        "three" => three(args.path),
        "4" => four(args.path),
        "four" => four(args.path),
        _ => {
            println!("didn't match");
            Ok(())
        },
    };
    ()
}
