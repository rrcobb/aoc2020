use anyhow::{Result};

pub fn one() -> Result<()> {
    let content = include_str!("input/one.txt");

    // turn content from string to array of nums
    let nums = content
        .split('\n')
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
