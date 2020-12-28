use clap::arg_enum;
use structopt::StructOpt;

macro_rules! import {
    ( $( $x:ident ),* ) => {
        $(
            mod $x;
            use $x::$x;
        )*
    };
}

import!(one, two, three, four, five, six, seven, eight, nine);

arg_enum! {
    #[derive(Debug)]
    enum Day {
        One, Two, Three, Four, Five, Six, Seven, Eight, Nine
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
        Seven => seven(),
        Eight => eight(),
        Nine => nine(),
    };
}
