use anyhow::{Result, ensure};
use std::str::FromStr;

#[derive(Debug)]
struct Seat {
    line: String,
    row: usize,
    column: usize,
}

impl FromStr for Seat {
    type Err = anyhow::Error;

    fn from_str(raw_line: &str) -> Result<Self, Self::Err> {
        ensure!(raw_line.len() == 10, "lines must be 10 characters across");
        let mut row = 0;
        let two: usize = 2;
        for (i, c) in raw_line[0..7].chars().enumerate() {
            match c {
                'F' => {},
                'B' => { row += two.pow(6 - i as u32) },
                _ => {}
            }
        }
        let mut column = 0;
        for (i, c) in raw_line[7..10].chars().enumerate() {
            match c {
                'L' => {},
                'R' => { column += two.pow(2 - i as u32) },
                _ => {}
            }
        }

        let line = raw_line.to_owned();
        let seat = Seat { line, row, column };

        Ok(seat)
    }
}

impl Seat {
    fn id(&self) -> usize {
        (self.row * 8) + self.column
    }
}


pub fn five() -> Result<()> {
    let content = include_str!("input/five.txt");

    let seats = content
        .lines()
        .map(|line| line.parse::<Seat>())
        .map(|r| r.unwrap())
        .collect::<Vec<Seat>>();
    let mut ids = seats.iter().map(|s| s.id() ).collect::<Vec<usize>>();
    ids.sort_unstable();
    for (i, id) in ids.iter().enumerate() {
        if i > 0 && id - 1 != ids[i -1] {
            println!("{}", id -1);
        }
    }
    Ok(())
}
