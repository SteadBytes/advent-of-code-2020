use std::collections::HashSet;

fn part_1(group_answers: &Vec<Vec<&str>>) -> usize {
    // TODO: Avoid creating so many HashSets?
    group_answers
        .iter()
        .map(|group| {
            group
                .iter()
                .flat_map(|answers| answers.chars())
                .collect::<HashSet<char>>()
                .len()
        })
        .sum()
}

fn part_2(group_answers: &Vec<Vec<&str>>) -> usize {
    // TODO: Avoid creating so many HashSets?
    group_answers
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|answers| answers.chars().collect::<HashSet<char>>())
        })
        .map(|mut group_sets| {
            if let Some(first_set) = group_sets.next() {
                // Length of the intersection of each set of answers in the group
                group_sets.fold(first_set, |set1, set2| &set1 & &set2).len()
            } else {
                // Empty group -> return 0 element for sum() operation
                // This should never occur with well-formed input as per the puzzle
                // description however if it does occur it is not a cause for error
                // and can be handled gracefully.
                0
            }
        })
        .sum()
}

fn parse_input(input: &str) -> Result<Vec<Vec<&str>>, ParseError> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|l| {
                    if l.chars().all(|c| c.is_ascii_lowercase()) {
                        Ok(l)
                    } else {
                        Err(ParseError::InvalidChar)
                    }
                })
                .collect()
        })
        .collect()
}

pub fn run(input: &str) {
    let group_answers = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(&group_answers));
    println!("Part 2: {}", part_2(&group_answers));
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum ParseError {
    InvalidChar,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
abc

a
b
c

ab
ac

a
a
a
a

b
";

    #[test]
    fn parse_input_example() {
        let group_answers = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(
            group_answers,
            vec![
                vec!["abc"],
                vec!["a", "b", "c"],
                vec!["ab", "ac"],
                vec!["a", "a", "a", "a"],
                vec!["b"]
            ]
        );
    }

    #[test]
    fn parse_input_invalid() {
        let input = "\
abc
def

123
a

b
";
        assert_eq!(parse_input(&input), Err(ParseError::InvalidChar));
    }

    #[test]
    fn part_1_example() {
        let group_answers = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(&group_answers), 11);
    }

    #[test]
    fn part_2_example() {
        let group_answers = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_2(&group_answers), 6);
    }
}
