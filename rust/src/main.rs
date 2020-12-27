use aoc2020::*;
use std::io::{self, Read};

fn main() {
    let day = std::env::args().nth(1).expect("must provide a day e.g. 1");
    // TODO: Macro for this?
    let run = match day.as_ref() {
        "1" => d01::run,
        "3" => d03::run,
        "4" => d04::run,
        "5" => d05::run,
        "6" => d06::run,
        "7" => d07::run,
        "8" => d08::run,
        "9" => d09::run,
        "10" => d10::run,
        "11" => d11::run,
        "12" => d12::run,
        "13" => d13::run,
        "14" => d14::run,
        "15" => d15::run,
        "16" => d16::run,
        "17" => d17::run,
        "18" => d18::run,
        "19" => d19::run,
        "20" => d20::run,
        "21" => d21::run,
        "22" => d22::run,
        _ => panic!("must provide a valid day that has been implemented e.g. 1"),
    };

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    run(&input);
}
