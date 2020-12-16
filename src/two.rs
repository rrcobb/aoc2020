use anyhow::{Result, bail, ensure};
use std::str::FromStr;

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


pub fn two() -> Result<()> {
    let content = include_str!("input/two.txt");

    let valid_pwds: usize = content
        .split("\n")
        .map(|s| s.parse::<PasswordDay>())
        .filter_map(Result::ok)
        .filter(|pwd| pwd.valid())
        .count();

    println!("{}", valid_pwds);
    Ok(())
}

