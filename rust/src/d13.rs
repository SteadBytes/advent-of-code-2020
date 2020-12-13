fn part_1(earliest_timestamp: u32, bus_ids: &Vec<u32>) -> u32 {
    let mut best_bus: Option<u32> = None;
    let mut min_wait = u32::MAX;
    for id in bus_ids {
        let wait_time = id - (earliest_timestamp % id);
        if wait_time < min_wait {
            min_wait = wait_time;
            best_bus = Some(*id);
        }
    }
    // FIXME
    best_bus.unwrap() * min_wait
}

fn part_2() {}

fn parse_input(input: &str) -> Result<(u32, Vec<u32>), ParseError> {
    let mut lines = input.lines();
    let timestamp = lines
        .next()
        .ok_or(ParseError::MissingTimestamp)?
        .parse::<u32>()
        .map_err(|e| ParseError::InvalidTimestamp(e))?;
    let bus_ids = lines
        .next()
        .ok_or(ParseError::MissingBusIds)?
        .split(",")
        .filter(|s| *s != "x")
        .map(|s| s.parse::<u32>().map_err(|e| ParseError::InvalidBusId(e)))
        .collect::<Result<Vec<u32>, ParseError>>()?;

    Ok((timestamp, bus_ids))
}

pub fn run(input: &str) {
    let (earliest_timestamp, bus_ids) = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(earliest_timestamp, &bus_ids));
    // println!("Part 2: {}", part_2(&parsed));
}

#[derive(PartialEq, Eq, Debug)]
enum ParseError {
    MissingTimestamp,
    MissingBusIds,
    InvalidTimestamp(std::num::ParseIntError),
    InvalidBusId(std::num::ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
939
7,13,x,x,59,x,31,19";

    #[test]
    fn parse_input_example() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT).unwrap(),
            (939, vec![7, 13, 59, 31, 19])
        );
    }

    #[test]
    fn part_1_example() {
        let (ts, bus_ids) = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(ts, &bus_ids), 295);
    }

    #[test]
    fn part_2_example() {}
}
