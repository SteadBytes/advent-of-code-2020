use std::cmp::min;

/// The puzzle description is, I suspect, deliberately convoluted. Since *every* adapter must be
/// used and adapter can only connect to another that is 1-3 jolts lower itself there is only
/// a single valid configuration - sorted in ascending order.
///
/// Assumes `ratings` is *already* sorted.
fn part_1(ratings: &Vec<usize>) -> Result<usize, Error> {
    let mut diff_1 = 0;
    let mut diff_3 = 0;

    for (a, b) in ratings[..ratings.len() - 1]
        .iter()
        .zip(ratings[1..ratings.len()].iter())
    {
        match b - a {
            1 => diff_1 += 1,
            2 => (),
            3 => diff_3 += 1,
            x => return Err(Error::InvalidAdapterDifference(x)),
        }
    }

    Ok(diff_1 * diff_3)
}

/// Dynamic programming approach based on the following observations:
/// - Not *all* the adapters need to be used
/// - First and last adapters (0 and max(ratings) + 3) are always in the same position (e.g.
///   1 valid arrangement)
/// - Each adapter has *at most* 3 others it can connect to e.g. there exists adapters with ratings
///   +1, +2 and +3 from it. It's not possible to have more than this otherwise there would be no
///   valid arrangements of all the adapters.
///
/// Assumes `ratings` is *already* sorted.
fn part_2(ratings: &Vec<usize>) -> Result<usize, Error> {
    let n = ratings.len();
    // Holds cumulative number of arrangements for each adapter - counts[i] is the number
    // of valid arrangements for adapters in ratings[i..n]. At the end of the loop
    // below, counts[0] contains the number of arrangements for all adapters.
    let mut counts = vec![0; n];
    // 1 arrangement of all adapters (sorted order)
    counts[n - 1] = 1;

    for (i, x) in ratings.iter().enumerate().rev().skip(1) {
        // Search the 3 possible other adapters
        let (lo, hi) = (i + 1, min(i + 3, n - 1)); // Stop at index 0
        counts[i] = (lo..=hi) // Indices of possible other adapters
            .filter(|&j| ratings[j] - x <= 3) // Valid connections
            .map(|j| counts[j]) // Number of arrangments for other adapters
            .sum() // Total arrangements for this adapter
    }

    Ok(counts[0])
}

fn parse_input(input: &str) -> Result<Vec<usize>, Error> {
    let mut ratings: Vec<usize> = input
        .lines()
        .map(|l| l.trim().parse().map_err(|e| Error::Parse(e)))
        .collect::<Result<Vec<usize>, Error>>()?;
    ratings.push(0);
    ratings.sort();
    ratings.push(ratings.last().expect("empty input") + 3);
    Ok(ratings)
}

pub fn run(input: &str) {
    let ratings = parse_input(input).expect("unable to parse input");
    println!(
        "Part 1: {}",
        part_1(&ratings).expect("invalid adapter difference encountered")
    );
    println!("Part 2: {}", part_2(&ratings).unwrap()); // FIXME
}

#[derive(PartialEq, Eq, Debug)]
enum Error {
    Parse(std::num::ParseIntError),
    InvalidAdapterDifference(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "\
16
10
15
5
1
11
7
19
6
12
4
";
    const EXAMPLE_INPUT_2: &str = "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
";

    #[test]
    fn parse_input_example_1() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT_1).unwrap(),
            [0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22]
        );
    }

    #[test]
    fn parse_input_example_2() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT_2).unwrap(),
            [
                0, 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34,
                35, 38, 39, 42, 45, 46, 47, 48, 49, 52
            ]
        );
    }

    #[test]
    fn part_1_example_1() {
        let ratings = parse_input(EXAMPLE_INPUT_1).unwrap();
        assert_eq!(part_1(&ratings).unwrap(), 35);
    }

    #[test]
    fn part_1_example_2() {
        let ratings = parse_input(EXAMPLE_INPUT_2).unwrap();
        assert_eq!(part_1(&ratings).unwrap(), 220);
    }

    #[test]
    fn part_2_example_1() {
        let ratings = parse_input(EXAMPLE_INPUT_1).unwrap();
        assert_eq!(part_2(&ratings).unwrap(), 8);
    }

    #[test]
    fn part_2_example_2() {
        let ratings = parse_input(EXAMPLE_INPUT_2).unwrap();
        assert_eq!(part_2(&ratings).unwrap(), 19208);
    }
}
