use crate::str::split_once;
// TODO: Choose numeric types more thoughtfully?
const PUBKEY_SUBJECT_NUMBER: u64 = 7;

/// Key points:
/// - Modular arithmetic w/ prime numbers
/// - Subject number transform:
///     - Start with `value=1`
///     - Repeat `loop_size` times:
///         - Set `value = value * subject_number`
///         - Set `value = value % 20201227`
///             - 20201227 is **prime**
/// - Cryptographic handshake:
///     - Card pubkey = subject number transform on `7` using it's loop size.
///     - Door pubkey = subject number transform on `7` using it's loop size.
///         - `7` is **prime**
///     - Card/door transfer pubkeys
///     - Encryption key is computed by **both** parties (the following produce the *same* result):
///         - Subject number transform on door pubkey using card loop size.
///         - Subject number transform on card pubkey using door loop size.
/// - Public keys are *known* (puzzle inputs)
/// - Loop sizes are *unknown*
/// - Encrytion key is *unknown*
/// - Main challenge is determining **loop sizes**
///     - Finding the encryption key is just plugging pubkey & loop size into subject number
///       transform after that.
///
/// ## Determining loop sizes
///
/// Since both the public keys and subject number for calculating the public keys are known, the
/// loop size is the only unkown in that calculation.
///
/// Simple linear search:
/// - Repeatedly perform the "inner" part of the subjec transform until the desired result (pubkey)
///   is produced (keeping track of how many repetitions were performed).
/// - Fairly certain this will work absolutely fine as the prime numbers here aren't huge.
fn part_1((card_pubkey, door_pubkey): (u64, u64)) -> u64 {
    let encryption_key = subject_transform(door_pubkey, find_loop_size(card_pubkey));
    // Sanity check in non-release build
    debug_assert_eq!(
        encryption_key,
        subject_transform(card_pubkey, find_loop_size(door_pubkey))
    );
    encryption_key
}

fn find_loop_size(pubkey: u64) -> u64 {
    let mut n = 0;
    let mut value = 1;
    while value != pubkey {
        value = transform_inner(PUBKEY_SUBJECT_NUMBER, value);
        n += 1
    }
    n
}

fn subject_transform(subject: u64, loop_size: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value = (value * subject) % 20201227;
    }
    value
}

fn transform_inner(subject: u64, value: u64) -> u64 {
    (value * subject) % 20201227
}

fn part_2() {
    todo!()
}

// TODO: Is `u64` sufficient?
fn parse_input(input: &str) -> Result<(u64, u64), ParseError> {
    split_once(input, "\n")
        .ok_or(ParseError::InvalidInput)
        .and_then(|(s1, s2)| {
            Ok((
                s1.trim()
                    .parse()
                    .map_err(|e| ParseError::InvalidPubkey(e))?,
                s2.trim()
                    .parse()
                    .map_err(|e| ParseError::InvalidPubkey(e))?,
            ))
        })
}

pub fn run(input: &str) {
    let pubkeys = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(pubkeys));
    // println!("Part 2: {}", part_2(&parsed));
}

#[derive(PartialEq, Eq, Debug)]
enum ParseError {
    InvalidInput,
    InvalidPubkey(std::num::ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
5764801
17807724";

    #[test]
    fn parse_input_example() {
        assert_eq!(parse_input(EXAMPLE_INPUT).unwrap(), (5764801, 17807724));
    }

    #[test]
    fn part_1_example() {
        let pubkeys = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(pubkeys), 14897079);
    }

    #[test]
    fn subject_transform_examples() {
        assert_eq!(subject_transform(PUBKEY_SUBJECT_NUMBER, 8), 5764801);
        assert_eq!(subject_transform(PUBKEY_SUBJECT_NUMBER, 11), 17807724);
        assert_eq!(subject_transform(17807724, 8), 14897079);
        assert_eq!(subject_transform(5764801, 11), 14897079);
    }

    #[test]
    fn find_loop_size_examples() {
        assert_eq!(find_loop_size(5764801), 8);
        assert_eq!(find_loop_size(17807724), 11);
    }
}
