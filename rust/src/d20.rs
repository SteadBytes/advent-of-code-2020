use crate::str::{rsplit_once, split_once};
use std::collections::HashSet;

/// Key points:
/// - Tiles are rotated **and** flipped
/// - Tile borders line up exactly
///     - Adjacent edges are identical
/// - Outermost edges won't line up with any other tiles
///
/// Observations:
///
/// Note: Diagrams are for illustrative purposes only and may not represent actually *correct*
/// puzzle layouts
///
/// - All tiles are squares of the same size
/// - Only the tile edges are required to determine a correct layout (inner data does not affect
///   the validity of a layout).
/// - Flipping an edge is equivalent to *reversing* it
///     - `..#.###... -> ...###.#..`
/// - AFAICT the border on each adjacent edge is unique (there will only be one other matching edge
///   for that pattern).
/// - Corner tiles will have *exactly 2* matched edges (`x`s denote unmatched patterns)
///     ```text
///     xxxx#
///     x   #
///     x   .
///     x   .
///     #..##
///     ```
/// - Center perimeter tiles will have *exactly 3* matched edges
///     ```text
///     xxxx# #xxx# #xxxx
///     x   # #   . .   x
///     x   . .   # #   x
///     x   . .   # #   x
///     #..## #...# #.#.#
///     ```
/// - Interior tiles will have *exactly 4* matched edges
///     ```text
///     xxxx# #xxx# #xxxx
///     x   # #   . .   x
///     x   . .   # #   x
///     x   . .   # #   x
///     #..## #...# #.#.#
///
///     #..## #...# #.#.#
///     x   . .   # #   x
///     x   . .   # #   x
///     x   . .   # #   x
///     #..## #...# #.##.
///
///     #..## #...# #.##.
///     x   . .   # #   x
///     x   # #   # #   x
///     x   . .   . .   x
///     xxxx. .xxx. .xxxx
///     ```
///
/// Solution:
/// - Extract tile Ids and edges (first/last columns, top/bottom rows) from input
/// - Compute the flipped versions (inverses) of the edges
/// - Find the corner tiles:
///     - For each pair of tiles, calculate the set difference between the edges of the first tile
///       and the transformations of the edges of the other tile (edges & their inverse).
///     - If the resulting set contains only 2 edges -> the first tile in the pair is a corner
///       tile.
/// - Return the product of the corner tile Ids
fn part_1(tiles: &[Tile]) -> u64 {
    // For good input that meets the above assumptions `collect`ing first into a `Vec` is not
    // necessary (use `Iterator::product`). However, I wanted to sanity check that *exactly*
    // 4 corner Ids had been found before returning a result to ensure errors in my assumptions
    // and/or the input data were clearer.
    let corner_ids: Vec<u64> = tiles
        .iter()
        .filter(|t1| {
            t1.edges
                .iter()
                .filter(|&e| {
                    tiles
                        .iter()
                        .filter(|t2| t1 != t2)
                        .any(|t2| t2.possible_edges.contains(e))
                })
                .count()
                == 2
        })
        .map(|t| t.id)
        .collect();

    // Sanity check
    assert_eq!(corner_ids.len(), 4);

    corner_ids.iter().product()
}

fn part_2() {}

// FIXME: Improve error handling (ParseError::InvalidInput provides 0 useful information).
fn parse_input(input: &str) -> Result<Vec<Tile>, ParseError> {
    input
        .trim() // Puzzle input has a trailing newline
        .split("\n\n")
        .map(|tile| {
            let (heading, grid) = split_once(tile, "\n").ok_or(ParseError::InvalidInput)?;
            let id = rsplit_once(heading, " ")
                .and_then(|(id, _)| id.strip_suffix(":"))
                .ok_or(ParseError::InvalidInput)
                .and_then(|id| id.parse().map_err(|e| ParseError::InvalidId(e)))?;

            let data: Vec<&str> = grid.lines().collect();
            // Sanity check dimensions
            let height = data.len();
            let width = data[0].len();
            if height == 0 || width != height || data.iter().any(|r| r.len() != width) {
                return Err(ParseError::InvalidInput);
            }

            // Unchecked indexing/unwraps OK because of above sanity checks
            let edges: HashSet<String> = [
                // Top row
                data[0].to_string(),
                // Bottom row
                data[width - 1].to_string(),
                // First col
                data.iter().map(|r| r.chars().next().unwrap()).collect(),
                // Last col
                data.iter().map(|r| r.chars().last().unwrap()).collect(),
            ]
            .iter()
            .cloned()
            .collect();

            let possible_edges = edges
                .iter()
                .cloned()
                .chain(edges.iter().map(|e| e.chars().rev().collect()))
                .collect();

            Ok(Tile {
                id,
                edges,
                possible_edges,
            })
        })
        .collect()
}

pub fn run(input: &str) {
    let tiles = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(&tiles));
    // println!("Part 2: {}", part_2(&parsed));
}

#[derive(PartialEq, Eq, Debug)]
enum ParseError {
    InvalidInput,
    InvalidId(std::num::ParseIntError),
}

#[derive(PartialEq, Eq, Debug)]
struct Tile {
    id: u64,
    /// First/last columns, top/bottom rows
    edges: HashSet<String>,
    /// First/last columns, top/bottom rows *and* their inverses. Computed in advance
    /// and stored on `Tile` struct to avoid re-creating it multiple times in `part_1`.
    possible_edges: HashSet<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    #[test]
    fn parse_input_example() {
        // First 2 tiles from example input,
        let input = "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..";

        assert_eq!(
            parse_input(&input).unwrap(),
            [
                Tile {
                    id: 2311,
                    edges: [".#####..#.", "...#.##..#", "..##.#..#.", "..###..###"]
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                    #[rustfmt::skip]
                    possible_edges: [
                        // Normal edges
                        ".#####..#.", "...#.##..#", "..##.#..#.", "..###..###",
                        // Flipped edges
                        ".#..#####.", "#..##.#...", ".#..#.##..", "###..###..",
                    ]
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                },
                Tile {
                    id: 1951,
                    edges: [".#####..#.", "#...##.#..", "#.##...##.", "##.#..#..#"]
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                    #[rustfmt::skip]
                    possible_edges: [
                        // Normal edges
                        ".#####..#.", "#...##.#..", "#.##...##.", "##.#..#..#",
                        // Flipped edges
                        ".#..#####.", "..#.##...#", ".##...##.#", "#..#..#.##",
                    ]
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                }
            ]
        );
    }

    #[test]
    fn part_1_example() {
        let tiles = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(&tiles), 20899048083289);
    }

    #[test]
    fn part_2_example() {}
}
