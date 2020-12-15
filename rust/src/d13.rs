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

/// ## Examples
///
/// 1. `[7,13,x,x,59,x,31,19]`
///
/// Find the earliest timestamp `t` such that:
///
/// - `7` departs @ `t`
/// - `13` departs @ `t + 1`
/// - `59` departs @ `t + 4`
/// - `31` departs @ `t + 6`
/// - `19` departs @ `t + 7`
///
/// There are no restrictions for times `t + 2`, `t + 3` and `t + 5`.
///
/// Answer: `t = 1068781`
///
/// 2. `[17,x,13,19]`
///
/// Find the earliest timestamp `t` such that:
///
/// - `17` departs @ `t`
/// - `13` departs @ `t + 2`
/// - `19` departs @ `t + 3`
///
/// There are no restrictions for time `t + 1`.
///
/// Answer: `t = 3417`
///
/// 3. `[67, 7, 59, 61]`
///
/// Find the earliest timestamp `t` such that:
///
/// - `67` departs @ `t`
/// - `7` departs @ `t + 1`
/// - `59` departs @ `t + 2`
/// - `61` departs @ `t + 3`
///
/// Answer: `t = 754018`
///
/// 4. `[67, x, 7, 59, 61]`
///
/// Find the earliest timestamp `t` such that:
///
/// - `67` departs @ `t`
/// - `7` departs @ `t + 2`
/// - `59` departs @ `t + 3`
/// - `61` departs @ `t + 4`
///
/// There are no restrictions for time `t + 1`.
///
/// Answer: `t = 779210`
///
/// 5. `[67, 7, x, 59, 61]`
///
/// Find the earliest timestamp `t` such that:
///
/// - `67` departs @ `t`
/// - `7` departs @ `t + 1`
/// - `59` departs @ `t + 4`
/// - `61` departs @ `t + 5`
///
/// There are no restrictions for time `t + 2`.
///
/// Answer: `t = 1261476`
///
/// 6. `[1789, 37, 47, 1889]`
///
/// Find the earliest timestamp `t` such that:
///
/// - `1789` departs @ `t`
/// - `37` departs @ `t + 1`
/// - `47` departs @ `t + 2`
/// - `1889` departs @ `t + 3`
///
/// Answer: `t = 1202161486`
///
/// ## General case
///
/// Find a `t` such that `(t+i) % xi == 0` for a given schedule `[x1, x2, ..., xn]`.
///
/// Observation: All the bus IDs (all `xi`) are **prime numbers** (at least in the examples).
/// - `xi` must be a prime factor of `t + i`
///
/// Observation: The period of a pattern of bus IDs is equal to the LCM of the bus IDs. For example,
/// given the schedule `[2, 3]`:
///
/// ```text
/// t    bus 2    bus 3
/// 0      D        D
/// 1      -        -
/// 2     *D*       -  <---- [2, 3]
/// 3      -       *D*
/// 4      D        -
/// 5      -        -
/// 6      D        D
/// 7      -        -
/// 8     *D*       -  <---- [2, 3]
/// 9      -       *D*
/// ...
/// ```
///
/// The first occurence of bus 2 departing at time `t` and bus 3 departing at time `t + 1` is `t
/// = 2`. The next occurence of this is `t = 8`. The period of this pattern is `6` and `LCM(2, 3)
/// = 6`.
///
/// This extends to the next bus ID - the period of the above pattern in addition to the next bus
/// ID departing at `t + 2` is equal to the LCM of the period of the above pattern and the next bus
/// ID (equivalently, the LCM of all the bus IDs so far):
///
/// ```text
/// t    bus 2    bus 3    bus 5
/// 1     D        D        D
/// 2     -        -        -
/// 3     D        -        -
/// 4     -        D        -  <---- [2, 3]
/// 5     D        -        -
/// 6     -        -        D
/// 7     D        D        -
/// 8     -        -        -
/// 9    *D*       -        -  <---- [2, 3, 5]
/// 10    -       *D*       -
/// 11    D        -       *D*
/// 12    -        -        -
/// 13    D        D        -
/// 14    -        -        -
/// 15    D        -        -
/// 16    -        D        D  <---- [2, 3]
/// 17    D        -        -
/// 18    -        -        -
/// 19    D        D        -
/// 20    -        -        -
/// 21    D        -        D
/// 22    -        D        -  <---- [2, 3]
/// 23    D        -        -
/// 24    -        -        -
/// 25    D        D        -
/// 26    -        -        D
/// 27    D        -        -
/// 28    -        D        -  <---- [2, 3]
/// 29    D        -        -
/// 30    -        -        -
/// 31    D        D        D
/// 32    -        -        -
/// 33    D        -        -
/// 34    -        D        -  <---- [2, 3]
/// 35    D        -        -
/// 36    -        -        D
/// 37    D        D        -
/// 38    -        -        -
/// 39   *D*       -        -  <---- [2, 3, 5]
/// 40    -       *D*       -
/// 41    D        -       *D*
/// ```
///
/// The first occurence of bus 2 departing at time `t` and bus 3 departing at time `t + 1` and bus
/// 5 departing at time `t + 2` is `t = 9`. The next occurence of this is `t = 39`. The period of
/// this pattern is `30` and `LCM(2, 3, 5) = LCM(LCM(2, 3), 5) = LCM(6, 5) = 30`.
///
/// This holds for all the bus IDs in the schedule (given that all the bus IDs are prime).
///
/// This forms the basis of a systematic search for `t` which reduces the search space greatly from
/// a brute force search from `1` to `t`:
///
/// - Start with a step size of `1`.
/// - Linearly search for the first value of `t` when the first bus will depart at the correct
///   time.
/// - *Multiply* the step size by the first bus ID.
///   - LCM of prime numbers is their product.
///   - Step size remains at the LCM of previous bus ID(s).
/// - Using the *increased* step size, linearly search for the *next* `t` such that that next bus
///   will depart at the correct time.
///   - Increasing the search step size like this ensures that *only* values for `t` where the
///     previous bus ID(s) pattern is correct are tested.
/// - Repeat for the entire schedule.
/// - The final value of `t` is the earliest timestamp such that each bus ID in the schedule
///   departs at their offsets within the schedule.
///
///   Note: This algorithm defintely requires 64 bit integers and (although not for my puzzle
///   input) potentially some form of "big integer" representation if 64 bits doesn't suffice.
///
///   TODO: I'm fairly confident that there's are more sophisticated solution using number theory
///   (primes, modular arithmetic) that avoids a large amount (if not all) of the linear searching.
///   Something is tickling the back of my brain from mathematics classes but I can't quite
///   materialise it :/ However, this completes pretty much instantaneously so I'm happy with it for
///   now.
fn part_2(schedule: &Schedule) -> u64 {
    let mut t = 1;
    let mut step = 1;
    for (i, id) in schedule {
        while (t + *i as u64) % id != 0 {
            t += step;
        }
        step *= id;
    }
    t
}

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
    println!("Part 2: {}", part_2(&schedule));
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

    // TODO: Move these to doctests on part_2 to go along with explanation?
    #[test]
    fn part_2_example() {
        let (_, bus_ids) = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_2(&bus_ids), 1068781);
    }

    #[test]
    fn part_2_other_examples() {
        let tests = [
            (vec![(0, 17), (2, 13), (3, 19)], 3417),
            (vec![(0, 67), (1, 7), (2, 59), (3, 61)], 754018),
            (vec![(0, 67), (2, 7), (3, 59), (4, 61)], 779210),
            (vec![(0, 67), (1, 7), (3, 59), (4, 61)], 1261476),
            (vec![(0, 1789), (1, 37), (2, 47), (3, 1889)], 1202161486),
        ];
        for (schedule, ts) in tests.iter() {
            assert_eq!(part_2(&schedule), *ts);
        }
    }
}
