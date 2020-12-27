use crate::str::split_once;
use std::collections::{HashSet, VecDeque};

/// Quick and simple simulation
///
/// Returns `u16` as the largest possible score (deck in ascending order) is less than `u16::MAX`.
/// The following would panic due to overflow otherwise:
///
/// ```
/// (0_u16..=50).map(|x| x.pow(2)).sum::<u16>();
/// ```
pub fn part_1(mut p1: Deck, mut p2: Deck) -> u16 {
    loop {
        let p1_top = p1.pop_front().unwrap();
        let p2_top = p2.pop_front().unwrap();

        if p1_top > p2_top {
            p1.push_back(p1_top);
            p1.push_back(p2_top);
        } else {
            p2.push_back(p2_top);
            p2.push_back(p1_top);
        }

        if p1.is_empty() {
            return p2
                .iter()
                .rev()
                .enumerate()
                .map(|(i, &x)| (i + 1) as u16 * x as u16)
                .sum();
        }
        if p2.is_empty() {
            return p1
                .iter()
                .rev()
                .enumerate()
                .map(|(i, &x)| (i + 1) as u16 * x as u16)
                .sum();
        }
    }
}

/// Key points:
/// - Recursive sub-games start with the *remaining* `n` cards in each players deck from the parent
///   game, where `n` is the number on their respective drawn cards for the parent round
///     - Not including the cards drawn in that round
/// - Recurse *only* when both players have sufficient cards to do so
/// - Winner of sub-game becomes winner of the round in the parent game that initiated the sub-game
/// - If a previous round in current game had the exact same deck for each player, that *game*
///   instantly ends with **player 1** as winner
///   - Checked *before* cards are dealt
///   - Prevent inifinite recursion
///
/// Python-esque pseudocode for this recursion:
/// ```text
/// def play_game(p1_deck, p2_deck, prev_states):
///     loop:
///         # Infinite recursion prevention
///         if (p1_deck, p2_deck) in prev_states:
///             # P1 instantly wins game
///             return P1
///         prev_states.add((p1_deck, p2_deck))
///
///         c1, c2 = draw_cards(p1_deck, p2_deck)
///         if can_recurse(p1_deck, p2_deck, c1, c2):
///             winner = play_game(p1_deck[:c1], p2_deck[:c2], {})
///         else if c1 > c2:
///             winner = P1
///         else:
///             winner = P2
///
///         if winner is P1:
///             p1_deck.append(c1)
///             p1_deck.append(c2)
///         else:
///             p2_deck.append(c2)
///             p2_deck.append(c1)
///
///         if len(p1_deck) == 0:
///             return P2
///         if len(p2_deck) == 0:
///             return P1
///
/// def draw_cards(p1_deck, p2_deck):
///     return p1_deck.top(), p2_deck.top()
///
/// def can_recurse(p1_deck, p2_deck, card_1, card_2);
///     return len(p1_deck) >= card_1 and len(p2_deck) >= card_2
///
/// winner = play_game(p1_deck, p2_deck, {})
/// # calculate score for winner as in part_1...
/// ```
///
/// Let's first try implementing this recursion directly and then move to iterative implementation
/// if necessary/desirable...
fn part_2(p1: &mut Deck, p2: &mut Deck) -> u16 {
    let winner = play_game(p1, p2, HashSet::new());
    match winner {
        Players::P2 => p2
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &x)| (i + 1) as u16 * x as u16)
            .sum(),
        Players::P1 => p1
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &x)| (i + 1) as u16 * x as u16)
            .sum(),
    }
}

fn play_game(p1: &mut Deck, p2: &mut Deck, mut prev_states: HashSet<(Deck, Deck)>) -> Players {
    loop {
        let round_state = (p1.clone(), p2.clone());
        if prev_states.contains(&round_state) {
            return Players::P1;
        }
        prev_states.insert(round_state);

        let p1_top = p1.pop_front().unwrap();
        let p2_top = p2.pop_front().unwrap();

        let winner = if p1.len() >= p1_top as usize && p2.len() >= p2_top as usize {
            // Start new sub-game
            play_game(
                &mut p1.iter().take(p1_top as usize).cloned().collect(),
                &mut p2.iter().take(p2_top as usize).cloned().collect(),
                prev_states.clone(),
            )
        } else if p1_top > p2_top {
            Players::P1
        } else {
            Players::P2
        };

        match winner {
            Players::P1 => {
                p1.push_back(p1_top);
                p1.push_back(p2_top);
            }
            Players::P2 => {
                p2.push_back(p2_top);
                p2.push_back(p1_top);
            }
        }

        if p1.is_empty() {
            return Players::P2;
        }
        if p2.is_empty() {
            return Players::P1;
        }
    }
}

fn parse_input(input: &str) -> Result<(Deck, Deck), ParseError> {
    let (p1_part, p2_part) = split_once(input, "\n\n").ok_or(ParseError::InvalidInput)?;
    let p1 = p1_part
        .lines()
        .skip(1) // "Player 1:"
        .map(|l| l.parse::<u8>())
        .collect::<Result<VecDeque<_>, _>>()
        .map_err(|e| ParseError::InvalidCard(e))?;
    let p2 = p2_part
        .lines()
        .skip(1) // "Player 2:"
        .map(|l| l.parse::<u8>())
        .collect::<Result<VecDeque<_>, _>>()
        .map_err(|e| ParseError::InvalidCard(e))?;
    // TODO: More sanity checks? e.g. complete set of cards present (contiguous set from 1..n)
    if p1.len() != p2.len() {
        return Err(ParseError::MissingCards);
    }

    Ok((p1, p2))
}

pub fn run(input: &str) {
    let (p1, p2) = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(p1.clone(), p2.clone()));
    println!("Part 2: {}", part_2(&mut p1.clone(), &mut p2.clone()));
}

type Deck = VecDeque<u8>;

#[derive(PartialEq, Eq, Debug)]
enum Players {
    P1,
    P2,
}

#[derive(PartialEq, Eq, Debug)]
enum ParseError {
    InvalidInput,
    InvalidCard(std::num::ParseIntError),
    MissingCards,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn parse_input_example() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT).unwrap(),
            (
                [9, 2, 6, 3, 1].iter().copied().collect::<VecDeque<_>>(),
                [5, 8, 4, 7, 10].iter().copied().collect::<VecDeque<_>>()
            )
        );
    }

    #[test]
    fn part_1_example() {
        let (p1, p2) = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(p1, p2), 306);
    }

    #[test]
    fn part_2_example() {
        let (mut p1, mut p2) = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_2(&mut p1, &mut p2), 291);
    }
}
