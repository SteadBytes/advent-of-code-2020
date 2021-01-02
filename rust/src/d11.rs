use std::fmt;
use Location::*;
use SeatState::*;

// TODO: Plenty of room for cleanup/de-duplication between part_1 and part_2 (mainly in various
// methods on Grid) but this works and executes v fast on my laptop.

/// Key points:
/// - Rules based on adjacent seats:
///     - Empty w/ `0` adjacent occupied -> occupied
///     - Occupied w/ `>=4` visible occupied seats -> empty
///     - Floor never changes
fn part_1(layout: &mut Grid) -> usize {
    // TODO: Limit this to prevent infinite loop in the case of malformed input?
    loop {
        if !layout.tick() {
            return layout.count_occupied();
        }
    }
}

/// Key points:
/// - Rules based on **visible seats** instead of adjacent
///     - Empty w/ 0 visible occupied -> occupied
///     - Occupied w/ `>=5` visible occupied seats -> empty
///     - Floor never changes
/// - Visible seats = first seat within line of sight of each direction
///
/// `4` visible occupied seats:
///
/// ```text
/// #...|..#/
/// .\.#|../.
/// .#\.|./..
/// ...\|/...
/// ..#-L---#
/// .../|\...
/// ../.|.\..
/// #/..#..\.
/// /..#....\
/// ```
/// "Ray casting" approach:
/// - Generate coordinates for each location in a "ray" by following the points along it's vector
///     - Repeatedly add the unit vector of the ray to the current coordinate
/// - Traverse the ray coordinates until a visible seat is found or the boundary of the grid is reached
fn part_2(layout: &mut Grid) -> usize {
    loop {
        if !layout.tick_v2() {
            return layout.count_occupied();
        }
    }
}

fn parse_input(input: &str) -> Result<Grid, ParseError> {
    // TODO: Clean this up
    let width = input.lines().next().ok_or(ParseError::EmptyInput)?.len();
    let g = input
        .lines()
        .flat_map(|l| {
            if l.len() != width {
                Err(ParseError::MalformedGrid)
            } else {
                Ok(l.chars()
                    .map(|c| match c {
                        'L' => Ok(Seat(Empty)),
                        '#' => Ok(Seat(Occupied)),
                        '.' => Ok(Floor),
                        _ => Err(ParseError::InvalidCharacter(c)),
                    })
                    .collect::<Result<Vec<Location>, ParseError>>()?)
            }
        })
        .flatten()
        .collect::<Vec<Location>>();
    // Each row has already been checked for equal width -> height calculation is correct
    let height = g.len() / width;
    Ok(Grid { g, width, height })
}

