use anyhow::Result;
use std::collections::HashMap;

pub fn ten() -> Result<()> {
    let file = include_str!("input/ten.txt");
    let mut nums: Vec<i64> = file
        .lines()
        .map(str::parse::<i64>)
        .filter_map(Result::ok)
        .collect();

    // sort
    nums.sort();
    // prepend the 0
    nums.insert(0,0);
    // generate differences
    let diffs: Vec<i64> = nums
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect();
    // count differences of 1 and 3
    let ones = diffs.iter().filter(|&n| *n == 1).count();
    // add an extra three for the device
    let threes = diffs.iter().filter(|&n| *n == 3).count() + 1;

    dbg!("part one", ones, threes, ones * threes);

    // part 2
    // build a graph of potential connections 'in' from each number
    // add an item for the device
    let max = nums.last().unwrap() + 3;
    nums.insert(nums.len(), max);
    // build a 'graph' (map of edges)
    let graph: HashMap<i64, Vec<i64>> = nums
        .iter()
        .map(|&n| (n, nums.iter().filter(|&i| n - i > 0 && n - i < 4).map(|&i| i).collect()))
        .collect();
    // count number of paths through the graph, starting at the starting at the (max) and going
    // back, recursively
    let count = paths_in(max.clone(), &graph);

    dbg!("part two", count);
    Ok(())
}

cached::cached_key! {
    PATHS_IN: cached::UnboundCache<i64, i64> = cached::UnboundCache::new();
    Key = { num };
    fn paths_in(num: i64, map: &HashMap<i64, Vec<i64>>) -> i64 = {
        let back_nodes = &map[&num];
        match back_nodes.len() {
            0 => 1,
            _ => back_nodes.iter().map(|&n| paths_in(n, map) ).sum()
        }
    }
}
