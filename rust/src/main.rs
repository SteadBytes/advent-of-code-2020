mod d01;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;

use std::io::{self, Read};

fn main() {
    let day = std::env::args().nth(1).expect("must provide a day e.g. 1");
    let run = match day.as_ref() {
        "1" => d01::run,
        "3" => d03::run,
        "4" => d04::run,
        "5" => d05::run,
        "6" => d06::run,
        "7" => d07::run,
        "8" => d08::run,
        _ => panic!("must provide a valid day that has been implemented e.g. 1"),
    };

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    run(&input);
}
