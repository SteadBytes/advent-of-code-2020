use crate::str::split_once;
use std::collections::HashMap;
/// Key points:
/// - Each allergen is found in **exactly 1** ingredient
/// - Each ingredient contains **0 or more** allergens
/// - Allergens are **not** always marked
/// - When allergens *are* marked the source ingredient will be present in the corresponding
///   ingredient list.
/// - When not marked, an ingredient containing that allergen *may still be present*.
///
/// Observation: An ingredient *can't possibly* contain an allergen if for all allergens, the number of times
/// the allergen was listed in the input is greater than the number of times the allergen apeared
/// in the allergens list of an ingredients list containing that ingredient.
/// - e.g. all allergens appear more times than could possibly be satisfied by a given ingredient.
///
/// Using `EXAMPLE_INPUT`, we have the following *total* number of times listed for each allergen:
/// ```text
/// dairy = 2
/// fish = 2
/// soy = 1
/// ```
///
/// The following number of times an allergen is listed against an ingredient:
/// ```text
/// mxmxvkd -> dairy = 2
/// mxmxvkd -> fish = 2
/// mxmxvkd -> soy = 0
///
/// kfcds -> dairy = 1
/// kfcds -> fish = 1
/// kfcds -> soy = 0
///
/// sqjhc -> dairy = 1
/// sqjhc -> fish = 2
/// sqjhc -> soy = 1
///
/// nhms -> dairy = 1
/// nhms -> fish = 1
/// nhms -> soy = 0
///
/// trh -> dairy = 1
/// trh -> fish = 0
/// trh -> soy = 0
///
/// fvjkl -> dairy = 1
/// fvjkl -> fish = 0
/// fvjkl -> soy = 1
///
/// sbzzf -> dairy = 1
/// sbzzf -> fish = 1
/// sbzzf -> soy = 0
/// ```
///
/// Out of these, `kfcds`, `nhms`, `trh` and `sbzzf` all meet the criteria for containing no
/// allergens.
///
/// Need:
/// - Counts for each allergen, per ingredient
/// - Total counts for each allergen
/// - Total counts for each ingredient
///     - Calculate final answer (number of times non-allergenic ingredients are listed)
fn part_1(freqs: &Frequencies) -> usize {
    let non_allergenic = freqs.allergen_by_ingredient.iter().filter(|(_, m)| {
        m.iter()
            .all(|(a, i)| freqs.allergen_totals.get(a).unwrap() > i)
    });

    non_allergenic
        .filter_map(|(i, _)| freqs.ingredient_totals.get(i))
        .sum()
}

fn part_2() {
    todo!()
}

#[derive(PartialEq, Eq, Debug)]
struct Frequencies<'a> {
    allergen_by_ingredient: HashMap<&'a str, HashMap<&'a str, usize>>,
    allergen_totals: HashMap<&'a str, usize>,
    ingredient_totals: HashMap<&'a str, usize>,
}

fn parse_input(input: &str) -> Result<Frequencies, ParseError> {
    let mut allergen_freqs_by_ingredient: HashMap<&str, HashMap<&str, usize>> = HashMap::new();
    let mut allergen_total_freqs: HashMap<&str, usize> = HashMap::new();
    let mut ingredient_total_freqs: HashMap<&str, usize> = HashMap::new();

    for l in input.lines() {
        let (ingredients_part, allergens_part) = split_once(l, "(contains ").expect("FIXME");
        let ingredients = ingredients_part.split(" ").collect::<Vec<_>>();
        let allergens = allergens_part
            .strip_suffix(")")
            .unwrap()
            .split(", ")
            .collect::<Vec<_>>();
        for a in &allergens {
            *allergen_total_freqs.entry(a).or_insert(0) += 1;
        }
        for i in ingredients {
            *ingredient_total_freqs.entry(i).or_insert(0) += 1;
            let m = allergen_freqs_by_ingredient
                .entry(i)
                .or_insert(HashMap::new());
            for a in &allergens {
                *m.entry(a).or_insert(0) += 1;
            }
        }
    }

    Ok(Frequencies {
        allergen_by_ingredient: allergen_freqs_by_ingredient,
        allergen_totals: allergen_total_freqs,
        ingredient_totals: ingredient_total_freqs,
    })
}

pub fn run(input: &str) {
    let parsed = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(&parsed));
    // println!("Part 2: {}", part_2(&parsed));
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn parse_input_example() {}

    #[test]
    fn part_1_example() {
        let parsed = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_1(&parsed), 5);
    }

    #[test]
    fn part_2_example() {
        // let parsed = parse_input(EXAMPLE_INPUT).unwrap();
        // assert_eq!(part_2(&parsed),);
    }
}
