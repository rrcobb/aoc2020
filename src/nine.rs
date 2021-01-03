use anyhow::Result;
use std::collections::{HashSet, HashMap};
use itertools::Itertools;

pub fn nine() -> Result<()> {
    const PREAMBLE: usize = 25;
    let content = include_str!("input/nine.txt");
    let nums: Vec<i32> = content.lines()
        .map(str::parse::<i32>)
        .filter_map(Result::ok)
        .collect();

    // map of numbers to sets of sums that they participate in
    let mut sums: HashMap<i32, HashSet<i32>> = 
        nums
        .clone()
        .iter()
        .take(PREAMBLE)
        .permutations(2)
        .map(|pair| match pair.as_slice() {
            [a, b, ..] =>  Ok((**a, (**a + **b).clone())),
            _ => Err("somehow not a permutation"),
        })
        .filter_map(Result::ok)
        .into_grouping_map()
        .collect();

    let mut invalid = 0;
    // for each number greater than the preamble
    for i in PREAMBLE..nums.len() {
        let to_check = nums[i];
        let mut next_set = HashSet::new();
        let mut found = false;
        let to_drop = nums[i - PREAMBLE];
        for (num, set) in &mut sums {
            // check that the number is one of those sets
            if set.contains(&to_check) {
                found = true;
            }
            next_set.insert(to_check + num);
            set.insert(to_check + num);
            set.remove(&(num + to_drop));
        }
        // if the new number wasn't in any of the sets
        // then it's the one, so stop and print it out
        if !found {
            println!("{} didn't follow the rule", to_check);
            invalid = to_check;
            break;
        }

        // update the map
        // drop the earliest number from the map
        sums.remove(&to_drop);
        // and add the newest number to the map, with its sets of prior numbers
        sums.insert(to_check, next_set);
    }

    // keep a index and a sum
    let mut tail = 0;
    let mut head = 0;
    let mut sum = nums[0];
    // we'll just panic if we're accessing out of bounds vecs
    while sum != invalid {
        if sum < invalid {
            // if the sum is lower than the target, increase the head, and add to the sum
            head += 1;
            sum += nums[head];
        }
        if sum > invalid {
            // if the sum is larger than the target, increase the tail, and subtract from the sum
            sum -= nums[tail];
            tail += 1;
        }
    }
    // if the sum equals the target, tail...head is the range
    dbg!("found at ", tail, head);

    // then find the smallest and largest numbers in the range
    let min = nums[tail..=head].iter().min().unwrap();
    let max  = nums[tail..=head].iter().max().unwrap();

    // and their sum is the answer
    let sum = min + max;
    dbg!(sum, min, max);

    Ok(())
}
