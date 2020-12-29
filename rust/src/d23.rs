use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

type Cups = HashMap<u32, Rc<RefCell<ListNode>>>;

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
/// Ownership:
/// - No cup is permanently removed from the circular list during the game
///     - "Picking up" the 3 cups from the circular list is temporary as they are placed *back* into the list in
///        a different position
///     - Not **dropped**
/// - Something *other* than the list itself needs to own items in the list, with the list itself
///   maintaining references between them to form the circular list.
///
/// Idea: `HashMap` + Linked List (*rustaceans gasp*)
/// - Circular LL to represent circular list of cups
///     - e.g. last node points to first node
/// - `HashMap` from cup label to LL node
///     - **Owns** the LL nodes
/// - Can retrieve a node by cup label via the `HashMap`
/// - Remove and insert nodes in the circular list via LL
/// - Traversal from any position via LL
/// - Calculate final result as `HashMap` lookup for cup `1` node then LL traversal until `1` is reached
///   again.
/// - LL node next pointers:
///     - As nodes can be removed from the LL but *not* dropped, their next pointers may not be
///       present at any given time -> use `Option`.
/// Simple simulation
fn part_1(cup_labels: &[u32]) -> String {
    let (cups, starting_cup) = build_cups(cup_labels.iter().copied()); // FIXME
    let mut current_cup = Rc::clone(cups.get(&starting_cup).unwrap());
    for _ in 0..100 {
        // "Remove" 3 cups
        let c1 = current_cup.borrow().next_node();
        let c2 = c1.borrow().next_node();
        let c3 = c2.borrow().next_node();

        // Choose destination cup
        // Since nodes are stored in HashMap by label, we don't actually need to traverse
        // the LL. Instead, decrement the destination label value so long as it is equal to either
        // the current cup label or any of the 3 removed cup labels. Wrap around to the maximum
        // label value if 0 is hit.
        let destination_cup = {
            let max_label = cups.keys().max().unwrap();
            let mut destination_label = current_cup.borrow().label;
            while destination_label == current_cup.borrow().label
                || destination_label == c1.borrow().label
                || destination_label == c2.borrow().label
                || destination_label == c3.borrow().label
            {
                destination_label -= 1;
                if destination_label == 0 {
                    destination_label = *max_label;
                }
            }
            Rc::clone(cups.get(&destination_label).unwrap())
        };

        // "Insert" removed cups after destination cup
        // Update current cup next pointer to third removed cup next pointer
        current_cup.borrow_mut().next = Some(Weak::clone(&c3.borrow().next.as_ref().unwrap()));
        // Update destination cup and third removed cup next pointers
        c3.borrow_mut().next = Some(Weak::clone(
            &destination_cup.borrow().next.as_ref().unwrap(),
        ));
        destination_cup.borrow_mut().next = Some(Rc::downgrade(&c1));
        // Select new current cup
        let next = current_cup.borrow().next_node();
        current_cup = next;
    }

    let traversal = cups_after_1(&cups);
    // FIXME: There's definitely a better way to do this
    traversal
        .iter()
        .map(|label| format!("{}", label))
        .collect::<Vec<_>>()
        .join("")
}

fn cups_after_1(cups: &Cups) -> Vec<u32> {
    let mut traversal = vec![];
    let mut current = cups.get(&1).unwrap().borrow().next_node();
    loop {
        traversal.push(current.borrow().label);
        let next = current.borrow().next_node();
        if next.borrow().label == 1 {
            break;
        }
        current = Rc::clone(&next);
    }
    traversal
}

