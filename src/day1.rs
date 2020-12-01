use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::iter::FromIterator;

use std::collections::{HashSet, BTreeSet};
use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u64]) -> Option<u64> {
    input
        .iter()
        .cartesian_product(input)
        .filter_map(|(x, y)| if x + y == 2020 { Some(x * y) } else { None })
        .nth(0)
}

#[aoc(day1, part2, BruteForce)]
pub fn solve_part2(input: &[u64]) -> Option<u64> {
    input
        .iter()
        .cartesian_product(input)
        .cartesian_product(input)
        .filter_map(|((x, y), z)| {
            if x + y + z == 2020 {
                Some(x * y * z)
            } else {
                None
            }
        })
        .nth(0)
}

#[aoc(day1, part2, HashSet)]
pub fn solve_part2_hashset(input: &[u64]) -> Option<u64> {
    let input_set: HashSet<&u64> = HashSet::from_iter(input.iter());
    input
        .iter()
        .cartesian_product(input)
        .filter_map(|(x, y)| {
            let z = 2020 - x - y;
            input_set.get(&z).map(|z| *z * x * y)
        })
        .nth(0)
}

#[aoc(day1, part2, BTreeSet)]
pub fn solve_part2_btreeset(input: &[u64]) -> Option<u64> {
    let input_set: BTreeSet<&u64> = BTreeSet::from_iter(input.iter());
    input
        .iter()
        .cartesian_product(input)
        .filter_map(|(x, y)| {
            let z = 2020 - x - y;
            input_set.get(&z).map(|z| *z * x * y)
        })
        .nth(0)
}