pub fn run(input: &str) {
    let layout = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(&mut layout.clone()));
    println!("Part 2: {}", part_2(&mut layout.clone()));
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
enum SeatState {
    Empty,
    Occupied,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
enum Location {
    Floor,
    Seat(SeatState),
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
struct Grid {
    /// Flat representation of grid shape `(width, height)`
    g: Vec<Location>,
    width: usize,
    height: usize,
}

impl Grid {
    const ADJACENT_DIRS: [(i8, i8); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    /// Seat transition rules:
    /// - Empty & `0` adjacent occupied -> occupied
    /// - Occupied `& >= 4` adjacent occupied -> empty
    /// - Otherwise -> no change
    fn tick(&mut self) -> bool {
        let mut g_next = self.g.clone();
        let mut change = false;

        self.g
            .iter()
            .enumerate()
            .filter_map(|(i, l)| match l {
                Floor => None,
                Seat(state) => Some((i, state)),
            })
            .for_each(|(i, s)| match (s, self.adjacent_occupied(i)) {
                (Empty, 0) => {
                    change = true;
                    g_next[i] = Seat(Occupied);
                }
                (Occupied, x) if x >= 4 => {
                    change = true;
                    g_next[i] = Seat(Empty)
                }
                _ => (),
            });

        self.g = g_next;
        change
    }

    /// Seat transition rules:
    /// - Empty & `0` visible occupied -> occupied
    /// - Occupied `& >= 5` visible occupied -> empty
    /// - Otherwise -> no change
    fn tick_v2(&mut self) -> bool {
        let mut g_next = self.g.clone();
        let mut change = false;

        self.g
            .iter()
            .enumerate()
            .filter_map(|(i, l)| match l {
                Floor => None,
                Seat(state) => Some((i, state)),
            })
            .for_each(|(i, s)| match (s, self.visible_occupied(i)) {
                (Empty, 0) => {
                    change = true;
                    g_next[i] = Seat(Occupied);
                }
                (Occupied, x) if x >= 5 => {
                    change = true;
                    g_next[i] = Seat(Empty)
                }
                _ => (),
            });

        self.g = g_next;
        change
    }

    fn count_occupied(&self) -> usize {
        self.g.iter().filter(|l| **l == Seat(Occupied)).count()
    }

    fn adjacent_occupied(&self, idx: usize) -> usize {
        let (x, y) = self.pos(idx).unwrap(); // FIXME
        Grid::ADJACENT_DIRS
            .iter()
            .filter_map(|(dx, dy)| {
                // Using Option::and_then to filter out-of-bounds locations e.g. (-1, -1)
                let x1 = checked_add(x, *dx);
                let y1 = checked_add(y, *dy);
                x1.zip(y1)
                    .and_then(|(x, y)| self.loc(x, y))
                    .and_then(|i| self.g.get(i))
            })
            .filter(|l| **l == Location::Seat(Occupied))
            .count()
    }

    fn visible_occupied(&self, idx: usize) -> usize {
        let (x, y) = self.pos(idx).unwrap(); // FIXME
        Grid::ADJACENT_DIRS
            .iter()
            .filter_map(|(dx, dy)| {
                // Repeatedly apply dx, dy until seat is found or out of bounds
                let mut ray = (1..)
                    .map(|i| {
                        let x1 = checked_add(x, *dx * i);
                        let y1 = checked_add(y, *dy * i);
                        x1.zip(y1)
                            .and_then(|(x, y)| self.loc(x, y))
                            .and_then(|i| self.g.get(i))
                    })
                    // Edge of grid reached
                    .take_while(|x| x.is_some());
                // Follow ray until *first* seat
                ray.find(|l| match l {
                    Some(Seat(_)) => true,
                    _ => false,
                })
                .and_then(|l| Some(l.unwrap()))
            })
            .filter(|l| **l == Seat(Occupied))
            .count()
    }

    fn pos(&self, idx: usize) -> Option<(usize, usize)> {
        if idx < self.g.len() {
            Some((idx % self.width, idx / self.width))
        } else {
            None
        }
    }

    fn loc(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.g
                .chunks(self.width)
                .map(|c| {
                    c.iter()
                        .map(|l| match l {
                            Floor => ".",
                            Seat(Empty) => "L",
                            Seat(Occupied) => "#",
                        })
                        .collect::<Vec<_>>()
                        .join("")
                })
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

fn checked_add(x: usize, rhs: i8) -> Option<usize> {
    if rhs < 0 {
        x.checked_sub(rhs.abs() as usize)
    } else {
        x.checked_add(rhs as usize)
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum ParseError {
    EmptyInput,
    MalformedGrid,
    InvalidCharacter(char),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn parse_input_example() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT).unwrap(),
            Grid {
                g: vec![
                    Seat(Empty),
                    Floor,
                    Seat(Empty),
                    Seat(Empty),
                    Floor,
                    Seat(Empty),
                    Seat(Empty),
                    Floor,
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Floor,
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Floor,
                    Seat(Empty),
                    Floor,
                    Seat(Empty),
                    Floor,
                    Floor,
                    Seat(Empty),
                    Floor,
                    Floor,
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Floor,
                    Seat(Empty),
                    Seat(Empty),
                    Floor,
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Floor,
                    Seat(Empty),
                    Seat(Empty),
                    Floor,
                    Seat(Empty),
                    Seat(Empty),
                    Floor,
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Floor,
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Floor,
                    Seat(Empty),
                    Seat(Empty),
                    Floor,
                    Floor,
                    Seat(Empty),
                    Floor,
                    Seat(Empty),
                    Floor,
                    Floor,
                    Floor,
                    Floor,
                    Floor,
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Floor,
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Floor,
                    Seat(Empty),
                    Seat(Empty),
                    Floor,
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Seat(Empty),
                    Floor,
                    Seat(Empty),
                    Seat(Empty)
                ],
                width: 10,
                height: 10,
            }
        );
    }

    #[test]
    fn part_1_example() {
        let mut layout = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(&mut layout), 37);
    }

    #[test]
    fn part_2_example() {
        let mut layout = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_2(&mut layout), 26);
    }

    #[test]
    fn visible_occupied() {
        let test_cases = [
            (
                "\
.......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....",
                (3, 4),
                8,
            ),
            (
                "\
.............
.............
.L.L.#.#.#.#.
.............",
                (2, 2),
                0,
            ),
            (
                "\
.##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.",
                (3, 3),
                0,
            ),
            (
                "\
#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
L#LL.LL.#L
L.#L.LL.#L
L.#LLL#.#L
..L.#.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#",
                (0, 0),
                1,
            ),
        ];
        for (input, (x, y), expected) in &test_cases {
            let layout = parse_input(&input).unwrap();
            assert_eq!(
                layout.visible_occupied(layout.loc(*x, *y).unwrap()),
                *expected
            );
        }
    }
}