/// Unable to find an exploitable pattern in the cup movements. Let's try the brute force
/// full simulation approach of `part_1` just on a *much* larger scale...
///
/// Brute force: Extend `cups` to `1000000` cups and run `part_1` algorithm  `10000000` times.
fn part_2(cup_labels: &[u32]) -> u64 {
    let (cups, starting_cup) = build_cups(
        cup_labels
            .iter()
            .copied()
            .chain((cup_labels.iter().max().unwrap() + 1)..=1000000),
    );
    let max_label = 1000000;
    let mut current_cup = Rc::clone(cups.get(&starting_cup).unwrap());
    for _ in 0..10000000 {
        // "Remove" 3 cups
        let c1 = current_cup.borrow().next_node();
        let c2 = c1.borrow().next_node();
        let c3 = c2.borrow().next_node();

        // Choose destination cup
        // Since nodes are stored in HashMap by label, we don't actually need to traverse
        // the LL. Instead, decrement the destination label value so long as it is equal to either
        // the current cup label or any of the 3 removed cup labels. Wrap around to the maximum
        // label value if 0 is hit.
        let destination_cup = {
            let mut destination_label = current_cup.borrow().label;
            while destination_label == current_cup.borrow().label
                || destination_label == c1.borrow().label
                || destination_label == c2.borrow().label
                || destination_label == c3.borrow().label
            {
                destination_label -= 1;
                if destination_label == 0 {
                    destination_label = max_label;
                }
            }
            Rc::clone(cups.get(&destination_label).unwrap())
        };

        // "Insert" removed cups after destination cup
        // Update current cup next pointer to third removed cup next pointer
        current_cup.borrow_mut().next = Some(Weak::clone(&c3.borrow().next.as_ref().unwrap()));
        // Update destination cup and third removed cup next pointers
        c3.borrow_mut().next = Some(Weak::clone(
            &destination_cup.borrow().next.as_ref().unwrap(),
        ));
        destination_cup.borrow_mut().next = Some(Rc::downgrade(&c1));
        // Select new current cup
        let next = current_cup.borrow().next_node();
        current_cup = next;
    }

    let n1 = cups.get(&1).unwrap().borrow().next_node().borrow().label;
    let n2 = cups
        .get(&1)
        .unwrap()
        .borrow()
        .next_node()
        .borrow()
        .next_node()
        .borrow()
        .label;

    n1 as u64 * n2 as u64
}

fn build_cups<T>(cup_labels: T) -> (Cups, u32)
where
    T: Iterator<Item = u32>,
{
    let mut cup_labels = cup_labels.into_iter();
    let mut m = HashMap::new();
    let first_label = cup_labels.next().unwrap();
    // Insert the first cup
    let head = Rc::new(RefCell::new(ListNode {
        label: first_label,
        next: None,
    }));
    m.insert(first_label, head);

    // Build nodes for remaining cups
    let mut prev_node = Rc::clone(m.get(&first_label).unwrap());
    for l in cup_labels {
        let new_node = Rc::new(RefCell::new(ListNode {
            label: l,
            next: None,
        }));
        prev_node.borrow_mut().next = Some(Rc::downgrade(&new_node));
        prev_node = Rc::clone(&new_node);
        m.insert(l, new_node);
    }

    // Link last cup to first cup to ensure circular list
    prev_node.borrow_mut().next = Some(Rc::downgrade(m.get(&first_label).unwrap()));

    (m, first_label)
}

fn parse_input(input: &str) -> Result<Vec<u32>, ParseError> {
    input
        .lines()
        .next()
        .ok_or(ParseError::InvalidInput)?
        .chars()
        .map(|d| d.to_digit(10).ok_or(ParseError::InvalidLabel(d)))
        .collect()
}

pub fn run(input: &str) {
    let cup_labels = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(&cup_labels));
    println!("Part 2: {}", part_2(&cup_labels));
}

#[derive(Debug)]
struct ListNode {
    label: u32, // Part 2 requires `u32` range for labels
    next: Option<Weak<RefCell<ListNode>>>,
}

impl ListNode {
    /// FIXME: This will panic if the list is not circular
    fn next_node(&self) -> Rc<RefCell<ListNode>> {
        self.next.as_ref().unwrap().upgrade().unwrap()
    }
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
        let (cups, starting_cup) = build_cups(cup_labels.iter().copied());

        assert_eq!(
            starting_cup, cup_labels[0],
            "starting cup was not the first cup in the input"
        );

        assert_eq!(
            cups.len(),
            cup_labels.len(),
            "incorrect number of cups parsed from input"
        );

        // Check node values match HashMap keys
        for label in &cup_labels {
            assert_eq!(
                cups.get(label).unwrap().borrow().label,
                *label,
                "node label does not match HashMap key"
            );
        }

        // Check LL traversal
        // Note: This will infinite loop if the list incorrectly never loops back round
        // to the starting node
        let mut traversal = vec![];
        let mut current = Rc::clone(cups.get(&starting_cup).unwrap());
        loop {
            traversal.push(current.borrow().label);
            let next = current.borrow().next_node();
            if next.borrow().label == starting_cup {
                break;
            }
            current = Rc::clone(&next);
        }
        assert_eq!(traversal, cup_labels, "LL traversal order incorrect");
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
