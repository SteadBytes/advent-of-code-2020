#![allow(dead_code)]

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum ParseError {
    InvalidLength,
    InvalidChar,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum Error {
    Parse(ParseError),
    NotFound,
}

type Seat = (u8, u8, u16); // max ID = 127 * 8 + 7 = 1023 < u16::MAX

/// Specification for binary space partioning sections e.g. rows, columns.
type BspSpec = (char, char, u8);

const ROW_MAX: u8 = 127;
const COL_MAX: u8 = 7;

const ROWSPEC: BspSpec = ('F', 'B', ROW_MAX);
const COLSPEC: BspSpec = ('L', 'R', COL_MAX);

/// Return `Ok(max_id)` if `seats` is not empty else `Err`.
fn part_1(seats: &[Seat]) -> Result<u16, Error> {
    seats
        .iter()
        .map(|(_, _, id)| *id)
        .max()
        .ok_or(Error::NotFound)
}

// fn part_2(seats: &[Seat]) -> Result<u16, Error> {}

fn parse_input(input: &str) -> Result<Vec<(u8, u8, u16)>, ParseError> {
    input
        .lines()
        .map(|l| {
            if l.len() != 10 {
                return Err(ParseError::InvalidLength);
            }
            assert_eq!(l.len(), 10);
            let (rowpart, colpart) = l.split_at(7);
            let row = bsp_search(ROWSPEC, rowpart)?;
            let col = bsp_search(COLSPEC, colpart)?;
            Ok((row, col, row as u16 * 8 + col as u16))
        })
        .collect()
}

fn bsp_search(spec: BspSpec, s: &str) -> Result<u8, ParseError> {
    let mut low = 0;
    let (lowc, highc, mut high) = spec;
    let mut mid = (low + high) / 2;
    for c in s.chars() {
        if c == lowc {
            high = mid;
        } else if c == highc {
            low = mid + 1;
        } else {
            return Err(ParseError::InvalidChar);
        }
        mid = (low + high) / 2;
    }
    Ok(mid)
}

pub fn run(input: &str) {
    let parsed = parse_input(input).expect("unable to parse input");
    assert!(parsed.len() > 0, "no boarding passes in input");
    println!("Part 1: {}", part_1(&parsed).unwrap());
    // /* println!("Part 2: {}", part_2(&parsed)); */
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL
";

    #[test]
    fn parse_input_example() {
        let seats = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(
            seats,
            [(44, 5, 357), (70, 7, 567), (14, 7, 119), (102, 4, 820)]
        );
    }

    #[test]
    fn part_1_example() {
        let seats = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(&seats), Ok((820)));
    }

    #[test]
    fn part_2_example() {}
}
