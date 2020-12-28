// TODO: The game state and circular list representation could be wrappeed up in
// structs to ensure their respective invariants are maintained and to provide a better
// API. This works though and I've spent too much time on it as is! :D

/// Key points:
/// - **Clockwise** circular list of cups
/// - First cup in **input** = starting *current cup*
/// - For 100 turns:
///     - 3 cups clockwise of current cup are "picked up" (removed)
///     - Destination cup is chosen as the cup with label == cup - 1
///         - If cup - 1 is a label of picked up cups, keep subracting 1 as necessary and
///           wrap around to choose highest labelled cup if 0 is reached.
///     - Place picked up cups clockwise of destination cup
///     - Current cup = cup clockwise of current cup
/// - Return value should be string of all the labels on the cups starting after cup 1
///
/// ## Data structures
///
/// Operations required:
/// - Removes of *next* `n` cups from any position
///     - Pick up chosen cups
/// - Inserts of `n` cups at any position
///     - Insert picked up cups after destination
/// - Access to cup with a specific label
///     - Choose destination cup by current cup label - 1
/// - Forwards (clockwise) traversal of entire list from any position
///     - Find destination cup
///     - Build final return value
///
/// Represent as `Vec` of links indexed by cup label (-1 to account for 0 based index):
/// - Note: for part 2, this will only work on 32-bit or greater machines due to
///   requiring `usize` to hold up to `1000000` for indexing.
///
/// ```
/// // 389125467
/// //
/// //  3 8 9
/// // 7     1
/// // 6     2
/// //   5 4
/// let mut cups = [2, 4, 8, 5, 6, 7, 3, 9, 1];
/// let next_cup = |cups: &[usize], label: usize| cups[label-1];
///
/// // Cup 1 is linked to cup 2
/// assert_eq!(next_cup(&cups, 1), 2);
/// // Cup 2 is linked to cup 4
/// assert_eq!(next_cup(&cups, 2), 4);
/// // Cup 8 is linked to cup 9
/// assert_eq!(next_cup(&cups, 8), 9);
/// // Cup 9 is linked to cup 1
/// assert_eq!(next_cup(&cups, 9), 1);
///
/// // Moving cup `1` to be immediately after cup `8`:
/// //
/// //  3 8 1
/// // 7     9
/// // 6     2
/// //   5 4
/// let move_cup = |cups: &mut[usize], src_label: usize, dest_label: usize| {
///		let src_next = next_cup(cups, src_label);
///		let dest_next = next_cup(cups, dest_label);
///		cups[dest_label-1] = src_label;
///		cups[src_label-1] = dest_next;
///		cups[dest_next-1] = src_next;
/// };
///
/// move_cup(&mut cups, 1, 8);
/// // Cup 8 is linked to cup 1
/// assert_eq!(next_cup(&cups, 8), 1);
/// // Cup 1 is linked to cup 9
/// assert_eq!(next_cup(&cups, 1), 9);
/// // Cup 9 is linked to cup 2
/// assert_eq!(next_cup(&cups, 9), 2);
/// // Cup 2 and cup 3 links are unchanged (rest of circle is intact)
/// assert_eq!(next_cup(&cups, 2), 4);
/// assert_eq!(next_cup(&cups, 3), 8);
///
/// // Moving consecutive cups 3, 8, 1 to immediately after cup 4
/// //
/// //  5 6 7
/// // 1     9
/// // 8     2
/// //   3 4
/// let move3 = |cups: &mut[usize], src: usize, dest: usize| {
///   let c3 = next_cup(&cups, next_cup(&cups, src));
///		let dest_next = next_cup(&cups, dest);
///	  let c3_next = next_cup(&cups, c3);
///	  cups[dest-1] = src;
///	  cups[c3-1] = dest_next;
///   cups[dest_next + 1] = c3_next;
/// };
///
/// move3(&mut cups, 3, 4);
/// assert_eq!(next_cup(&cups, 4), 3);
/// assert_eq!(next_cup(&cups, 3), 8);
/// assert_eq!(next_cup(&cups, 8), 1);
/// assert_eq!(next_cup(&cups, 1), 5);
/// assert_eq!(next_cup(&cups, 5), 6);
/// assert_eq!(next_cup(&cups, 6), 7);
/// assert_eq!(next_cup(&cups, 7), 9);
/// assert_eq!(next_cup(&cups, 9), 2);
/// assert_eq!(next_cup(&cups, 2), 4);
/// ```
/// No tricks/patterns, just simulation using the described circular list
/// representation.
fn part_1(cup_labels: &[usize]) -> String {
    let max_cup = cup_labels.iter().max().unwrap().to_owned() as usize;
    let starting_cup = cup_labels[0] as usize;
    let mut cups = build_circular_list(cup_labels);

    play(&mut cups, starting_cup, max_cup, 100);

    let mut traversal = vec![];
    let mut curr = cups[0];
    while curr != 1 {
        traversal.push(curr);
        curr = cups[curr - 1];
    }

    traversal
        .iter()
        .map(|label| format!("{}", label))
        .collect::<Vec<_>>()
        .join("")
}

