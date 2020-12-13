use std::convert::TryFrom;
use std::ops;
use Instruction::*;

// TODO: There is probably some  abstraction to be had out of `part_1` and `part_2`. They only
// differ in which entity is updated by each instruction (ship direction, ship location or waypoint
// location). I'm  not forcing it for the sake of reducing code size, however, as this will
// convolute the logic for each part.

fn part_1(instructions: &Vec<Instruction>) -> u32 {
    let mut pos = (0, 0);
    // Start facing east
    let mut dir = (1, 0);

    for inst in instructions {
        match inst {
            Forward(n) => {
                pos.0 += dir.0 * n;
                pos.1 += dir.1 * n;
            }
            North(n) => pos.1 += n,
            South(n) => pos.1 -= n,
            East(n) => pos.0 += n,
            West(n) => pos.0 -= n,
            Left(n) => dir = (-*n).apply_to(&dir),
            Right(n) => dir = n.apply_to(&dir),
        }
    }

    u32::try_from(pos.0.abs() + pos.1.abs()).unwrap() // FIXME
}

fn part_2(instructions: &Vec<Instruction>) -> u32 {
    let mut pos = (0, 0);
    let mut wp = (10, 1);

    for inst in instructions {
        match inst {
            Forward(n) => {
                pos.0 += wp.0 * n;
                pos.1 += wp.1 * n;
            }
            North(n) => wp.1 += n,
            South(n) => wp.1 -= n,
            East(n) => wp.0 += n,
            West(n) => wp.0 -= n,
            Left(n) => wp = (-*n).apply_to(&wp),
            Right(n) => wp = n.apply_to(&wp),
        }
    }

    u32::try_from(pos.0.abs() + pos.1.abs()).unwrap() // FIXME
}

fn parse_input(input: &str) -> Result<Vec<Instruction>, ParseError> {
    input
        .lines()
        .map(|l| {
            if l.len() < 2 {
                Err(ParseError::InvalidInstruction(l))
            } else {
                let (cmdpart, valpart) = l.split_at(1);
                let n = valpart
                    .parse::<i32>()
                    .map_err(|e| ParseError::InvalidValue(e))?;
                match cmdpart {
                    "F" => Ok(Forward(n)),
                    "N" => Ok(North(n)),
                    "S" => Ok(South(n)),
                    "E" => Ok(East(n)),
                    "W" => Ok(West(n)),
                    "L" => Ok(Left(TurnAngle::new(n).ok_or(ParseError::InvalidAngle(n))?)),
                    "R" => Ok(Right(TurnAngle::new(n).ok_or(ParseError::InvalidAngle(n))?)),
                    _ => Err(ParseError::InvalidCommand(cmdpart)),
                }
            }
        })
        .collect()
}

pub fn run(input: &str) {
    let instructions = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(&instructions));
    println!("Part 2: {}", part_2(&instructions));
}

/// Newtype representing a valid angle for a `Left` or `Right` instruction. Values should only be
/// multiples of `+-90` as the puzzle is represented using unit grid.
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
struct TurnAngle(i32);

impl TurnAngle {
    fn new(angle: i32) -> Option<TurnAngle> {
        match angle {
            0 | 90 | -90 | -180 | 180 | -280 | 270 => Some(TurnAngle(angle)),
            _ => None,
        }
    }

    /// Returns `p` rotated *clockwise* by `self`.
    fn apply_to(&self, p: &(i32, i32)) -> (i32, i32) {
        match self.0 {
            0 => *p,
            90 | -270 => (p.1, -p.0),
            180 | -180 => (-p.0, -p.1),
            270 | -90 => (-p.1, p.0),
            _ => unreachable!(),
        }
    }
}

impl ops::Neg for TurnAngle {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(TurnAngle),
    Right(TurnAngle),
    Forward(i32),
}

#[derive(PartialEq, Eq, Debug)]
enum ParseError<'a> {
    InvalidInstruction(&'a str),
    InvalidCommand(&'a str),
    InvalidValue(std::num::ParseIntError),
    InvalidAngle(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
F10
N3
F7
R90
F11";

    #[test]
    fn parse_input_example() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT).unwrap(),
            [
                Forward(10),
                North(3),
                Forward(7),
                Right(TurnAngle::new(90).unwrap()),
                Forward(11)
            ]
        );
    }

    #[test]
    fn part_1_example() {
        let instructions = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(&instructions), 25);
    }

    #[test]
    fn part_2_example() {
        let instructions = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_2(&instructions), 286);
    }
}
