use crate::str::split_once;
use std::ops::RangeInclusive;

type FieldSpec<'a> = (&'a str, Vec<RangeInclusive<u32>>);
type Ticket = Vec<u32>;

fn part_1(fields: &Vec<FieldSpec>, nearby_tickets: &Vec<Ticket>) -> u32 {
    nearby_tickets
        .iter()
        .flat_map(|ticket| {
            ticket.iter().filter(|x| {
                !fields
                    .iter()
                    .any(|(_, valid_ranges)| valid_ranges.iter().any(|r| r.contains(x)))
            })
        })
        .sum()
}

fn part_2() {}

fn parse_input(input: &str) -> Result<(Vec<FieldSpec>, Ticket, Vec<Ticket>), ParseError> {
    // FIXME: Please god tidy this up and handle errors better!
    let mut sections = input.split("\n\n");
    // Parse fields
    let fields = sections
        .next()
        .ok_or(ParseError::MalformedInput)?
        .lines()
        .map(|l| {
            let (name, rest) = split_once(l, ":").ok_or(ParseError::MalformedInput)?;
            let ranges = rest
                .split(" or ")
                .map(|s| {
                    let (l, h) = split_once(s.trim(), "-").ok_or(ParseError::MalformedInput)?;
                    Ok(l.parse::<u32>().map_err(|_| ParseError::MalformedInput)?
                        ..=h.parse::<u32>().map_err(|_| ParseError::MalformedInput)?)
                })
                .collect::<Result<Vec<RangeInclusive<u32>>, ParseError>>()?;
            Ok((name, ranges))
        })
        .collect::<Result<Vec<FieldSpec>, ParseError>>()?;
    // Parse my ticket
    let my_ticket = sections
        .next()
        .map(|s| s.lines().skip(1).next().ok_or(ParseError::MalformedInput))
        .ok_or(ParseError::MalformedInput)?
        .and_then(|l| parse_ticket(l))?;

    // Parse nearby tickets
    let nearby_tickets = sections
        .next()
        .ok_or(ParseError::MalformedInput)?
        .lines()
        .skip(1)
        .map(|l| parse_ticket(l))
        .collect::<Result<Vec<Ticket>, ParseError>>()?;

    Ok((fields, my_ticket, nearby_tickets))
}

fn parse_ticket(s: &str) -> Result<Ticket, ParseError> {
    s.split(",")
        .map(|s| s.parse::<u32>().map_err(|_| ParseError::MalformedInput))
        .collect::<Result<Vec<u32>, ParseError>>()
}

pub fn run(input: &str) {
    let (fields, _, nearby_tickets) = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(&fields, &nearby_tickets));
    // println!("Part 2: {}", part_2(&parsed));
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum ParseError {
    MalformedInput,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn parse_input_example() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT).unwrap(),
            (
                vec![
                    ("class", vec![1..=3, 5..=7]),
                    ("row", vec![6..=11, 33..=44]),
                    ("seat", vec![13..=40, 45..=50])
                ],
                vec![7, 1, 14],
                vec![
                    vec![7, 3, 47],
                    vec![40, 4, 50],
                    vec![55, 2, 20],
                    vec![38, 6, 12],
                ]
            )
        );
    }

    #[test]
    fn part_1_example() {
        let (fields, _, nearby_tickets) = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(&fields, &nearby_tickets), 71);
    }

    #[test]
    fn part_2_example() {}
}
