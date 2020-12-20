fn part_1(start_nums: &Vec<usize>) -> usize {
    play(start_nums, 2020)
}

fn part_2(start_nums: &Vec<usize>) -> usize {
    play(start_nums, 30000000)
}

/// Play rounds of the memory game until `target_turn` is reached. At which point, return the
/// number spoken on that turn.
fn play(start_nums: &Vec<usize>, target_turn: usize) -> usize {
    // Skip the starting rounds (see turn_history)
    let mut prev = *start_nums.last().unwrap();
    let mut turn = start_nums.len();
    // Pre-allocate storage for the number spoken on each turn.
    // - Index -> number
    // - Value -> turn last spoken
    // Indexing by the spoken number is possible because, AFAICT, all the numbers fit well
    // within usize on both 32-bit and 64-bit architecture. This is significantly faster to access
    // than a HashMap.
    let mut th = vec![0; target_turn];
    for (i, x) in start_nums[..start_nums.len() - 1].iter().enumerate() {
        th[*x] = i + 1;
    }
    loop {
        let next = match th[prev] {
            // Previous spoken number spoken for the first time on the last turn
            0 => 0,
            // Previous spoken number had been spoken at least once before last turn
            x => turn - x,
        };

        // Speak number
        th[prev] = turn;
        prev = next;

        turn += 1;

        if turn == target_turn {
            return prev;
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<usize>, ParseError> {
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
    #[cfg_attr(not(feature = "slowtests"), ignore)]
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
