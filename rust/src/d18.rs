use Token::*;

fn part_1(exprs: &[Vec<Token>]) -> u64 {
    todo!()
}

fn part_2() {}

fn parse_input(input: &str) -> Result<Vec<Vec<Token>>, ParseError> {
    input
        .lines()
        .map(|l| {
            let mut r = vec![];
            let mut chars = l.chars().peekable();
            while let Some(&c) = chars.peek() {
                match c {
                    // Basic symbols
                    '+' => {
                        r.push(Plus);
                        chars.next();
                    }
                    '*' => {
                        r.push(Star);
                        chars.next();
                    }
                    '(' => {
                        r.push(LParen);
                        chars.next();
                    }
                    ')' => {
                        r.push(RParen);
                        chars.next();
                    }
                    // Whitespace is insignificant -> skip
                    ' ' => {
                        chars.next();
                    }
                    // Integer (string of consecutive base 10 digits)
                    _ if c.is_digit(10) => {
                        chars.next();
                        let mut x = c.to_digit(10).unwrap() as u64;
                        while let Some(Some(digit)) = chars.peek().map(|c| c.to_digit(10)) {
                            x = x * 10 + digit as u64;
                            chars.next();
                        }
                        r.push(Int(x))
                    }
                    _ => return Err(ParseError::InvalidCharacter(c)),
                }
            }
            Ok(r)
        })
        .collect()
}

pub fn run(input: &str) {
    let parsed = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(&parsed));
    // println!("Part 2: {}", part_2(&parsed));
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
enum Token {
    Int(u64),
    Plus,
    Star,
    LParen,
    RParen,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum ParseError {
    InvalidCharacter(char),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUTS: [&str; 6] = [
        "1 + 2 * 3 + 4 * 5 + 6",
        "1 + (2 * 3) + (4 * (5 + 6))",
        "2 * 3 + (4 * 5)",
        "5 + (8 * 3 + 9 + 3 * 4 * 3)",
        "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
        "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
    ];
    const PART_1_EXAMPLE_ANSWERS: [u64; 6] = [71, 51, 26, 437, 12240, 13632];

    #[test]
    fn parse_input_examples() {
        // Examples are given as single lines, however the puzzle input is multline - join into
        // a single input for a simpler test.
        let single_input = EXAMPLE_INPUTS.join("\n");
        let expected: [Vec<Token>; EXAMPLE_INPUTS.len()] = [
            vec![
                Int(1),
                Plus,
                Int(2),
                Star,
                Int(3),
                Plus,
                Int(4),
                Star,
                Int(5),
                Plus,
                Int(6),
            ],
            vec![
                Int(1),
                Plus,
                LParen,
                Int(2),
                Star,
                Int(3),
                RParen,
                Plus,
                LParen,
                Int(4),
                Star,
                LParen,
                Int(5),
                Plus,
                Int(6),
                RParen,
                RParen,
            ],
            vec![
                Int(2),
                Star,
                Int(3),
                Plus,
                LParen,
                Int(4),
                Star,
                Int(5),
                RParen,
            ],
            vec![
                Int(5),
                Plus,
                LParen,
                Int(8),
                Star,
                Int(3),
                Plus,
                Int(9),
                Plus,
                Int(3),
                Star,
                Int(4),
                Star,
                Int(3),
                RParen,
            ],
            vec![
                Int(5),
                Star,
                Int(9),
                Star,
                LParen,
                Int(7),
                Star,
                Int(3),
                Star,
                Int(3),
                Plus,
                Int(9),
                Star,
                Int(3),
                Plus,
                LParen,
                Int(8),
                Plus,
                Int(6),
                Star,
                Int(4),
                RParen,
                RParen,
            ],
            vec![
                LParen,
                LParen,
                Int(2),
                Plus,
                Int(4),
                Star,
                Int(9),
                RParen,
                Star,
                LParen,
                Int(6),
                Plus,
                Int(9),
                Star,
                Int(8),
                Plus,
                Int(6),
                RParen,
                Plus,
                Int(6),
                RParen,
                Plus,
                Int(2),
                Plus,
                Int(4),
                Star,
                Int(2),
            ],
        ];

        assert_eq!(parse_input(&single_input).unwrap(), expected);
    }

    #[test]
    fn part_1_examples() {
        for (input, expected) in EXAMPLE_INPUTS.iter().zip(&PART_1_EXAMPLE_ANSWERS) {
            let parsed = parse_input(input).unwrap();
            assert_eq!(part_1(&parsed), *expected);
        }
    }

    #[test]
    fn part_2_example() {}
}
