use crate::dbg_eprintln;
use OpType::*;
use Token::*;

// TODO: Plenty of room for improvement - better error handling, reducing allocations during
// eval/parsing e.t.c but this works correctly and (on my laptop at least) executes my puzzle input
// ,to the human eye, instantaneously.

// TODO: return Result?
fn part_1(exprs: &[Vec<Token>]) -> u64 {
    let getop: OpLookup = |op: &OpType| match op {
        Add => (|x, y| x + y, 0),
        Mul => (|x, y| x * y, 0),
    };

    exprs
        .iter()
        .try_fold(0, |acc, e| evaluate(&e, getop).map(|x| acc + x))
        .unwrap()
}

// TODO: return Result?
fn part_2(exprs: &[Vec<Token>]) -> u64 {
    let getop: OpLookup = |op: &OpType| match op {
        Add => (|x, y| x + y, 1),
        Mul => (|x, y| x * y, 0),
    };

    exprs
        .iter()
        .try_fold(0, |acc, e| evaluate(&e, getop).map(|x| acc + x))
        .unwrap()
}

/// Evaluate an infix expression `expr` to a single value.
///
/// `expr` is first transformed into postfix notation via a simplified (no operator
/// associativity) shunting-yard algorithm before the resulting expression is evaluated.
fn evaluate(expr: &[Token], getop: OpLookup) -> Result<u64, ParseError> {
    dbg_eprintln!("expr (infix): {:?}", expr);
    // Shunting-yard algorithm
    let mut outq: Vec<Token> = vec![]; // Queue of values/operators in postfix notation
    let mut opstack: Vec<Token> = vec![]; // Stack of operators being processed
    for t in expr {
        dbg_eprintln!("t: {:?}", t);
        match t {
            Int(_) => outq.push(*t),
            LParen => opstack.push(*t),
            Operator(optype) => {
                let (_, op_precedence) = getop(optype);
                while let Some(top) = opstack.last() {
                    dbg_eprintln!("top: {:?}", top);
                    match top {
                        LParen => break,
                        // Note: Normal shunting-yard alogrithm would take associativity into
                        // account here. Since both possible operators have the same (left)
                        // associativity this has been removed.
                        Operator(top_optype) => {
                            let (_, top_op_precedence) = getop(top_optype);
                            if top_op_precedence >= op_precedence {
                                outq.push(opstack.pop().unwrap());
                            } else {
                                break;
                            }
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
        dbg_eprintln!("t: {:?}", t);
        match t {
            Int(x) => stack.push(x),
            // TODO: Assign a function to operators directly to avoid this duplication?
            Operator(op) => {
                if let (Some(y), Some(x)) = (stack.pop(), stack.pop()) {
                    let (f, _) = getop(&op);
                    stack.push(f(x, y));
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
                        r.push(Operator(Add));
                        chars.next();
                    }
                    '*' => {
                        r.push(Operator(Mul));
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
    println!("Part 2: {}", part_2(&tokens));
}

/// Returns a tuple of `(apply_fn, precedence)` for a given `OpType`.
type OpLookup = fn(&OpType) -> (fn(u64, u64) -> u64, u8);

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
enum OpType {
    Add,
    Mul,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
enum Token {
    Int(u64),
    Operator(OpType),
    LParen,
    RParen,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum ParseError {
    InvalidCharacter(char),
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
    const PART_2_EXAMPLE_ANSWERS: [u64; 6] = [231, 51, 46, 1445, 669060, 23340];

    #[test]
    fn parse_input_examples() {
        // Examples are given as single lines, however the puzzle input is multline - join into
        // a single input for a simpler test.
        let single_input = EXAMPLE_INPUTS.join("\n");
        let expected: [Vec<Token>; EXAMPLE_INPUTS.len()] = [
            vec![
                Int(1),
                Operator(Add),
                Int(2),
                Operator(Mul),
                Int(3),
                Operator(Add),
                Int(4),
                Operator(Mul),
                Int(5),
                Operator(Add),
                Int(6),
            ],
            vec![
                Int(1),
                Operator(Add),
                LParen,
                Int(2),
                Operator(Mul),
                Int(3),
                RParen,
                Operator(Add),
                LParen,
                Int(4),
                Operator(Mul),
                LParen,
                Int(5),
                Operator(Add),
                Int(6),
                RParen,
                RParen,
            ],
            vec![
                Int(2),
                Operator(Mul),
                Int(3),
                Operator(Add),
                LParen,
                Int(4),
                Operator(Mul),
                Int(5),
                RParen,
            ],
            vec![
                Int(5),
                Operator(Add),
                LParen,
                Int(8),
                Operator(Mul),
                Int(3),
                Operator(Add),
                Int(9),
                Operator(Add),
                Int(3),
                Operator(Mul),
                Int(4),
                Operator(Mul),
                Int(3),
                RParen,
            ],
            vec![
                Int(5),
                Operator(Mul),
                Int(9),
                Operator(Mul),
                LParen,
                Int(7),
                Operator(Mul),
                Int(3),
                Operator(Mul),
                Int(3),
                Operator(Add),
                Int(9),
                Operator(Mul),
                Int(3),
                Operator(Add),
                LParen,
                Int(8),
                Operator(Add),
                Int(6),
                Operator(Mul),
                Int(4),
                RParen,
                RParen,
            ],
            vec![
                LParen,
                LParen,
                Int(2),
                Operator(Add),
                Int(4),
                Operator(Mul),
                Int(9),
                RParen,
                Operator(Mul),
                LParen,
                Int(6),
                Operator(Add),
                Int(9),
                Operator(Mul),
                Int(8),
                Operator(Add),
                Int(6),
                RParen,
                Operator(Add),
                Int(6),
                RParen,
                Operator(Add),
                Int(2),
                Operator(Add),
                Int(4),
                Operator(Mul),
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
    fn part_2_examples() {
        for (input, expected) in EXAMPLE_INPUTS.iter().zip(&PART_2_EXAMPLE_ANSWERS) {
            let exprs = parse_input(input).unwrap();
            assert_eq!(part_2(&exprs), *expected);
        }
    }

    #[test]
    fn part_2_examples_joined() {
        // Examples are given as single lines, however the puzzle input is multline - join into
        // a single input for a simpler test.
        let single_input = EXAMPLE_INPUTS.join("\n");
        let exprs = parse_input(&single_input).unwrap();
        // Sum of individual expression results given in puzzle
        assert_eq!(part_2(&exprs), 694173);
    }
}
