use std::collections::HashMap;

fn part_1(start_nums: &Vec<u32>) -> u32 {
    play(start_nums, 2020)
}

fn part_2(start_nums: &Vec<u32>) -> u32 {
    play(start_nums, 30000000)
}

/// Play rounds of the memory game until `target_turn` is reached. At which point, return the
/// number spoken on that turn.
fn play(start_nums: &Vec<u32>, target_turn: u32) -> u32 {
    // Skip the starting rounds (see turn_history)
    let mut prev = *start_nums.last().unwrap();
    let mut turn = start_nums.len() as u32;
    // {num: turn_last_spoken}
    // Initialised with the first n - 1 turns e.g. {0: 1, 3: 2} for input "0,3,6"
    let mut turn_history = start_nums[..start_nums.len() - 1]
        .iter()
        .enumerate()
        .map(|(i, x)| (*x, (i + 1) as u32))
        .collect::<HashMap<u32, u32>>();
    loop {
        let next = if let Some(x) = turn_history.get(&prev) {
            // Previous spoken number had been spoken at least once before last turn
            turn - x
        } else {
            // Previous spoken number spoken for the first time on the last turn
            0
        };

        // Speak number
        turn_history.insert(prev, turn);
        prev = next;

        turn += 1;

        if turn == target_turn {
            return prev;
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<u32>, ParseError> {
    input
        .trim()
        .split(",")
        .map(|s| s.parse().map_err(|e| ParseError::InvalidNumber(e)))
        .collect()
}

pub fn run(input: &str) {
    let start_nums = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(&start_nums));
    println!("Part 2: {}", part_2(&start_nums));
}

#[derive(PartialEq, Eq, Debug)]
enum ParseError {
    InvalidNumber(std::num::ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "0,3,6";

    #[test]
    fn parse_input_example() {
        assert_eq!(parse_input(EXAMPLE_INPUT).unwrap(), [0, 3, 6]);
    }

    #[test]
    fn part_1_examples() {
        let tests = [
            ("0,3,6", 436),
            ("1,3,2", 1),
            ("2,1,3", 10),
            ("1,2,3", 27),
            ("2,3,1", 78),
            ("3,2,1", 438),
            ("3,1,2", 1836),
        ];
        for (input, expected) in &tests {
            let start_nums = parse_input(input).unwrap();
            assert_eq!(part_1(&start_nums), *expected);
        }
    }

    #[test]
    fn part_2_examples() {
        let tests = [
            ("0,3,6", 175594),
            ("1,3,2", 2578),
            ("2,1,3", 3544142),
            ("1,2,3", 261214),
            ("2,3,1", 6895259),
            ("3,2,1", 18),
            ("3,1,2", 362),
        ];
        for (input, expected) in &tests {
            let start_nums = parse_input(input).unwrap();
            assert_eq!(part_2(&start_nums), *expected);
        }
    }
}
