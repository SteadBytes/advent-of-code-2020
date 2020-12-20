use crate::char_enum;
use std::collections::HashSet;
use CubeState::*;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Clone, Copy)]
struct Coord3(i64, i64, i64);

impl Coord3 {
    // TODO: return impl Iterator<Item = Coord3>
    fn neighbours(&self) -> Vec<Coord3> {
        let mut r = Vec::with_capacity(26);
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if dx == 0 && dy == 0 && dz == 0 {
                        continue;
                    }
                    r.push(Coord3(self.0 + dx, self.1 + dy, self.2 + dz))
                }
            }
        }
        r
    }
}

/// Observation: Only need to keep track of the coordinates containing active cubes.
///
/// All required information can de derived from knowing the current set of coordinates
/// containing active cubes. The order in which cubes are inspected in a cycle does not
/// matter as state changes all occur simulataneously. The neighbours of a given cube can
/// be calculated from it's coordinate and it's state derived from the set of active cubes (e.g. in
/// the set -> active, not in the set -> inactive).
///
/// Observation: Since the grid is infinite, neighbour coordinates have no bounds
/// (e.g. is this cube on the edge of the grid).
///
/// Using a set structure instead of directly representing the state as a grid e.g.
/// `Vec<Vec<Vec<CubeState>>>` removes the need to grow the grid as the active cubes move
/// "out"removes the need to grow the grid as the active cubes move "outwards". It is also more
/// memory efficient as only the active cubes are tracked at any given time rather than all
/// cubes.
fn part_1(active_cubes: &HashSet<Coord3>) -> usize {
    let mut state = active_cubes.clone();
    for _ in 0..6 {
        let mut next_state = HashSet::new(); // TODO: Avoid this?
        let mut inactive = HashSet::new(); // TODO: Avoid this?
        for c in state.iter() {
            // Iterate through neighbours
            // - Update inactive set
            // - Count active neighbours
            // Update next_state according to active_neighbours
            let mut active_neighbours = 0;
            for n in c.neighbours() {
                if state.contains(&n) {
                    active_neighbours += 1;
                } else {
                    inactive.insert(n);
                }
            }

            // Apply rule 1:
            // > If a cube is active and exactly 2 or 3 of its neighbors are also active,
            // > the cube remains active. Otherwise, the cube becomes inactive.
            if active_neighbours == 2 || active_neighbours == 3 {
                next_state.insert(*c);
            } else {
                inactive.insert(*c);
            }
        }

        for c in inactive.iter() {
            // Count active neighbours & update next_state accordingly
            let active_neighbours = c.neighbours().iter().filter(|n| state.contains(n)).count();

            // Apply rule 2:
            // > If a cube is inactive but exactly 3 of its neighbors are active, the cube
            // > becomes active. Otherwise, the cube remains inactive.
            if active_neighbours == 3 {
                next_state.insert(*c);
            }
        }
        // Update all cubes simulataneously
        state = next_state;
    }
    state.len()
}

fn part_2() {}

fn parse_input(input: &str) -> Result<HashSet<Coord3>, ParseError> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| match CubeState::new(c) {
                    Ok(state) => Ok((Coord3(x as i64, y as i64, 0_i64), state)),
                    Err(_) => Err(ParseError::MalformedInput),
                })
        })
        .filter_map(|x| match x {
            Ok((coord, Active)) => Some(Ok(coord)),
            Ok((_, _)) => None,
            Err(e) => Some(Err(e)),
        })
        .collect()
}

pub fn run(input: &str) {
    let initial_state = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(&initial_state));
    // println!("Part 2: {}", part_2(&parsed));
}

char_enum! {
    CubeState {
        (Active, '#'),
        (InActive, '.'),
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum ParseError {
    MalformedInput,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
.#.
..#
###";

    #[test]
    fn parse_input_example() {
        let expected_coords: [Coord3; 5] = [
            Coord3(1, 0, 0),
            Coord3(2, 1, 0),
            Coord3(0, 2, 0),
            Coord3(1, 2, 0),
            Coord3(2, 2, 0),
        ];
        assert_eq!(
            parse_input(EXAMPLE_INPUT).unwrap(),
            expected_coords.iter().copied().collect()
        );
    }

    #[test]
    fn part_1_example() {
        let state = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(&state), 112);
    }

    #[test]
    fn part_2_example() {}
}
