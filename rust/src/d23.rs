use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

type Cups = HashMap<u8, Rc<RefCell<ListNode>>>;

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
fn part_1(mut cups: Cups, starting_cup: u8) -> String {
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

    // FIXME
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
    // FIXME: There's definitely a better way to do this
    traversal
        .iter()
        .map(|label| format!("{}", label))
        .collect::<Vec<_>>()
        .join("")
}

fn part_2() {
    todo!()
}

fn parse_input(input: &str) -> Result<(Cups, u8), ParseError> {
    let cup_labels: Vec<u8> = input
        .lines()
        .next()
        .ok_or(ParseError::InvalidInput)?
        .chars()
        .map(|d| {
            d.to_digit(10)
                // Parsed successfully as base 10 -> safe cast to u8
                .map(|x| x as u8)
                .ok_or(ParseError::InvalidLabel(d))
        })
        .collect::<Result<_, _>>()?;

    let mut m = HashMap::new();
    // Insert the first cup
    let head = Rc::new(RefCell::new(ListNode {
        label: cup_labels[0],
        next: None,
    }));
    m.insert(cup_labels[0], head);

    // Build nodes for remaining cups
    let mut prev_node = Rc::clone(m.get(&cup_labels[0]).unwrap());
    for l in &cup_labels[1..] {
        let new_node = Rc::new(RefCell::new(ListNode {
            label: *l,
            next: None,
        }));
        prev_node.borrow_mut().next = Some(Rc::downgrade(&new_node));
        prev_node = Rc::clone(&new_node);
        m.insert(*l, new_node);
    }

    // Link last cup to first cup to ensure circular list
    m.get(cup_labels.last().unwrap()).unwrap().borrow_mut().next =
        Some(Rc::downgrade(m.get(&cup_labels[0]).unwrap()));

    Ok((m, cup_labels[0]))
}

pub fn run(input: &str) {
    let (cups, starting_cup) = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(cups, starting_cup));
    // println!("Part 2: {}", part_2(&parsed));
}

#[derive(Debug)]
struct ListNode {
    label: u8,
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
        let expected_labels = [3, 8, 9, 1, 2, 5, 4, 6, 7];

        let (cups, starting_cup) = parse_input(EXAMPLE_INPUT).unwrap();

        assert_eq!(
            starting_cup, expected_labels[0],
            "starting cup was not the first cup in the input"
        );

        assert_eq!(
            cups.len(),
            expected_labels.len(),
            "incorrect number of cups parsed from input"
        );

        // Check node values match HashMap keys
        for label in &expected_labels {
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
        assert_eq!(traversal, expected_labels, "LL traversal order incorrect");
    }

    #[test]
    fn part_1_example() {
        let (cups, starting_cup) = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(cups, starting_cup), "67384529");
    }

    // #[test]
    // fn part_2_example() {
    //     let parsed = parse_input(EXAMPLE_INPUT).unwrap();
    //     assert_eq!(part_2(&parsed),);
    // }
}
