fn part_1() {
    todo!()
}

fn part_2() {
    todo!()
}

fn parse_input(input: &str) -> Result<(), ParseError> {
    todo!()
}

pub fn run(input: &str) {
    let parsed = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(&parsed));
    // println!("Part 2: {}", part_2(&parsed));
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
";

    #[test]
    fn parse_input_example() {}

    #[test]
    fn part_1_example() {
        let parsed = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(&parsed), );
    }

    #[test]
    fn part_2_example() {
        let parsed = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_2(&parsed), );
    }
}
