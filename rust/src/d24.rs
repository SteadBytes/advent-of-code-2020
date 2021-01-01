use crate::char_enum;
use std::collections::HashSet;
use Direction::*;
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
/// ## Initial high-level algorithm
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
        let tile_loc = directions.iter().fold((0, 0, 0), |loc, dir| dir.apply(loc));
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

fn part_2() {
    todo!()
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
    // println!("Part 2: {}", part_2(&parsed));
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

impl Direction {
    fn apply(&self, (x, y, z): (i32, i32, i32)) -> (i32, i32, i32) {
        match self {
            East => (x + 1, y - 1, z),
            Southeast => (x, y - 1, z + 1),
            Southwest => (x - 1, y, z + 1),
            West => (x - 1, y + 1, z),
            Northwest => (x, y + 1, z - 1),
            Northeast => (x + 1, y, z - 1),
        }
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
    fn direction_apply() {
        let loc = (0, 0, 0);
        assert_eq!(East.apply(loc), (1, -1, 0));
        assert_eq!(Southeast.apply(loc), (0, -1, 1));
        assert_eq!(Southwest.apply(loc), (-1, 0, 1));
        assert_eq!(West.apply(loc), (-1, 1, 0));
        assert_eq!(Northwest.apply(loc), (0, 1, -1));
        assert_eq!(Northeast.apply(loc), (1, 0, -1));
    }

    #[test]
    fn part_1_example() {
        let tile_directions = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(&tile_directions), 10);
    }

    // #[test]
    // fn part_2_example() {
    //     let parsed = parse_input(EXAMPLE_INPUT).unwrap();
    //     assert_eq!(part_2(&parsed),);
    // }
}
