use structopt::StructOpt;

macro_rules! import {
    ( $( $x:ident ),* ) => {
        $(
            mod $x;
            use $x::$x;
        )*
    };
}

import!(one, two, three, four, five);

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
        "3" => three(args.path),
        "three" => three(args.path),
        "4" => four(args.path),
        "four" => four(args.path),
        "5" => five(args.path),
        "five" => five(args.path),
        _ => {
            println!("didn't match");
            Ok(())
        },
    };
    ()
}
