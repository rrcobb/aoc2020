use structopt::StructOpt;
use anyhow::{Context, Result, bail, ensure};
use std::str::FromStr;
use std::convert::TryInto;

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
    min: u8,
    max: u8,
    letter: char,
    password: String,
}

impl FromStr for PasswordDay {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split(" ").collect();
        ensure!(parts.len() == 3, "must have three space separated parts to a line");

        let range = parts[0].split("-")
            .map(|s| s.parse::<u8>())
            .filter_map(Result::ok)
            .collect::<Vec<u8>>();
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
        let count = self.count_letter();
        count >= self.min && count <= self.max
    }

    fn count_letter(&self) -> u8 {
        self.password.matches(self.letter).count().try_into().unwrap()
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
        _ => Ok(()),
    };
    ()
}
