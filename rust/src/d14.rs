use crate::str::split_once;
use std::collections::HashMap;
use Instruction::*;

fn part_1(program: &Vec<Instruction>) -> u64 {
    let mut mask: &BitMask = match program.get(0) {
        Some(Mask(m)) => m,
        _ => panic!("invalid program"), // FIXME
    };
    let mut mem: HashMap<usize, u64> = HashMap::new();

    for inst in &program[1..] {
        match inst {
            Mask(m) => mask = m,
            Mem(addr, val) => {
                mem.insert(*addr, mask.apply(*val));
            }
        }
    }

    mem.values().sum()
}

fn part_2() {}

fn parse_input(input: &str) -> Result<Vec<Instruction>, ParseError> {
    // FIXME: Parse this properly - there's many invalid inputs that would get through
    if !input.starts_with("mask") {
        return Err(ParseError::InvalidInstruction);
    }
    input
        .lines()
        .map(|l| match &l[..4] {
            "mask" => {
                let mask = &l[7..];
                if mask.len() != 36 {
                    Err(ParseError::InvalidMask(mask))
                } else {
                    Ok(Mask(BitMask::new(mask)))
                }
            }
            "mem[" => {
                let (addr, rest) =
                    split_once(&l[4..], "]").ok_or(ParseError::InvalidInstruction)?;
                let addr = addr
                    .parse::<usize>()
                    .map_err(|e| ParseError::InvalidAddress(e))?;
                let val = rest[3..]
                    .parse::<u64>()
                    .map_err(|e| ParseError::InvalidValue(e))?;
                Ok(Mem(addr, val))
            }
            _ => return Err(ParseError::InvalidInstruction),
        })
        .collect()
}

pub fn run(input: &str) {
    let program = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(&program));
    // println!("Part 2: {}", part_2(&parsed));
}

/// Apply bitmask `mask` to `x`:
/// - `0` or `1` in `mask` overwrites the corresponding bit in `x`.
/// - `X` in `mask` leaces the corresponding bit in `x` unchanged.
///
/// ```text
/// value:  000000000000000000000000000000001011  (decimal 11)
/// mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
/// result: 000000000000000000000000000001001001  (decimal 73)
/// ```
///
/// Masking is performed in two steps - an `AND` operation to set `0` bits and an `OR` operation to
/// set `1` bits. In each case, `mask` is transformed into an appropriate integer mask for the
/// desired operation:
///
/// - `AND`: replace `X`s with `1`s, leaving `1`s and `0`s unchanged.
/// - `OR`: replace `X`s *and* `1`s with `0`s, leaving `1`s unchanged.
///
/// The above example would use the following operations:
///
/// ```text
/// value:  000000000000000000000000000000001011 (decimal 11)
/// mask:   111111111111111111111111111111111101 AND
/// result: 000000000000000000000000000000001001
///
/// value:  000000000000000000000000000000001001
/// mask:   000000000000000000000000000001000000 OR
/// result: 000000000000000000000000000001001001 (decimal 73)
/// ```
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
struct BitMask<'a> {
    src: &'a str,
    and_mask: u64,
    or_mask: u64,
}

impl<'a> BitMask<'a> {
    fn new(src: &'a str) -> Self {
        let (and_mask, or_mask) =
            src.chars()
                .rev()
                .enumerate()
                .fold((0, 0), |(and_mask, or_mask), (i, c)| {
                    let power = 2_u64.pow(i as u32);
                    match c {
                        'X' => (and_mask + power, or_mask),
                        '1' => (and_mask + power, or_mask + power),
                        '0' => (and_mask, or_mask),
                        _ => panic!("invalid mask character"), // FIXME
                    }
                });
        BitMask {
            src,
            and_mask,
            or_mask,
        }
    }

    /// Apply bitmask to `x`:
    /// - `0` or `1` in `self.src overwrites the corresponding bit in `x`.
    /// - `X` in `self.src` leaves the corresponding bit in `x` unchanged.
    ///
    /// ```text
    /// value:  000000000000000000000000000000001011  (decimal 11)
    /// mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    /// result: 000000000000000000000000000001001001  (decimal 73)
    /// ```
    /// ## Implementation
    ///
    /// Masking is performed in two steps - an `AND` operation to set `0` bits and an `OR` operation to
    /// set `1` bits. In each case, `self.src` is transformed into an appropriate integer mask for the
    /// desired operation:
    ///
    /// - `AND`: replace `X`s with `1`s, leaving `1`s and `0`s unchanged.
    /// - `OR`: replace `X`s *and* `1`s with `0`s, leaving `1`s unchanged.
    ///
    /// The above example would use the following operations:
    ///
    /// ```text
    /// x:      000000000000000000000000000000001011 (decimal 11)
    /// mask:   111111111111111111111111111111111101 AND
    /// result: 000000000000000000000000000000001001
    ///
    /// x:      000000000000000000000000000000001001
    /// mask:   000000000000000000000000000001000000 OR
    /// result: 000000000000000000000000000001001001 (decimal 73)
    /// ```
    fn apply(&self, x: u64) -> u64 {
        (x & self.and_mask) | self.or_mask
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum Instruction<'a> {
    Mask(BitMask<'a>),
    Mem(usize, u64),
}

#[derive(PartialEq, Eq, Debug)]
enum ParseError<'a> {
    InvalidInstruction,
    InvalidMask(&'a str),
    InvalidAddress(std::num::ParseIntError),
    InvalidValue(std::num::ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn masking_examples() {
        let tests = [
            ("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 11, 73),
            ("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 101, 101),
            ("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 0, 64),
        ];

        for (mask_src, x, expected) in tests.iter() {
            let mask = BitMask::new(mask_src);
            assert_eq!(mask.apply(*x), *expected);
        }
    }

    #[test]
    fn parse_input_example() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT).unwrap(),
            [
                Mask(BitMask::new("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")),
                Mem(8, 11),
                Mem(7, 101),
                Mem(8, 0)
            ]
        );
    }

    #[test]
    fn part_1_example() {
        let program = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(&program), 165);
    }

    #[test]
    fn part_2_example() {}
}
