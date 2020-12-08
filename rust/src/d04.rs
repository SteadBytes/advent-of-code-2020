use std::collections::HashMap;
use std::str::FromStr;
use PassportField::*;

const REQUIRED_FIELDS: [PassportField; 7] = [Ecl, Pid, Eyr, Hcl, Byr, Iyr, Hgt];

pub fn run(input: &str) {
    let passports = parse_input(input).expect("unable to parse input");
    println!("Part 1: {}", part_1(&passports));
    println!("Part 2: {}", part_2(&passports));
}

fn part_1<'a>(passports: &Vec<HashMap<PassportField, &'a str>>) -> usize {
    passports
        .iter()
        .filter(|p| REQUIRED_FIELDS.iter().all(|f| p.contains_key(f)))
        .count()
}

fn part_2<'a>(passports: &Vec<HashMap<PassportField, &'a str>>) -> usize {
    passports
        .iter()
        .filter(|p| {
            REQUIRED_FIELDS
                .iter()
                .all(|f| p.get(f).and_then(|v| Some(f.is_valid(v))).unwrap_or(false))
        })
        .count()
}

fn parse_input<'a>(input: &'a str) -> Result<Vec<HashMap<PassportField, &'a str>>, ParseError> {
    input
        .split("\n\n") // passports
        .map(|p| {
            p.lines()
                .flat_map(|l| l.split(" ")) // key:value pairs
                .map(|kv| {
                    let mut kv = kv.split(":");
                    let k = kv.next().ok_or(ParseError::InvalidKV)?;
                    let v = kv.next().ok_or(ParseError::InvalidKV)?;
                    Ok((PassportField::from_str(k)?, v))
                })
                .collect()
        })
        .collect()
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
enum ParseError {
    InvalidKV,
    InvalidKey,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
enum PassportField {
    Byr,
    Iyr,
    Eyr,
    Hgt,
    Hcl,
    Ecl,
    Pid,
    Cid,
}

impl FromStr for PassportField {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "byr" => Ok(Byr),
            "iyr" => Ok(Iyr),
            "eyr" => Ok(Eyr),
            "hgt" => Ok(Hgt),
            "hcl" => Ok(Hcl),
            "ecl" => Ok(Ecl),
            "pid" => Ok(Pid),
            "cid" => Ok(Cid),
            _ => Err(ParseError::InvalidKey),
        }
    }
}

impl PassportField {
    fn is_valid(&self, v: &str) -> bool {
        match self {
            Byr => parses_in_range::<i32>(v, 1920, 2002),
            Iyr => parses_in_range::<i32>(v, 2010, 2020),
            Eyr => parses_in_range::<i32>(v, 2020, 2030),
            Hgt => {
                // e.g. "150cm", "59in"
                let (height, unit) = v.split_at(v.len() - 2);
                match unit {
                    "cm" => parses_in_range(height, 150, 193),
                    "in" => parses_in_range(height, 59, 76),
                    _ => false,
                }
            }
            Hcl => &v[..1] == "#" && v[1..].chars().all(|c| c.is_ascii_hexdigit()),
            Ecl => match v {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            },
            Pid => v.len() == 9 && v.parse::<i32>().is_ok(),
            Cid => true, // Optional - no restrictions specified
        }
    }
}

fn parses_in_range<T: FromStr + PartialOrd>(s: &str, min: T, max: T) -> bool {
    s.parse::<T>()
        .ok()
        .map(|x| min <= x && x <= max)
        .unwrap_or(false)
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
    fn parse_input_example() {
        let parsed = parse_input(&EXAMPLE_INPUT).unwrap();
        assert_eq!(
            parsed,
            vec![
                vec![
                    (Ecl, "gry"),
                    (Pid, "860033327"),
                    (Eyr, "2020"),
                    (Hcl, "#fffffd"),
                    (Byr, "1937"),
                    (Iyr, "2017"),
                    (Cid, "147"),
                    (Hgt, "183cm")
                ],
                vec![
                    (Iyr, "2013"),
                    (Ecl, "amb"),
                    (Cid, "350"),
                    (Eyr, "2023"),
                    (Pid, "028048884"),
                    (Hcl, "#cfa07d"),
                    (Byr, "1929")
                ],
                vec![
                    (Hcl, "#ae17e1"),
                    (Iyr, "2013"),
                    (Eyr, "2024"),
                    (Ecl, "brn"),
                    (Pid, "760753108"),
                    (Byr, "1931"),
                    (Hgt, "179cm")
                ],
                vec![
                    (Hcl, "#cfa07d"),
                    (Eyr, "2025"),
                    (Pid, "166559648"),
                    (Iyr, "2011"),
                    (Ecl, "brn"),
                    (Hgt, "59in")
                ]
            ]
            .into_iter()
            .map(|kvs| kvs.into_iter().collect::<HashMap<PassportField, &str>>())
            .collect::<Vec<HashMap<PassportField, &str>>>()
        );
    }

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(&parse_input(EXAMPLE_INPUT).unwrap()), 2);
    }

    #[test]
    fn part_2_example() {
        let all_invalid = "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

        let all_valid = "\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        assert_eq!(part_2(&parse_input(all_invalid).unwrap()), 0);
        assert_eq!(part_2(&parse_input(all_valid).unwrap()), 4);
        assert_eq!(
            part_2(&parse_input(&format!("{}\n\n{}", all_valid, all_invalid)).unwrap()),
            4
        );
    }
}
