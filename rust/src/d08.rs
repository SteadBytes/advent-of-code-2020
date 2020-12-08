#![allow(dead_code)]
use crate::d07::split_once; // FIXME: Move this to shared library
use std::collections::HashSet;
use Opcode::*;

fn part_1(prg: &Vec<Instruction>) -> Result<i32, Error> {
    match execute_program(prg) {
        Err(x) => Ok(x),
        _ => Err(Error::ExpectedInfiniteLoop),
    }
}

fn part_2(prg: &Vec<Instruction>) -> Result<i32, Error> {
    // Brute force search for the opcode to fix
    // TODO: I feel like there might be a trick I'm missing in the puzzle that means
    // a brute force search isn't required...
    for i in 0..prg.len() {
        let changed = match prg[i] {
            (Nop, x) => (Jmp, x),
            (Jmp, x) => (Nop, x),
            _ => continue,
        };

        // TODO: Avoid copying + allocating!
        let new_prg = [&prg[..i], &[changed], &prg[i + 1..]].concat();
        if let Ok(x) = execute_program(&new_prg) {
            return Ok(x);
        };
    }

    Err(Error::ProgramFixNotFound)
}

/// Execute `prg` until termination *or* an inifinite loop is encountered.
///
/// If the program terminates then `Result::Ok` is returned, containing the final accumulator
/// value. If an inifinite loop is encountered then `Result::Err` is returned, containing the
/// accumulator value immediately before the loop is entered.
fn execute_program(prg: &Vec<Instruction>) -> Result<i32, i32> {
    let mut ip = 0;
    let mut acc = 0;
    let mut executed = HashSet::new();

    while ip < prg.len() {
        let instruction = &prg[ip];
        if executed.contains(&ip) {
            // Infinite loop!
            return Err(acc);
        }
        executed.insert(ip);
        match instruction {
            (Acc, x) => acc += x,
            (Jmp, x) if *x < 0 => {
                ip = ip.checked_sub(x.abs() as usize).unwrap();
                continue;
            }
            (Jmp, x) => {
                ip += *x as usize;
                continue;
            }
            (Nop, _) => (),
        }
        ip += 1;
    }

    Ok(acc)
}

fn parse_input(input: &str) -> Result<Vec<Instruction>, ParseError> {
    // TODO: Clean this up a bit
    input
        .lines()
        .map(|l| {
            let (op, arg) = split_once(l, " ").ok_or(ParseError::InvalidInstruction)?;
            let x = arg
                .parse::<i32>()
                .map_err(|e| ParseError::InvalidInteger(e))?;
            match op {
                "acc" => Ok((Acc, x)),
                "jmp" => Ok((Jmp, x)),
                "nop" => Ok((Nop, x)),
                _ => Err(ParseError::InvalidOpcode),
            }
        })
        .collect()
}

pub fn run(input: &str) {
    let program = parse_input(input).expect("unable to parse input");
    println!(
        "Part 1: {}",
        part_1(&program).expect("no infinite loop found")
    );
    println!(
        "Part 2: {}",
        part_2(&program).expect("unable to find a fix for input program")
    );
}

type Instruction = (Opcode, i32);

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Clone)]
enum Opcode {
    Acc,
    Jmp,
    Nop,
}

#[derive(PartialEq, Eq, Debug)]
enum ParseError {
    InvalidOpcode,
    InvalidInstruction,
    InvalidInteger(std::num::ParseIntError),
}

#[derive(PartialEq, Eq, Debug)]
enum Error {
    ExpectedInfiniteLoop,
    ProgramFixNotFound,
    Parse(ParseError),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn parse_input_example() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT).unwrap(),
            vec![
                (Nop, 0),
                (Acc, 1),
                (Jmp, 4),
                (Acc, 3),
                (Jmp, -3),
                (Acc, -99),
                (Acc, 1),
                (Jmp, -4),
                (Acc, 6)
            ]
        );
    }

    #[test]
    fn part_1_example() {
        let program = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(&program).unwrap(), 5);
    }

    #[test]
    fn part_2_example() {
        let program = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_2(&program).unwrap(), 8);
    }
}
