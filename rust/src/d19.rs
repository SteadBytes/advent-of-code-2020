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
    let re_str = rules2regex(&rules);
    dbg_eprintln!("{}", re_str);
    let re = Regex::new(&re_str).expect("invalid regex compiled from rule"); // FIXME
    messages.iter().filter(|s| re.is_match(s)).count()
}

/// Observation: Rules 8 and 11 *only* appear in rule `0: 8 11` (the target rule).
/// - Matching messages must therefore start/end with matches for rules `8`/`11`
///
/// New rule `8: 42 | 42 8` is equivalent to one or more matches of rule `42` (`42+`)
///
/// New rule `11: 42 31 | 42 11 31` is equivalent to `42 (?:11|?R))* 31`
/// - Recursive in between leading and trailing `42`/`31`
///
/// Rule `0 is now equivalent to `^42+ (?:42 (?:11|?R) 31)$`
///
/// Observation:
/// - Message must start with at least one match for rule `42`
/// - Message must end with at least one match for rule `31`
///
/// Observation: In a matching message, the number of matches for rule `31` in the *ending* section
/// (e.g. `31+$`) plus `1` is the *minimum* number of matches for rule `42` in the *preceding* section.
/// - Given the message starts/ends with matches for rules `42`/`31`
/// - Rule `11` starts with a single match for rule `42`, followed by `n` recursive matches
///   with 1 match for rule `42` and rule `31` each.
///
/// Match this using several Regexes:
/// - Check start & end patterns: `^(?P<start>(?:42)+)(?P<end>(?:31)+)$`
/// - Count number of rule `31` matches in `end` section: `(31)`
/// - Count number of rule `42` matches in `start` section: `(42)`
/// - Compare counts to determine validity
///
/// Note: As stated by the puzzle, this *only* applies to the rules/messages in the puzzle input
/// and does not hold in the general case.
fn part_2(rules: &Rules, messages: &[&str]) -> usize {
    // FIXME: Avoid this cloning?
    let mut rules = rules.clone();
    rules[8] = Composite(vec![vec![42], vec![42, 8]]);
    rules[11] = Composite(vec![vec![42, 31], vec![42, 11, 31]]);

    let r31_pattern = rule2regex(&rules, &rules[31]);
    let r42_pattern = rule2regex(&rules, &rules[42]);

    let start_end_re = Regex::new(&format!(
        "^(?P<start>(?:{})+)(?P<end>(?:{})+)$",
        r42_pattern, r31_pattern
    ))
    .expect("invalid start/end pattern");

    let r31_cap =
        Regex::new(&format!("({})", r31_pattern)).expect("invalid rule 31 capture pattern");
    let r42_cap =
        Regex::new(&format!("({})", r42_pattern)).expect("invalid rule 42 capture pattern");

    messages
        .iter()
        .filter(|m| {
            let mut start_end_caps = start_end_re.captures_iter(m);
            match (start_end_caps.next(), start_end_caps.next()) {
                (Some(c), None) => {
                    r42_cap.find_iter(&c["start"]).count() > r31_cap.find_iter(&c["end"]).count()
                }
                _ => false,
            }
        })
        .count()
}

pub fn run(input: &str) {
    let (rules, messages) = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(&rules, &messages));
    println!("Part 2: {}", part_2(&rules, &messages));
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

fn parse_id(s: &str) -> Result<usize, ParseError> {
    s.parse::<usize>().map_err(|e| ParseError::InvalidRuleId(e))
}

/// Compiles `rules[0]` into a single non-capturing Regex string to match an *entire* message.
fn rules2regex<'a>(rules: &[Rule<'a>]) -> String {
    // Include anchors as a message must match *entirely*
    return format!("^{}$", rule2regex(rules, &rules[0]));
}

/// Compiles `r` into a single non-capturing Regex string to match *part* of a message.
// TODO: Return Option or Result
// TODO: Avoid allocating String
// TODO: Avoid recursion - this was (IMO) the easiest/natural implementation and works absolutely
//       fine for the size of the puzzle input. However, it requires quite a lot of Vec allocation
//       & String joining as well as potentially causing issue for larger rules/rule sets.
fn rule2regex<'a>(rules: &[Rule<'a>], r: &'a Rule) -> String {
    match r {
        // Base case: Literal rules cannot be further expanded
        Literal(s) => s.to_string(),
        // Recursive case: Composite rules can be expanded into alternations between
        // groups of Literal rules
        Composite(alts) => {
            // Alternatives expanded into groups of literals
            let groups = alts
                .iter()
                .map(|seq| seq.iter().map(|id| rule2regex(rules, &rules[*id])));
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

    /// The example is missing some rules to make a contiguous set from `0..=41`. `parse_input`
    /// assumes a full set as it sorts by `id` then collects into a `Vec`. Without a full set,
    /// the solution functions will cause index out of bounds errors when looking up rule IDs.
    /// Forunately, the example rules don't *actually* use any of the missing ones so I have filled
    /// them in below with `Literal("x")` rules. These should never actually be matched against and
    /// if they are it will be clear as no messages will ever match them.
    /// This was quicker and easier than changing the implementation to accomodate for this (e.g.
    /// using a HashMap keyed by rule id).
    #[rustfmt::skip]
    const EXAMPLE_INPUT_2: &str = {
r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1
32: "z"
33: "z"
34: "z"
35: "z"
36: "z"
37: "z"
38: "z"
39: "z"
40: "z"
41: "z"
29: "z"
30: "z"

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#
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
        assert_eq!(rules2regex(&rules), "^a(?:ba|a)(?:ba|a)$");
    }

    #[test]
    fn part_1_example() {
        let (rules, messages) = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(&rules, &messages), 2);
    }

    #[test]
    fn part_2_example() {
        // FIXME: EXAMPLE_INTPUT_2 is missing some rules
        let (rules, messages) = parse_input(EXAMPLE_INPUT_2).unwrap();
        assert_eq!(part_2(&rules, &messages), 12);
    }
}
