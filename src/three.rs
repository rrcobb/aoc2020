use anyhow::Result;

pub fn three() -> Result<()> {
    let content = include_str!("input/three.txt");

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
