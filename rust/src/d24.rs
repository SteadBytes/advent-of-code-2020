use std::collections::HashSet;
use Direction::*;
const REFERENCE_TILE: Coord3 = Coord3 { x: 0, y: 0, z: 0 };
/// Key points:
/// - *Horizontal/pointy* hex grid
///     - Must be horizontal/pointy due to input directions being east, southeast, southwest, west,
///       northwest, northeast.
/// - Reference tile is in the **center** of the room
/// - Size of the room is unbounded
///     - Though we know it's sufficient to hold the pattern
/// - Tiles have 2 sides: 1 white, 1 black.
/// - *All* tiles begin white side up
/// - Each line of input represents directions from the reference tile to a tile to be flipped
/// - Tiles are flipped in order of input (line `0` -> line `n`)
/// - Tiles may be flipped multiple times
/// - Not required to actually construct the pattern, just track how many are black
///
/// ## High-level algorithm
///
/// 1. Maintain a set of *black* tiles.
/// 2. For each line of input:
///     1. Determine the *location* of the specified tile ("follow" the directions)
///     2. Update black tile set:
///         - If the location is not in the black tile set, the tile is currently white and will be
///           flipped to black. Add to black tile set.
///         - Otherwise, the tile is currently black and will be flipped to white. Remove from
///           black tile set.
/// 3. Return the length of the black tile set.
///
/// ## Hex grid representation
///
/// Constructing the grid is not actually necessary. The above algorithm only requires determining
/// some *location* for a tile on the grid given a set of directions from the reference tile. The
/// focus, then, should be on a representation that makes "following" the directions straight
/// forward. **Cube coordinates** should work here as "standard" cartesian coordinate operations
/// (most importantly adding/subtracting coordinates) can be used. This means that the input
/// directions can be translated from a list of compass directions to a list of change in coordinates.
///
/// The reference tile is at coordinate `(x, y ,z) = (0, 0 ,0)`. Compass directions can be
/// represented as follows:
/// - East -> `(-1, +1, 0)`
/// - Southeast -> `(-1, 0, +1)`
/// - Southwest ->  `(0, -1, +1)`
/// - West ->  `(+1, -1, 0)`
/// - Northwest ->  `(+1, 0, -1)`
/// - Northeast ->  `(0, +1, -1)`
///
/// ```text
///        / \     / \
///      /     \ /     \
///     |  0,-1 | +1,-1 |
///     |   +1  |   0   |
///    / \     / \     / \
///  /     \ /     \ /     \
/// | -1,0  |  x,y  | +1,0  |
/// |  +1   |   z   |   -1  |
///  \     / \     / \     /
///    \ /     \ /     \ /
///     | -1,+1 |  0,+1 |
///     |   0   |   -1  |
///      \     / \     /
///        \ /     \ /
/// ```
fn part_1(tile_directions: &Vec<Vec<Direction>>) -> usize {
    let mut black_tiles = HashSet::new();
    for directions in tile_directions {
        let tile_loc = directions
            .iter()
            .fold(REFERENCE_TILE, |loc, dir| loc.apply(dir));
        if black_tiles.contains(&tile_loc) {
            // Currently black -> flip to white
            black_tiles.remove(&tile_loc);
        } else {
            // Currently white -> flip to black
            black_tiles.insert(tile_loc);
        }
    }

    black_tiles.len()
}

/// Key points:
/// - Rules:
///     1. Black tile w/ 0 **or** > 2 adjacent black tiles -> flip to white
///     2. White tile w/ **exactly** 2 adjacent black tiles -> flip to black
/// - Rules applied simultaneously to every tile
/// - Day 0 begins with tiles in the state produced by `part_1`
///     - TODO: Is this right? Not 100% clear
/// - Locations of *all* white tiles do not need to be known - it is sufficient to know the
///   locations of all black tiles.
///   - White tiles can only flip if they are adjacent to black tiles, therefore on each iteration
///     all tiles > 1 tile away from any black tile is assumed to remain white.
///
/// ## High-level algorithm
///
/// 1. Compute starting state using a modified version of the algorithm in `part_1`.
///     - Track white tiles?
/// 2. For each day (100 in this case):
///     - Initialise *empty* set of black tiles
///     - Initialise *empty* set of white tiles *to check*
///     - Apply rule 1 to black tiles to partially build black tiles set and fully build white
///     tiles set:
///         - Insert into black tile set if rule **is not** matched (remains black)
///         - Insert any adjacent white tiles into white tiles set
///     - Apply rule 2 to white tiles set
///         - Insert into black tile set if rule **is** matched (flipped to black)
///     - Apply the changes by swapping the previous iteration black tile set with the current one
///         - TODO: Can this be avoided by tracking a diff rather than complete state?
/// 3. Return the length of the black tile set
fn part_2(tile_directions: &Vec<Vec<Direction>>) -> usize {
    // Build initial state via part 1 algorithm
    let mut black_tiles = HashSet::new();
    tile_directions
        .iter()
        .map(|directions| {
            directions
                .iter()
                .fold(REFERENCE_TILE, |loc, dir| loc.apply(&dir))
        })
        .for_each(|tile_loc| {
            if black_tiles.contains(&tile_loc) {
                // Currently black -> flip to white
                black_tiles.remove(&tile_loc);
            } else {
                // Currently white -> flip to black
                black_tiles.insert(tile_loc);
            }
        });

    // TODO: This can definitely be optimised to reduce the amount of copying/moving around that
    // takes place. That said, both part_1 and part_2 complete in ~0.1s on my machine so I'll leave
    // at that for now...
    for _ in 0..100 {
        let mut next_black = HashSet::new();
        let mut current_white = HashSet::new();
        for tile_loc in &black_tiles {
            let (black_adj, white_adj): (Vec<Coord3>, Vec<Coord3>) = tile_loc
                .iter_adjacent()
                .partition(|loc| black_tiles.contains(loc));
            if !(black_adj.len() == 0 || black_adj.len() > 2) {
                next_black.insert(*tile_loc);
            }
            current_white.extend(white_adj);
        }
        for tile_loc in current_white {
            if tile_loc
                .iter_adjacent()
                .filter(|loc| black_tiles.contains(loc))
                .count()
                == 2
            {
                next_black.insert(tile_loc);
            }
        }
        black_tiles = next_black
    }

    black_tiles.len()
}

