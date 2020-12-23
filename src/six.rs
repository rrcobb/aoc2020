use anyhow::Result;
use std::str::FromStr;
use std::collections::HashMap;

// group
// lines of characters
// set of unique characters in group
struct Group {
    characters: Vec<char>,
}

impl FromStr for Group {
    type Err = anyhow::Error;


    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let chas = string
            .chars()
            .filter(|c| !c.is_whitespace());
        
        let total = string.lines().count();

        let mut map: HashMap<char, usize> = HashMap::new();
        for cha in chas.into_iter() {
            let val = map.entry(cha).or_insert(0);
            *val += 1;
        }

        let characters = map.iter()
            .filter(|i| i.1 == &total)
            .map(|(cha, _count)| *cha)
            .collect();

        Ok(Group {
            characters,
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
