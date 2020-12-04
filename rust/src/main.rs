mod d01;
mod d03;
mod d04;
use std::io::{self, Read};

fn main() {
    let day = std::env::args()
        .nth(1)
        .expect("must provide a day e.g. d01");
    let run = match day.as_ref() {
        "d01" => d01::run,
        "d03" => d03::run,
        "d04" => d04::run,
        _ => panic!("must provide a valid day e.g. d01"),
    };

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    run(&input);
}
