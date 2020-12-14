type Schedule = Vec<(usize, u64)>;

fn part_1(earliest_timestamp: u64, schedule: &Schedule) -> u64 {
    let mut best_bus: Option<u64> = None;
    let mut min_wait = u64::MAX;
    for (_, id) in schedule {
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

fn parse_input(input: &str) -> Result<(u64, Vec<(usize, u64)>), ParseError> {
    let mut lines = input.lines();
    let timestamp = lines
        .next()
        .ok_or(ParseError::MissingTimestamp)?
        .parse::<u64>()
        .map_err(|e| ParseError::InvalidTimestamp(e))?;
    let schedule = lines
        .next()
        .ok_or(ParseError::MissingBusIds)?
        .split(",")
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(i, s)| {
            Ok((
                i,
                s.parse::<u64>().map_err(|e| ParseError::InvalidBusId(e))?,
            ))
        })
        .collect::<Result<Vec<(usize, u64)>, ParseError>>()?;

    Ok((timestamp, schedule))
}

pub fn run(input: &str) {
    let (earliest_timestamp, schedule) = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(earliest_timestamp, &schedule));
    // println!("Part 2: {}", part_2(&schedule));
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
            (939, vec![(0, 7), (1, 13), (4, 59), (6, 31), (7, 19)])
        );
    }

    #[test]
    fn part_1_example() {
        let (ts, schedule) = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(ts, &schedule), 295);
    }

    #[test]
    fn part_2_example() {}
}
