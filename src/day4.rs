use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::HashMap;

pub struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

impl Passport {
    fn required_fields_present(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid(&self) -> bool {
        self.byr
            .as_ref()
            .map(|byr| {
                byr.parse::<u32>()
                    .map(|byr| byr >= 1920 && byr <= 2002)
                    .unwrap_or(false)
            })
            .unwrap_or(false)
            && self
                .iyr
                .as_ref()
                .map(|iyr| {
                    iyr.parse::<u32>()
                        .map(|iyr| iyr >= 2010 && iyr <= 2020)
                        .unwrap_or(false)
                })
                .unwrap_or(false)
            && self
                .eyr
                .as_ref()
                .map(|eyr| {
                    eyr.parse::<u32>()
                        .map(|eyr| eyr >= 2020 && eyr <= 2030)
                        .unwrap_or(false)
                })
                .unwrap_or(false)
            && self
                .hgt
                .as_ref()
                .map(|hgt| {
                    hgt.ends_with("cm")
                        && hgt
                            .trim_end_matches("cm")
                            .parse::<u32>()
                            .map(|n| n >= 150 && n <= 193)
                            .unwrap_or(false)
                        || hgt.ends_with("in")
                            && hgt
                                .trim_end_matches("in")
                                .parse::<u32>()
                                .map(|n| n >= 59 && n <= 76)
                                .unwrap_or(false)
                })
                .unwrap_or(false)
            && self
                .hcl
                .as_ref()
                .map(|hcl| {
                    hcl.chars().nth(0).map(|c| c == '#').unwrap_or(false)
                        && hcl.chars().count() == 7
                        && u32::from_str_radix(hcl.trim_start_matches("#"), 16).is_ok()
                })
                .unwrap_or(false)
            && self
                .ecl
                .as_ref()
                .map(|ecl| {
                    ecl == "amb"
                        || ecl == "blu"
                        || ecl == "brn"
                        || ecl == "gry"
                        || ecl == "grn"
                        || ecl == "hzl"
                        || ecl == "oth"
                })
                .unwrap_or(false)
            && self
                .pid
                .as_ref()
                .map(|pid| {
                    pid.chars().count() == 9
                        && pid.chars().filter(|c| char::is_digit(*c, 10)).count() == 9
                })
                .unwrap_or(false)
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|passport| {
            passport
                .split_whitespace()
                .map(|field| {
                    let mut kv = field.split(':');
                    (kv.next().unwrap().to_owned(), kv.next().unwrap().to_owned())
                })
                .collect()
        })
        .map(|mut fields: HashMap<String, String>| Passport {
            byr: fields.remove("byr"),
            iyr: fields.remove("iyr"),
            eyr: fields.remove("eyr"),
            hgt: fields.remove("hgt"),
            hcl: fields.remove("hcl"),
            ecl: fields.remove("ecl"),
            pid: fields.remove("pid"),
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Passport]) -> usize {
    input
        .iter()
        .filter(|passport| passport.required_fields_present())
        .count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Passport]) -> usize {
    input.iter().filter(|password| password.is_valid()).count()
}
