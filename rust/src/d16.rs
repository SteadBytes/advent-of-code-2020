use crate::str::split_once;
use std::ops::RangeInclusive;
use std::str::FromStr;

fn part_1(td: &TicketData) -> u32 {
    let (fields, _, nearby_tickets) = td;
    nearby_tickets
        .iter()
        .flat_map(|t| t.0.iter().filter(move |x| !valid_for_any(fields, x)))
        .sum()
}

/// Experimentation shows that:
/// 1. Fields can be valid for multiple offsets
/// 2. The number of valid fields for each offset is equal to the offset
///
/// Point 2 is crucial, counting the valid fields for each offset gives the following
/// pattern:
///
/// ```text
/// [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19 20]
/// ```
///
/// Thus, the field for each offset can be chosen by choosing (in the above order) the
/// first field from each valid set that hasn't been chosen already.
fn part_2(td: &TicketData) -> u64 {
    let (fields, my_ticket, nearby_tickets) = td;

    // Filter out invalid tickets
    let valid_tickets: Vec<&Ticket> = nearby_tickets
        .iter()
        .filter(|t| t.0.iter().all(|x| valid_for_any(fields, x)))
        .collect();

    // Find possible fields for each offset
    let field_idxs = 0..my_ticket.0.len();
    let mut fields_for_offsets: Vec<(usize, Vec<&Field>)> = field_idxs
        .map(|i| {
            (
                i,
                fields
                    .iter()
                    .filter(|f| valid_tickets.iter().all(|t| f.contains(&t.0[i])))
                    .collect(),
            )
        })
        .collect();
    // This has [0, 1, ..., 20] pattern described above
    fields_for_offsets.sort_by(|a, b| a.1.len().cmp(&(b.1.len())));
    // Only 20 fields so Vec::contains will likely be faster (or at least on par with)
    // than using a HashSet - See benches/d16.rs
    let mut used_fields: Vec<&Field> = Vec::with_capacity(fields_for_offsets.len());
    fields_for_offsets
        .iter()
        // Choose fields for each offset
        .map(|(offset, fields)| {
            // FIXME: Is this unwrap safe?
            let field = fields.iter().find(|&&f| !used_fields.contains(&f)).unwrap();
            used_fields.push(field);
            (offset, field)
        })
        // Extract values from my_ticket corresponding to fields beggining with "departure"
        .filter_map(|(offset, f)| {
            if f.name.starts_with("departure") {
                Some(my_ticket.0[*offset] as u64)
            } else {
                None
            }
        })
        // Voila!
        .product()
}

/// Return whether `x` is a valid value for *any* of `fields`.
fn valid_for_any(fields: &Vec<Field>, x: &u32) -> bool {
    fields.iter().any(|f| f.contains(x))
}

fn parse_input(input: &str) -> Result<(Vec<Field>, Ticket, Vec<Ticket>), ParseError> {
    // FIXME: Please god tidy this up and handle errors better!
    let mut sections = input.split("\n\n");

    // Parse fields
    let fields = sections
        .next()
        .ok_or(ParseError::MalformedInput)?
        .lines()
        .map(|l| Field::new(l))
        .collect::<Result<Vec<Field>, ParseError>>()?;

    // Parse my ticket
    let my_ticket: Ticket = sections
        .next()
        .and_then(|s| s.lines().skip(1).next()) // Skip "your ticket" line
        .ok_or(ParseError::MalformedInput)?
        .parse()?;

    // Parse nearby tickets
    let nearby_tickets: Vec<Ticket> = sections
        .next()
        .ok_or(ParseError::MalformedInput)?
        .lines()
        .skip(1)
        .map(|l| l.parse())
        .collect::<Result<Vec<Ticket>, ParseError>>()?;

    Ok((fields, my_ticket, nearby_tickets))
}

pub fn run(input: &str) {
    let td = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(&td));
    println!("Part 2: {}", part_2(&td));
}

type TicketData<'a> = (Vec<Field<'a>>, Ticket, Vec<Ticket>);

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
struct Ticket(Vec<u32>);

impl FromStr for Ticket {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let field_vals = s
            .split(",")
            .map(|s| s.parse::<u32>().map_err(|_| ParseError::InvalidTicket))
            .collect::<Result<Vec<u32>, ParseError>>()?;
        Ok(Self(field_vals))
    }
}

/// Note: This is public to provide access in benches/d16.rs
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct Field<'a> {
    pub name: &'a str,
    pub r1: RangeInclusive<u32>,
    pub r2: RangeInclusive<u32>,
}

impl<'a> Field<'a> {
    // Cannot use FromStr due to lifetime requirement
    fn new(s: &'a str) -> Result<Self, ParseError> {
        let (name, rest) = split_once(s, ":").ok_or(ParseError::MalformedInput)?;
        let (s1, s2) = split_once(rest, " or ").ok_or(ParseError::MalformedInput)?;
        let r1 = {
            let (l, h) = split_once(s1.trim(), "-").ok_or(ParseError::MalformedInput)?;
            l.parse::<u32>().map_err(|_| ParseError::MalformedInput)?
                ..=h.parse::<u32>().map_err(|_| ParseError::MalformedInput)?
        };
        let r2 = {
            let (l, h) = split_once(s2.trim(), "-").ok_or(ParseError::MalformedInput)?;
            l.parse::<u32>().map_err(|_| ParseError::MalformedInput)?
                ..=h.parse::<u32>().map_err(|_| ParseError::MalformedInput)?
        };
        Ok(Field { name, r1, r2 })
    }

    fn contains(&self, x: &u32) -> bool {
        self.r1.contains(x) || self.r2.contains(x)
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum ParseError {
    MalformedInput,
    InvalidTicket,
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
                    Field {
                        name: "class",
                        r1: 1..=3,
                        r2: 5..=7
                    },
                    Field {
                        name: "row",
                        r1: 6..=11,
                        r2: 33..=44,
                    },
                    Field {
                        name: "seat",
                        r1: 13..=40,
                        r2: 45..=50
                    }
                ],
                Ticket(vec![7, 1, 14]),
                vec![
                    Ticket(vec![7, 3, 47]),
                    Ticket(vec![40, 4, 50]),
                    Ticket(vec![55, 2, 20]),
                    Ticket(vec![38, 6, 12]),
                ]
            )
        );
    }

    #[test]
    fn part_1_example() {
        let td = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(&td), 71);
    }

    #[test]
    fn part_2_example() {
        // Part 2 example input with class & row fields replaced with "departure" fields
        let input = "\
departure location: 0-1 or 4-19
departure station: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
500,14,9";
        let td = parse_input(&input).unwrap();
        assert_eq!(part_2(&td), 132);
    }
}
