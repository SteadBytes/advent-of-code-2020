use crate::dbg_eprintln;
use Token::*;

// TODO: return Result?
fn part_1(exprs: &[Vec<Token>]) -> u64 {
    exprs
        .iter()
        .try_fold(0, |acc, e| evaluate(&e).map(|x| acc + x))
        .unwrap()
}

fn part_2() {}

/// Evaluate an infix expression `expr` to a single value.
///
/// `expr` is first transformed into postfix notation via a simplified (no operator
/// precedence) shunting-yard algorithm before the resulting expression is evaluated.
fn evaluate(expr: &[Token]) -> Result<u64, ParseError> {
    dbg_eprintln!("expr (infix): {:?}", expr);
    // Shunting-yard algorithm
    let mut outq: Vec<Token> = vec![]; // Queue of values/operators in postfix notation
    let mut opstack: Vec<Token> = vec![]; // Stack of operators being processed
    for t in expr {
        dbg_eprintln!("t: {:?}", t);
        match t {
            Int(_) => outq.push(*t),
            LParen => opstack.push(*t),
            Plus | Star => {
                while let Some(top) = opstack.last() {
                    dbg_eprintln!("top: {:?}", top);
                    match top {
                        LParen => break,
                        // Normal shunting-yard alogrithm would take operator precedence into
                        // account here. Since there is none (other than parenthesis) this is
                        // simplified to just push the next operator onto the output.
                        Star | Plus => {
                            let op = opstack.pop().unwrap();
                            outq.push(op);
                        }
                        _ => return Err(ParseError::InvalidExpression),
                    }
                }
                opstack.push(*t);
            }
            RParen => {
                while let Some(op) = opstack.pop().filter(|op| *op != LParen) {
                    outq.push(op);
                }
            }
        }
    }

    while let Some(op) = opstack.pop().filter(|op| *op != LParen) {
        outq.push(op)
    }

    // Unprocessed tokens -> malformed input
    if !opstack.is_empty() {
        return Err(ParseError::InvalidExpression);
    }

    dbg_eprintln!("outq (postfix): {:?}", outq);

    // Evaluate postfix expression in outq
    let mut stack = vec![];
    for t in outq {
        match t {
            Int(x) => stack.push(x),
            // TODO: Assign a function to operators directly to avoid this duplication?
            Plus => {
                if let (Some(y), Some(x)) = (stack.pop(), stack.pop()) {
                    stack.push(x + y);
                } else {
                    // Missing operand
                    return Err(ParseError::InvalidExpression);
                }
            }
            Star => {
                if let (Some(y), Some(x)) = (stack.pop(), stack.pop()) {
                    stack.push(x * y);
                } else {
                    // Missing operand
                    return Err(ParseError::InvalidExpression);
                }
            }
            _ => unreachable!(),
        }
    }

    assert_eq!(stack.len(), 1);
    Ok(stack.pop().unwrap())
}

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
    let tokens = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(&tokens));
    // println!("Part 2: {}", part_2(&tokens));
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
    IncompleteExpression,
    InvalidExpression,
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
            let exprs = parse_input(input).unwrap();
            assert_eq!(part_1(&exprs), *expected);
        }
    }

    #[test]
    fn part_1_examples_joined() {
        // Examples are given as single lines, however the puzzle input is multline - join into
        // a single input for a simpler test.
        let single_input = EXAMPLE_INPUTS.join("\n");
        let exprs = parse_input(&single_input).unwrap();
        // Sum of individual expression results given in puzzle
        assert_eq!(part_1(&exprs), 26457);
    }

    #[test]
    fn part_2_example() {}
}
