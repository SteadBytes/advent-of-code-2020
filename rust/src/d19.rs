use crate::dbg_eprintln;
use crate::str::split_once;
use regex::Regex;
use Rule::*;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
enum Rule<'a> {
    Literal(&'a str),
    Composite(Vec<Vec<usize>>),
}

// Inedexed by rule Id
type Rules<'a> = Vec<Rule<'a>>;

/// Key properties:
/// - Two "base" types:
///     - Literal character matches `1: "b"`
///     - Composite rules match against other rules `2: 3 4`
/// - Alternation indicated by `|`
///     - Rule matches on *either* side of `|`
/// - Rule parts "consume" message text - subsequent rule parts match on the remaining text:
///     ```text
///     0: 1 2 2
///     1: "a"
///     2: "ba" | 1
///
///     message "abaa":
///         - "a" matches 1
///         - "ba" matches 2 (1st alternative)
///         - "a" matches 2 (2nd alternative)
///     ```
/// - No loops in rules (finite set of possible matches)
/// - Messages must *completely* match rule 0 to be valid
///     ```text
///     0: 1 2 2
///     1: "a"
///     2: "ba" | 1
///
///     message "abaab":
///         - "a" matches 1
///         - "ba" matches 2 (1st alternative)
///         - "a" matches 2 (2nd alternative)
///         - "b" remains unmatched (no more parts left in rule 0) -> not valid message
///     ```
///
/// Observation: Composite rules can be "compiled" to an alternation of string literals
///     ```text
///     0: 1 2 2
///     1: "a"
///     2: 3 1 | 1
///     3: "b"
///
///     0 = "ababa" | "abaa" | "aaa" | "aaba"
///     ```
///
/// Observation: An alternation of string literals can be represented by a Regex (non-capturing
/// groups as we only need to determine whether or not a message is a match):
///     ```text
///     0 = "ababa" | "abaa" | "aaa" | "aaba"
///     0 = ^a(?:ba|a)(?:ba|a)$
///     ```
fn part_1(rules: &Rules, messages: &[&str]) -> usize {
    let re_str = rules2regex(rules).unwrap(); // FIXME
    dbg_eprintln!("{}", re_str);
    let re = Regex::new(&re_str).expect("invalid regex compiled from rule"); // FIXME
    messages.iter().filter(|s| re.is_match(s)).count()
}

/// Compiles `rules[0]` into a single Regex string.
// TODO: Return Option or Result
// TODO: Avoid allocating String
// TODO: Avoid recursion - this was (IMO) the easiest/natural implementation and works absolutely
//       fine for the size of the puzzle input. However, it requires quite a lot of Vec allocation
//       & String joining as well as potentially causing issue for larger rules/rule sets.
fn rules2regex<'a>(rules: &[Rule<'a>]) -> Option<String> {
    fn inner<'a>(rules: &[Rule<'a>], r: &'a Rule) -> String {
        match r {
            // Base case: Literal rules cannot be further expanded
            Literal(s) => s.to_string(),
            // Recursive case: Composite rules can be expanded into alternations between
            // groups of Literal rules
            Composite(alts) => {
                // Alternatives expanded into groups of literals
                let groups = alts
                    .iter()
                    .map(|seq| seq.iter().map(|id| inner(rules, &rules[*id])));
                if alts.len() > 1 {
                    // Multiple alternatives -> multiple alternate non-capturing groups
                    format!(
                        "(?:{})", // Group for a single choice e.g. (?:abab)
                        groups
                            .map(|seq| seq.collect::<Vec<_>>().join(""))
                            .collect::<Vec<_>>()
                            .join("|")  // Match *one* of the choices
                    )
                } else {
                    // Single choice -> match string of literals
                    groups.flatten().collect::<Vec<_>>().join("")
                }
            }
        }
    }

    // Include anchors as a message must match *entirely*
    return Some(format!("^{}$", inner(rules, &rules[0])));
}

fn part_2() {}

