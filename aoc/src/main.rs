use structopt::StructOpt;
use anyhow::{Context, Result};

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
        _ => Ok(()),
    };
    ()
}
