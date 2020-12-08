#![allow(dead_code)]
use crate::d07::split_once; // FIXME: Move this to shared library
use std::collections::HashSet;
use Opcode::*;

fn part_1(prg: &Vec<Instruction>) -> i32 {
    // TODO: Clean this up
    let mut pc = 0;
    let mut acc = 0;
    let mut executed = HashSet::new();

    loop {
        let instruction = &prg[pc];
        if executed.contains(&pc) {
            return acc;
        }
        executed.insert(pc);
        match instruction {
            (Acc, x) => acc += x,
            (Jmp, x) if *x < 0 => {
                pc = pc.checked_sub(x.abs() as usize).unwrap();
                continue;
            }
            (Jmp, x) => {
                pc += *x as usize;
                continue;
            }
            (Nop, _) => (),
        }
        pc += 1;
    }
}

fn part_2() {}

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
    println!("Part 1: {}", part_1(&program));
    // println!("Part 2: {}", part_2(&parsed));
}

type Instruction = (Opcode, i32);

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
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
        assert_eq!(part_1(&program), 5);
    }

    #[test]
    fn part_2_example() {}
}
