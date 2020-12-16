use crate::str::split_once;
use std::collections::HashMap;
use Instruction::*;

fn part_1(program: &Vec<Instruction>) -> u64 {
    let mut mask = match program.get(0) {
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

fn part_2(program: &Vec<Instruction>) -> u64 {
    let mut mask = match program.get(0) {
        Some(Mask(m)) => FloatingBitMask::new(m.src),
        _ => panic!("invalid program"), // FIXME
    };
    let mut mem: HashMap<u64, u64> = HashMap::new();

    for inst in &program[1..] {
        match inst {
            Mask(m) => mask = FloatingBitMask::new(m.src),
            Mem(addr, val) => {
                for masked_addr in mask.apply_all(*addr as u64) {
                    mem.insert(masked_addr, *val);
                }
            }
        }
    }

    mem.values().sum()
}

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
    println!("Part 2: {}", part_2(&program));
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
struct BitMask<'a> {
    /// Original mask e.g. `XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X`
    src: &'a str,
    /// `AND` mask derived from `self.src` (see `BitMask.apply`)
    and_mask: u64,
    /// `OR` mask derived from `self.src` (see `BitMask.apply`)
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
    /// x:      000000000000000000000000000000001012  (decimal 11)
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
struct FloatingBitMask<'a> {
    src: &'a str,
    /// `OR` mask to set `1` and `0` bits as per `self.src`
    or_mask: u64,
    /// Mask with `1`s corresponding to positions of `X`s in `self.src`
    x_mask: u64,
    /// `XOR` masks for each combination of floating bit values in `self.src`.
    combo_masks: Vec<u64>,
}

impl<'a> FloatingBitMask<'a> {
    fn new(src: &'a str) -> Self {
        // TODO: Clean this up :/
        let (or_mask, x_mask, mut combo_masks) = src.chars().rev().enumerate().fold(
            (0, 0, Vec::new()),
            |(or_mask, x_mask, mut combo_masks), (i, c)| {
                let power = 2_u64.pow(i as u32);
                match c {
                    'X' => {
                        let x_bit = 1 << i;
                        // Create new combinations with this bit included for all previous
                        // combinations
                        for i in 0..combo_masks.len() {
                            let val = *combo_masks.get(i).unwrap();
                            combo_masks.push(val | x_bit);
                        }
                        // Just this bit by itself
                        combo_masks.push(x_bit as u64);
                        (or_mask, x_mask + power, combo_masks)
                    }
                    '1' => (or_mask + power, x_mask, combo_masks),
                    '0' => (or_mask, x_mask, combo_masks),
                    _ => panic!("invalid mask character"), // FIXME
                }
            },
        );
        // One combination needs all floating bits set to 0
        combo_masks.push(0);

        FloatingBitMask {
            src,
            or_mask,
            x_mask,
            combo_masks,
        }
    }

    /// Apply *all* values of the floating bitmask to `x`.
    /// - `0` in `self.src` leaves the corresponding bit in `x` unchanged.
    /// - `1` in `self.src` sets the corresponding bit in `x` to `1`.
    /// - `X` in `self.src` represents a *floating* bit with value `0` **and** `1` at the same
    ///   time. Thus generating *multiple* concrete masks from this bitmask.
    ///
    /// Initial mask application:
    /// ```text
    /// x:       000000000000000000000000000000101010  (decimal 42)
    /// mask:    000000000000000000000000000000X1001X
    /// result:  000000000000000000000000000000X1101X
    /// ```
    /// `result` contains 2 floating bits which correspond to 4 concrete values:
    ///
    /// ```text
    /// 000000000000000000000000000000011010  (decimal 26)
    /// 000000000000000000000000000000011011  (decimal 27)
    /// 000000000000000000000000000000111010  (decimal 58)
    /// 000000000000000000000000000000111011  (decimal 59)
    /// ```
    fn apply_all(&'a self, x: u64) -> impl Iterator<Item = u64> + 'a {
        // Set 1s and 0s according to self.or_mask and set *all* floating bits to 0.
        let x = (x | self.or_mask) & !self.x_mask;
        // Apply all floating bit combinations
        self.combo_masks.iter().map(move |mask| x ^ mask)
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

    const EXAMPLE_INPUT_1: &str = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    const EXAMPLE_INPUT_2: &str = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

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
            parse_input(EXAMPLE_INPUT_1).unwrap(),
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
        let program = parse_input(EXAMPLE_INPUT_1).unwrap();
        assert_eq!(part_1(&program), 165);
    }

    #[test]
    fn part_2_example() {
        let program = parse_input(EXAMPLE_INPUT_2).unwrap();
        assert_eq!(part_2(&program), 208);
    }

    #[test]
    fn masking_examples_p2() {
        let tests = [("000000000000000000000000000000X1001X", 42, [26, 27, 58, 59])];

        for (mask_src, x, mut expected) in tests.iter() {
            let mask = FloatingBitMask::new(mask_src);
            let mut values = mask.apply_all(*x).collect::<Vec<u64>>();
            values.sort();
            expected.sort();
            assert_eq!(values, expected);
        }
    }
}
