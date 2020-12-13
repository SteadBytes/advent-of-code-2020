use std::fmt;
use Location::*;
use SeatState::*;

fn part_1(layout: &mut Grid) -> usize {
    // TODO: Limit this to prevent infinite loop in the case of malformed input?
    loop {
        if !layout.tick() {
            return layout.count_occupied();
        }
    }
}

fn part_2(layout: &mut Grid) -> usize {
    todo!()
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
    // println!("Part 2: {}", part_2(&parsed));
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
    /// Seat transition rules:
    /// - Empty & no adjacent occupied -> occupied
    /// - Occupied & >= 4 adjacent occupied -> empty
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

    fn count_occupied(&self) -> usize {
        self.g.iter().filter(|l| **l == Seat(Occupied)).count()
    }

    fn adjacent_occupied(&self, idx: usize) -> usize {
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
        let (x, y) = self.pos(idx).unwrap(); // FIXME
        ADJACENT_DIRS
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
}