fn parse_input(input: &str) -> Result<Vec<Vec<Direction>>, ParseError> {
    input
        .lines()
        .enumerate()
        .map(|(lineno, l)| {
            // TODO: Is there a cleaner way to do this e.g. with a `fold`?
            let mut r = vec![];
            let mut col = 0;
            while col < l.len() {
                if let Ok(d) = l[col..col + 1].parse::<Direction>() {
                    col += 1;
                    r.push(d);
                } else if let Ok(d) = l[col..col + 2].parse::<Direction>() {
                    col += 2;
                    r.push(d);
                } else {
                    return Err(ParseError::InvalidDirections(lineno, col));
                }
            }

            Ok(r)
        })
        .collect()
}

pub fn run(input: &str) {
    let tile_directions = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(&tile_directions));
    println!("Part 2: {}", part_2(&tile_directions));
}

str_enum! {
    Direction {
        (East, "e"),
        (Southeast, "se"),
        (Southwest, "sw"),
        (West, "w"),
        (Northwest, "nw"),
        (Northeast, "ne"),
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Copy, Clone)]
struct Coord3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Coord3 {
    fn apply(&self, dir: &Direction) -> Coord3 {
        let (x, y, z) = match dir {
            East => (self.x + 1, self.y - 1, self.z),
            Southeast => (self.x, self.y - 1, self.z + 1),
            Southwest => (self.x - 1, self.y, self.z + 1),
            West => (self.x - 1, self.y + 1, self.z),
            Northwest => (self.x, self.y + 1, self.z - 1),
            Northeast => (self.x + 1, self.y, self.z - 1),
        };

        Coord3 { x, y, z }
    }

    fn iter_adjacent<'a>(&'a self) -> impl Iterator<Item = Coord3> + 'a {
        [East, Southeast, Southwest, West, Northwest, Northeast]
            .iter()
            .map(move |dir| self.apply(dir))
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum ParseError {
    /// `(lineno, col)`
    InvalidDirections(usize, usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn parse_input_example() {
        let tile_directions = parse_input(EXAMPLE_INPUT).unwrap();
        // Just testing the first two to save typing out all the rest...
        assert_eq!(
            tile_directions[..2],
            [
                vec![
                    Southeast, Southeast, Northwest, Northeast, Northeast, Northeast, West,
                    Southeast, East, Southwest, West, Southwest, Southwest, West, Northeast,
                    Northeast, West, Southeast, West, Southwest
                ],
                vec![
                    Northeast, East, East, Northeast, Southeast, Northwest, Northwest, West,
                    Southwest, Northeast, Northeast, West, Northwest, West, Southeast, West,
                    Northeast, Northwest, Southeast, Southwest, East, Southwest
                ]
            ]
        );
    }

    #[test]
    fn apply_direction() {
        let loc = Coord3 { x: 0, y: 0, z: 0 };
        assert_eq!(loc.apply(&East), Coord3 { x: 1, y: -1, z: 0 });
        assert_eq!(loc.apply(&Southeast), Coord3 { x: 0, y: -1, z: 1 });
        assert_eq!(loc.apply(&Southwest), Coord3 { x: -1, y: 0, z: 1 });
        assert_eq!(loc.apply(&West), Coord3 { x: -1, y: 1, z: 0 });
        assert_eq!(loc.apply(&Northwest), Coord3 { x: 0, y: 1, z: -1 });
        assert_eq!(loc.apply(&Northeast), Coord3 { x: 1, y: 0, z: -1 });
    }

    #[test]
    fn part_1_example() {
        let tile_directions = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(&tile_directions), 10);
    }

    #[test]
    fn part_2_example() {
        let tile_directions = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_2(&tile_directions), 2208);
    }
}
