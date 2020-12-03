#![allow(dead_code)]
use std::fmt;

type Grid = Vec<Vec<Square>>;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum Error {
    Parse,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum Square {
    Open,
    Tree,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Square::Open => ".",
                Square::Tree => "#",
            }
        )
    }
}

fn part_1(g: &Grid) -> i32 {
    let slope: (usize, usize) = (3, 1);
    let width = g[0].len();
    let height = g.len();

    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;

    while y < height {
        if g[y][x] == Square::Tree {
            trees += 1;
        }
        x = (x + slope.0) % width;
        y += slope.1;
    }

    trees
}

// fn part_2() {}

fn parse_input(input: &str) -> Result<Vec<Vec<Square>>, Error> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Ok(Square::Open),
                    '#' => Ok(Square::Tree),
                    _ => Err(Error::Parse),
                })
                .collect()
        })
        .collect()
}

fn grid_to_string(g: &Grid) -> String {
    g.iter()
        .map(|r| {
            r.iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("\n")
}
pub fn run(input: &str) {
    let grid = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(&grid));
    // println!("Part 2: {}", part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    // '.' = open
    // '#' = tree
    const EXAMPLE_INPUT: &str = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn parse_input_example() {
        // String -> Grid -> String round trip test as I really don't feel like typing
        // out [[Open, Open, Tree...], ...]
        let grid = parse_input(EXAMPLE_INPUT).expect("unable to parse input");
        assert_eq!(grid_to_string(&grid), EXAMPLE_INPUT);
    }

    #[test]
    fn part_1_example() {
        let grid = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(&grid), 7);
    }

    #[test]
    fn part_2_example() {}
}
