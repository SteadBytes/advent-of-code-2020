mod d01;
use std::io::{self, Read};

fn main() {
    let day = std::env::args()
        .nth(1)
        .expect("must provide a day e.g. d01");
    let run = match day.as_ref() {
        "d01" => d01::run,
        _ => panic!("must provide a valid day e.g. d01"),
    };

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    run(&input);
}
