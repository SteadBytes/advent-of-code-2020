#![allow(dead_code)]
use std::collections::{HashMap, VecDeque};

const MY_BAG: &str = "shiny gold";

/// Graph of bag contents rules e.g.
///
/// ```text
/// {
///   "light red" -> [("bright white", 1), ("muted yellow", 2)],
///   "dark orange" -> [("bright white", 3), ("muted yellow", 4)],
///   ...,
///   "dotted black" -> [],
/// }
/// ```
type BagRules<'a> = HashMap<&'a str, Vec<(&'a str, usize)>>;

fn part_1(rules: &BagRules) -> usize {
    let possible_outer_bags = rules
        .keys()
        .filter(|&&k| k != MY_BAG && rules.get(k).unwrap().len() > 0);
    let mut found_outer_bags: Vec<&str> = Vec::new();
    for start_bag in possible_outer_bags {
        let mut queue: VecDeque<&str> = VecDeque::new();
        queue.push_front(start_bag);

        while !queue.is_empty() {
            let b = queue.pop_front().unwrap();
            if b == MY_BAG {
                found_outer_bags.push(start_bag);
                break;
            }
            for (inner_bag, _) in rules.get(b).unwrap() {
                queue.push_back(inner_bag);
            }
        }
    }

    found_outer_bags.len()
}

fn part_2(rules: &BagRules) -> usize {
    // Both implementations left in for sake of comparison.

    // count_required_bags_recursive(rules, MY_BAG) - 1 // Don't count my bag
    count_required_bags_iterative(rules, MY_BAG)
}

fn count_required_bags_recursive<'a>(rules: &'a BagRules, bag: &'a str) -> usize {
    1 + rules
        .get(bag)
        .unwrap()
        .iter()
        .map(|(child, n)| n * count_required_bags_recursive(rules, child))
        .sum::<usize>()
}

fn count_required_bags_iterative<'a>(rules: &'a BagRules, bag: &'a str) -> usize {
    let mut total = 0;
    let mut s = vec![(bag, 1)];

    while !s.is_empty() {
        let (bag, n) = s.pop().unwrap();
        total += n;
        for (child, x) in rules.get(bag).unwrap() {
            s.push((child, n * x))
        }
    }

    total - 1 // Don't count start bag
}

pub fn run(input: &str) {
    let rules = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(&rules));
    println!("Part 2: {}", part_2(&rules));
}

/// Rough syntax for puzzle input:
/// ```text
/// rules = rule [{ "\n" rule }]
/// rule = bag " bags contain " ["no other bags." | rule-items "."]
/// rule-items = { rule-item ", " }
/// rule-item = { digit } " " bag "bag" [ "s" ]
/// bag = { letter } " " { letter }
/// ```
fn parse_input(input: &str) -> Result<BagRules, ParseError> {
    // FIXME: Actually return error cases
    Ok(input
        .lines()
        .map(|line| {
            let (l, r) = split_once(line, " contain ").unwrap();
            let (_, parent) = rsplit_once(l, " ").unwrap();
            if r == "no other bags." {
                (parent, vec![])
            } else {
                let children = r.trim_end_matches(".").split(", ").map(|child| {
                    let (quantity, rest) = split_once(child, " ").unwrap();
                    let (_, bag) = rsplit_once(rest, " ").unwrap();
                    (bag, quantity.parse::<usize>().unwrap())
                });
                (parent, children.collect())
            }
        })
        .collect())
}

pub fn split_once<'a>(s: &'a str, pat: &'a str) -> Option<(&'a str, &'a str)> {
    let mut split = s.splitn(2, pat);
    match (split.next(), split.next()) {
        (Some(s1), Some(s2)) => Some((s1, s2)),
        _ => None,
    }
}

fn rsplit_once<'a>(s: &'a str, pat: &'a str) -> Option<(&'a str, &'a str)> {
    let mut split = s.rsplitn(2, pat);
    match (split.next(), split.next()) {
        (Some(s1), Some(s2)) => Some((s1, s2)),
        _ => None,
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn parse_input_example() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT).unwrap(),
            vec![
                ("light red", vec![("bright white", 1), ("muted yellow", 2)]),
                (
                    "dark orange",
                    vec![("bright white", 3), ("muted yellow", 4)],
                ),
                ("bright white", vec![("shiny gold", 1)]),
                ("muted yellow", vec![("shiny gold", 2), ("faded blue", 9)]),
                ("shiny gold", vec![("dark olive", 1), ("vibrant plum", 2)]),
                ("dark olive", vec![("faded blue", 3), ("dotted black", 4)]),
                ("vibrant plum", vec![("faded blue", 5), ("dotted black", 6)]),
                ("faded blue", vec![]),
                ("dotted black", vec![]),
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn part_1_example() {
        let rules = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(&rules), 4);
    }

    #[test]
    fn part_2_example() {
        let input = "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        let rules = parse_input(input).unwrap();
        assert_eq!(part_2(&rules), 126);
    }
}
