use structopt::StructOpt;
use clap::arg_enum;

macro_rules! import {
    ( $( $x:ident ),* ) => {
        $(
            mod $x;
            use $x::$x;
        )*
    };
}

import!(one, two, three, four, five, six);

arg_enum! {
    #[derive(Debug)]
    enum Day {
        One, Two, Three, Four, Five, Six
    }
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(possible_values = &Day::variants(), case_insensitive = true)]
    day: Day,
}

fn main() {
    let args = Cli::from_args();
    use self::Day::*;
    let _ = match args.day {
        One => one(),
        Two => two(),
        Three => three(),
        Four => four(),
        Five => five(),
        Six => six(),
    };
    ()
}
