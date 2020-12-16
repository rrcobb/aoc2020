use anyhow::Result;
use std::str::FromStr;
use std::collections::HashSet;

// group
// lines of characters
// set of unique characters in group
struct Group {
    raw: String,
    characters: HashSet<char>,
}

impl FromStr for Group {
    type Err = anyhow::Error;


    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let characters = string
            .lines()
            .map(|line|
                line.chars()
                .filter(|c| !c.is_whitespace())
                .collect::<HashSet<char>>()
            ).fold_first(|memo, set| {
                set.intersection(&memo).cloned().collect()
            }).unwrap();

        Ok(Group {
            raw: string.into(),
            characters: characters,
        })
    }
}

pub fn six() -> Result<()> {
    let content = include_str!("input/six.txt");
    let total: u64 = content.split("\n\n")
        .map(str::parse::<Group>)
        .map(Result::unwrap)
        .map(|g| g.characters.len() as u64)
        .sum();

    println!("{} is the total", total);

    Ok(())
}