fn parse_id(s: &str) -> Result<usize, ParseError> {
    s.parse::<usize>().map_err(|e| ParseError::InvalidRuleId(e))
}

fn parse_input(input: &str) -> Result<(Rules, Vec<&str>), ParseError> {
    let mut lines = input.lines();
    let mut rules = lines
        .by_ref()
        .take_while(|l| *l != "")
        .map(|l| {
            let (id, rest) = split_once(l, ": ").ok_or(ParseError::InvalidRule)?;
            let id = parse_id(id)?;

            let rule = if rest.starts_with('"') {
                Literal(
                    rest.strip_prefix("\"")
                        .and_then(|s| s.strip_suffix("\""))
                        .ok_or(ParseError::InvalidRule)?,
                )
            } else {
                let alt_rules = rest
                    .split(" | ")
                    .map(|rule_seq| {
                        rule_seq
                            .split_ascii_whitespace()
                            .map(|id| parse_id(id))
                            .collect::<Result<_, _>>()
                    })
                    .collect::<Result<_, _>>()?;
                Composite(alt_rules)
            };
            Ok((id, rule))
        })
        .collect::<Result<Vec<_>, _>>()?;
    rules.sort_by(|(id_a, _), (id_b, _)| id_a.cmp(id_b));
    let rules = rules.into_iter().map(|(_, r)| r).collect();

    Ok((rules, lines.collect()))
}

pub fn run(input: &str) {
    let (rules, messages) = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(&rules, &messages));
    // println!("Part 2: {}", part_2(&parsed));
}

#[derive(PartialEq, Eq, Debug)]
enum ParseError {
    InvalidRule,
    InvalidRuleId(std::num::ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const EXAMPLE_INPUT: &str = {
r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#
    };

    /// Puzzle input is *not* in order of rule Id - this is the same input as the example but with
    /// the rules re-ordered.
    #[rustfmt::skip]
    const EXAMPLE_INPUT_REORDERED: &str = {
r#"2: 4 4 | 5 5
1: 2 3 | 3 2
4: "a"
0: 4 1 5
5: "b"
3: 4 5 | 5 4

ababbb
bababa
abbbab
aaabbb
aaaabbb"#
    };

    #[test]
    fn parse_input_example() {
        for input in &[EXAMPLE_INPUT, EXAMPLE_INPUT_REORDERED] {
            let (rules, messages) = parse_input(input).unwrap();
            assert_eq!(
                rules,
                vec![
                    Composite(vec![vec![4, 1, 5]]),
                    Composite(vec![vec![2, 3], vec![3, 2]]),
                    Composite(vec![vec![4, 4], vec![5, 5]]),
                    Composite(vec![vec![4, 5], vec![5, 4]]),
                    Literal("a"),
                    Literal("b"),
                ]
            );

            assert_eq!(
                messages,
                ["ababbb", "bababa", "abbbab", "aaabbb", "aaaabbb"]
            );
        }
    }

    #[test]
    fn parse_input_example_reordered() {
        let (rules, messages) = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(
            rules,
            vec![
                Composite(vec![vec![4, 1, 5]]),
                Composite(vec![vec![2, 3], vec![3, 2]]),
                Composite(vec![vec![4, 4], vec![5, 5]]),
                Composite(vec![vec![4, 5], vec![5, 4]]),
                Literal("a"),
                Literal("b"),
            ]
        );

        assert_eq!(
            messages,
            ["ababbb", "bababa", "abbbab", "aaabbb", "aaaabbb"]
        );
    }

    #[test]
    fn rules2regex_simple() {
        let rules = [
            Composite(vec![vec![1, 2, 2]]),
            Literal("a"),
            Composite(vec![vec![3, 1], vec![1]]),
            Literal("b"),
        ];
        assert_eq!(rules2regex(&rules).unwrap(), "^a(?:ba|a)(?:ba|a)$");
    }

    #[test]
    fn part_1_example() {
        let (rules, messages) = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(&rules, &messages), 2);
    }

    #[test]
    fn part_2_example() {}
}
