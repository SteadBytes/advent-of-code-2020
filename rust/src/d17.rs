use crate::char_enum;
use std::collections::HashSet;
use CubeState::*;

// FIXME: Remove major duplication from part_1 and part_2 as they differ in their type
// signature (Coord3 vs Coord4).

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
            let active_neighbours = c.neighbours().filter(|n| state.contains(n)).count();

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

/// Exactly the same as `part_1`, just using 4 dimensions.
fn part_2(active_cubes: &HashSet<Coord4>) -> usize {
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
            let active_neighbours = c.neighbours().filter(|n| state.contains(n)).count();

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
    let initial_state_3d = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(&initial_state_3d));

    let initial_state_4d = initial_state_3d
        .iter()
        .map(|c| Coord4::from_3d(c))
        .collect();
    println!("Part 2: {}", part_2(&initial_state_4d));
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Clone, Copy)]
struct Coord3(i64, i64, i64);

impl<'a> Coord3 {
    fn neighbours(&'a self) -> impl Iterator<Item = Coord3> + 'a {
        (-1..=1)
            .flat_map(move |dx| (-1..=1).flat_map(move |dy| (-1..=1).map(move |dz| (dx, dy, dz))))
            .filter(|x| *x != (0, 0, 0))
            .map(move |(dx, dy, dz)| Coord3(self.0 + dx, self.1 + dy, self.2 + dz))
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Clone, Copy)]
struct Coord4(i64, i64, i64, i64);

impl<'a> Coord4 {
    fn from_3d(c: &Coord3) -> Self {
        Self(c.0, c.1, c.2, 0)
    }

    fn neighbours(&'a self) -> impl Iterator<Item = Coord4> + 'a {
        // TODO: Surely there's a better way to do this...
        (-1..=1)
            .flat_map(move |dx| {
                (-1..=1).flat_map(move |dy| {
                    (-1..=1).flat_map(move |dz| (-1..=1).map(move |dw| (dx, dy, dz, dw)))
                })
            })
            .filter(|x| *x != (0, 0, 0, 0))
            .map(move |(dx, dy, dz, dw)| Coord4(self.0 + dx, self.1 + dy, self.2 + dz, self.3 + dw))
    }
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
    fn part_2_example() {
        let initial_state_3d = parse_input(EXAMPLE_INPUT).unwrap();
        let initial_state_4d: HashSet<Coord4> = initial_state_3d
            .iter()
            .map(|c| Coord4::from_3d(c))
            .collect();

        assert_eq!(part_2(&initial_state_4d), 848);
    }
}
