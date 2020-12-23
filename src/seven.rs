use anyhow::Result;
use std::str::FromStr;
use std::collections::HashMap;
use regex::Regex;

type Color = String;

#[derive(Debug)]
struct Rule {
    parent: Color,
    children: Vec<(Color, u32)>,
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> { 
        let mut parts = string.split(" bags contain ");
        let parent = parts.next().unwrap();
        let parent = parent.into();
        let re = Regex::new(r"(\d+)(.+)bag")?;
        let children = parts.next()
            .unwrap_or_else(|| panic!("failed on {:?}", string))
            .split(',')
            .map(|child_bag| match re.captures(&child_bag) {
                    None => Err("child bag didn't match regex"),
                    Some(caps) => {
                        let num_bags = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
                        let child_color: String = caps.get(2).unwrap().as_str().trim().into();
                        Ok((child_color, num_bags))
                    }
                })
            .filter_map(Result::ok)
            .collect();

        Ok(Self {parent, children})
    }
}

pub fn seven() -> Result<()> {
    let content = include_str!("input/seven.txt");
    let mut child_to_parents: HashMap<Color, Vec<Color>> = HashMap::new();
    let rules: Vec<Rule> = content
        .lines()
        .map(str::parse::<Rule>)
        .filter_map(Result::ok)
        .map(|rule| {
            for (child, _) in rule.children.iter() {
                child_to_parents.entry(child.to_string()).or_insert(vec![]);
                child_to_parents.get_mut(child).unwrap().push(rule.parent.clone());
            }
            rule
        })
        .collect();
    let mut to_find = Vec<Color> = vec!["shiny gold".into()];

    Ok(())
}

// we have a shiny gold bag
// find the number of bags that could, theoretically, hold a shiny gold bag
// find all the holders of a shiny gold bag
// find all the holders of those bags

// let parent_map = HashMap<Color, Vec<Color>>;
// let children_map = HashMap<Color, Vec<(Color, num)>>;
// 
// walk from shiny gold
//     - collect each parent
//     - collect the parent of each parent
//     - mark nodes as visited
//     - or if we don't care, add to Set
