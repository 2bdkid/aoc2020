use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;
use std::collections::{BTreeSet, HashSet};
use std::iter::FromIterator;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[aoc(day1, part1, BruteForce)]
pub fn solve_part1(input: &[u64]) -> Option<u64> {
    input
        .iter()
        .cartesian_product(input)
        .filter_map(|(x, y)| if x + y == 2020 { Some(x * y) } else { None })
        .nth(0)
}

#[aoc(day1, part1, HashSet)]
pub fn solve_part1_hashset(input: &[u64]) -> Option<u64> {
    let input_set: HashSet<&u64> = HashSet::from_iter(input);
    input
        .iter()
        .filter_map(|&x| {
            let y = 2020 - x;
            input_set.get(&y).map(|&y| x * y)
        })
        .nth(0)
}

#[aoc(day1, part1, BTreeSet)]
pub fn solve_part1_btreeset(input: &[u64]) -> Option<u64> {
    let input_set: BTreeSet<&u64> = BTreeSet::from_iter(input);
    input
        .iter()
        .filter_map(|&x| {
            let y = 2020 - x;
            input_set.get(&y).map(|&y| x * y)
        })
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
            input_set.get(&z).map(|&z| z * x * y)
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
            input_set.get(&z).map(|&&z| z * x * y)
        })
        .nth(0)
}

#[aoc(day1, part2, THREESUM)]
pub fn solve_part2_3sum(input: &[u64]) -> Option<u64> {
    let mut input = input.to_vec();
    input.sort();
    let n = input.len();

    for i in 0..n-2 {
        let a = input[i];
        let mut start = i;
        let mut end = n - 1;

        while start <= end {
            let b = input[start];
            let c = input[end];
            if a + b + c == 2020 {
                return Some(a * b * c);
            }
            if a + b + c < 2020 {
                start += 1;
            } else {
                end -= 1;
            }
        }
    }

    None
}
