use crate::str::split_once;
use std::collections::VecDeque;

type Deck = VecDeque<u8>;

/// Quick and simple simulation
fn part_1(mut p1: Deck, mut p2: Deck) -> u16 {
    loop {
        let p1_top = p1.pop_front().unwrap();
        let p2_top = p2.pop_front().unwrap();

        if p1_top > p2_top {
            p1.push_back(p1_top);
            p1.push_back(p2_top);
        } else {
            p2.push_back(p2_top);
            p2.push_back(p1_top);
        }

        if p1.is_empty() {
            return p2
                .iter()
                .rev()
                .enumerate()
                .map(|(i, &x)| (i + 1) as u16 * x as u16)
                .sum();
        }
        if p2.is_empty() {
            return p1
                .iter()
                .rev()
                .enumerate()
                .map(|(i, &x)| (i + 1) as u16 * x as u16)
                .sum();
        }
    }
}

fn part_2() {
    todo!()
}

fn parse_input(input: &str) -> Result<(Deck, Deck), ParseError> {
    let (p1_part, p2_part) = split_once(input, "\n\n").ok_or(ParseError::InvalidInput)?;
    let p1 = p1_part
        .lines()
        .skip(1) // "Player 1:"
        .map(|l| l.parse::<u8>())
        .collect::<Result<VecDeque<_>, _>>()
        .map_err(|e| ParseError::InvalidCard(e))?;
    let p2 = p2_part
        .lines()
        .skip(1) // "Player 2:"
        .map(|l| l.parse::<u8>())
        .collect::<Result<VecDeque<_>, _>>()
        .map_err(|e| ParseError::InvalidCard(e))?;
    // TODO: More sanity checks? e.g. complete set of cards present (contiguous set from 1..n)
    if p1.len() != p2.len() {
        return Err(ParseError::MissingCards);
    }

    Ok((p1, p2))
}

pub fn run(input: &str) {
    let (p1, p2) = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(p1.clone(), p2.clone()));
    // println!("Part 2: {}", part_2(&parsed));
}

#[derive(PartialEq, Eq, Debug)]
enum ParseError {
    InvalidInput,
    InvalidCard(std::num::ParseIntError),
    MissingCards,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn parse_input_example() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT).unwrap(),
            (
                [9, 2, 6, 3, 1].iter().copied().collect::<VecDeque<_>>(),
                [5, 8, 4, 7, 10].iter().copied().collect::<VecDeque<_>>()
            )
        );
    }

    #[test]
    fn part_1_example() {
        let (p1, p2) = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(p1, p2), 306);
    }

    // #[test]
    // fn part_2_example() {
    //     let parsed = parse_input(EXAMPLE_INPUT).unwrap();
    //     assert_eq!(part_2(&parsed),);
    // }
}
