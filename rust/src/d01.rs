use itertools::Itertools;

const TARGET: i32 = 2020;

pub fn run(input: &str) {
    let entries = parse_input(input);
    println!("Part 1: {}", part_1(&entries));
    println!("Part 2: {}", part_2(&entries));
}

fn part_1(entries: &[i32]) -> i32 {
    entries
        .iter()
        .combinations(2)
        .find(|c| c.iter().copied().sum::<i32>() == TARGET)
        .expect("no pairs that sum to 2020")
        .iter()
        .copied()
        .product()
}

fn part_2(entries: &[i32]) -> i32 {
    entries
        .iter()
        .combinations(3)
        .find(|c| c.iter().copied().sum::<i32>() == TARGET)
        .expect("no triples that sum to 2020")
        .iter()
        .copied()
        .product()
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|s| s.parse().expect("could not parse input to integers"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_example() {
        let input = "1721\n979\n366\n299\n675\n1456";
        assert_eq!(parse_input(input), [1721, 979, 366, 299, 675, 1456]);
    }

    #[test]
    fn part_1_example() {
        let entries = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(part_1(&entries), 514579);
    }

    #[test]
    fn part_2_example() {
        let entries = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(part_2(&entries), 241861950);
    }
}
