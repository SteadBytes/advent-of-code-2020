#![allow(dead_code)]

const REQUIRED_FIELDS: [&str; 7] = ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];

fn part_1(input: &str) -> usize {
    input
        .split("\n\n") // passports
        .map(|p| {
            p.lines()
                .flat_map(|l| l.split(" ")) // key:value pairs
                .map(|kv| kv.split(":").nth(0).expect("unable to parse k:v pair")) // keys
                .filter(|k| REQUIRED_FIELDS.contains(k))
        })
        .map(|ks| ks.count())
        .filter(|&x| x == REQUIRED_FIELDS.len())
        .count()
}

fn part_2() {}

fn parse_input(input: &str) {}

pub fn run(input: &str) {
    /* let parsed = parse_input(input).expect("unable to parse input"); */
    println!("Part 1: {}", part_1(&input));
    /* println!("Part 2: {}", part_2(&parsed)); */
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";

    #[test]
    fn parse_input_example() {}

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT), 2);
    }

    #[test]
    fn part_2_example() {}
}