/// Unable to find an exploitable pattern in the cup movements. Let's try the brute force
/// full simulation approach of `part_1` just on a *much* larger scale...
///
/// - Extend `cups` to `1000000` cups and run `part_1` algorithm  `10000000` times.
fn part_2(cup_labels: &[usize]) -> u64 {
    let max_cup = 1000000;
    let starting_cup = cup_labels[0];
    let mut cups = build_circular_list(
        &cup_labels
            .iter()
            .copied()
            .chain((cup_labels.iter().max().unwrap() + 1)..=max_cup)
            .collect::<Vec<_>>(),
    );

    play(&mut cups, starting_cup, max_cup, 10000000);

    let c1 = next_cup(&cups, 1);
    let c2 = next_cup(&cups, c1);
    c1 as u64 * c2 as u64
}

/// Build a circular list of cups represented as a as `Vec` of "links" indexed by cup
/// label (-1 to account for 0 based index):
/// See `part_1` docstring for more details.
fn build_circular_list(cup_labels: &[usize]) -> Vec<usize> {
    let n = cup_labels.len();
    let mut cups = vec![0; n];
    for (i, cup) in cup_labels[..n - 1].iter().enumerate() {
        cups[cup - 1] = cup_labels[i + 1];
    }
    cups[cup_labels[n - 2]] = cup_labels[0];

    cups
}

fn play(cups: &mut [usize], starting_cup: usize, max_cup: usize, rounds: usize) {
    let mut current_cup = starting_cup;
    for _ in 0..rounds {
        // "Take" 3 cups
        let c1 = next_cup(&cups, current_cup);
        let c2 = next_cup(&cups, c1);
        let c3 = next_cup(&cups, c2);
        let c3_next = next_cup(&cups, c3);

        let dest_cup = {
            let mut dest = current_cup;
            while dest == current_cup || dest == c1 || dest == c2 || dest == c3 {
                dest -= 1;
                if dest == 0 {
                    dest = max_cup;
                }
            }
            dest
        };

        // "Remove" the 3 cups
        cups[current_cup - 1] = c3_next;

        // "Insert" removed cups *after* the destination cup
        let old_dest = cups[dest_cup - 1];
        cups[dest_cup - 1] = c1;
        cups[c3 - 1] = old_dest;

        current_cup = c3_next;
    }
}

#[inline]
fn next_cup(cups: &[usize], label: usize) -> usize {
    cups[label - 1]
}

/// Note: Using `usize` to make the circular list representation easier to build and
/// work with. For part 2, this will only work on 32-bit or greater machines due to
/// requiring `usize` to hold up to `1000000` for indexing.
fn parse_input(input: &str) -> Result<Vec<usize>, ParseError> {
    input
        .lines()
        .next()
        .ok_or(ParseError::InvalidInput)?
        .chars()
        .map(|d| {
            d.to_digit(10)
                .map(|x| x as usize)
                .ok_or(ParseError::InvalidLabel(d))
        })
        .collect()
}

pub fn run(input: &str) {
    let cup_labels = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(&cup_labels));
    println!("Part 2: {}", part_2(&cup_labels));
}

#[derive(PartialEq, Eq, Debug)]
enum ParseError {
    InvalidInput,
    InvalidLabel(char),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "389125467";

    #[test]
    fn parse_input_example() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT).unwrap(),
            [3, 8, 9, 1, 2, 5, 4, 6, 7]
        );
    }

    #[test]
    fn build_cups_example() {
        let cup_labels = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(
            build_circular_list(
                &cup_labels
                    .iter()
                    .map(|x| *x as usize)
                    .collect::<Vec<usize>>()
            ),
            [2, 5, 8, 6, 4, 7, 3, 9, 1]
        );
    }

    #[test]
    fn part_1_example() {
        let cup_labels = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(&cup_labels), "67384529");
    }

    #[test]
    fn part_2_example() {
        let cup_labels = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_2(&cup_labels), 149245887792);
    }
}
