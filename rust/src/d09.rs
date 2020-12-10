use itertools::Itertools;

const PREAMBLE_LEN: usize = 25;

/// Find the first number (after the first `preamble_len` numbers) in `data` which does not have
/// the XMAS encoding property of being the *sum* of two of the `preamble_len` numbers before it.
///
/// - Values must be different
/// - There may exist multiple such pairs
fn part_1(data: &Vec<usize>, preamble_len: usize) -> Result<usize, Error> {
    // TODO: Use a more sophisticated algorithm (D.P?)
    // Brute force approach: Starting from i = preamble_len, iterate through length 2 combinations
    // of data[i - preamble_len: preamble_len]. If any pair has the property, increment i and move
    // on.  Otherwise return data[i].
    for i in preamble_len..data.len() {
        if data[i - preamble_len..i]
            .iter()
            .combinations(2)
            .find(|c| c.iter().copied().sum::<usize>() == data[i])
            == None
        {
            return Ok(data[i]);
        }
    }

    Err(Error::NotFound)
}

fn part_2() {}

/// Assuming XMAS encryption doesn't support negative numbers.
fn parse_input(input: &str) -> Result<Vec<usize>, Error> {
    input
        .lines()
        .map(|l| l.trim().parse().map_err(|e| Error::Parse(e)))
        .collect()
}

pub fn run(input: &str) {
    let encoded_data = parse_input(input).expect("unable to parse input");

    println!(
        "Part 1: {}",
        part_1(&encoded_data, PREAMBLE_LEN).expect(
            "unable to find number in input which does not have the XMAS encoding property"
        )
    );
    // println!("Part 2: {}", part_2(&parsed));
}

#[derive(PartialEq, Eq, Debug)]
enum Error {
    NotFound,
    Parse(std::num::ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
";

    #[test]
    fn parse_input_example() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT).unwrap(),
            vec![
                35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                309, 576,
            ]
        );
    }

    #[test]
    fn part_1_example() {
        let data = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(&data, 5).unwrap(), 127);
    }

    #[test]
    fn part_2_example() {}
}
