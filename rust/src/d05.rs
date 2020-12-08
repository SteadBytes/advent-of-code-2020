use std::collections::HashSet;

const ROW_MAX: u8 = 127;
const COL_MAX: u8 = 7;

const ROWSPEC: BspSpec = ('F', 'B', ROW_MAX);
const COLSPEC: BspSpec = ('L', 'R', COL_MAX);

const fn seat_id(row: u8, col: u8) -> u16 {
    row as u16 * 8 + col as u16
}

pub fn run(input: &str) {
    let parsed = parse_input(input).expect("unable to parse input");
    assert!(parsed.len() > 0, "no boarding passes in input");
    assert_eq!(input.lines().count(), parsed.len());
    println!("Part 1: {}", part_1(&parsed).unwrap());
    println!("Part 2: {}", part_2(&parsed).unwrap());
}

/// Returns `Ok(max_id)` if `seats` is not empty else `Err`.
fn part_1(seats: &[SeatId]) -> Result<u16, Error> {
    seats.iter().max().copied().ok_or(Error::NotFound)
}

fn part_2(seats: &[SeatId]) -> Result<u16, Error> {
    // TODO: Is there a smarter way to do this? I mean, this runs fast as hell anyway
    // as the input isn't that large but I feel like I'm missing something that would
    // allow for another method other than exhaustive search.
    let occupied_seats: HashSet<&u16> = seats.iter().collect();
    let all_ids = (0..ROW_MAX).flat_map(|r| (0..COL_MAX).map(move |c| seat_id(r, c)));
    all_ids
        .filter(|id| {
            !occupied_seats.contains(&id)
                && (occupied_seats.contains(&(id - 1)) && occupied_seats.contains(&(id + 1)))
        })
        .next()
        .ok_or(Error::NotFound)
}

fn parse_input(input: &str) -> Result<Vec<SeatId>, ParseError> {
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
            Ok(seat_id(row, col))
        })
        .collect()
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum ParseError {
    InvalidLength,
    InvalidChar,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum Error {
    NotFound,
}

type SeatId = u16; // max ID = 127 * 8 + 7 = 1023 < u16::MAX

/// Specification for binary space partioning sections e.g. rows, columns.
type BspSpec = (char, char, u8);

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
        assert_eq!(seats, [357, 567, 119, 820]);
    }

    #[test]
    fn part_1_example() {
        let seats = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(&seats), Ok(820));
    }
}
