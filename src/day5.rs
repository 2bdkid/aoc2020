use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::HashSet;
use std::iter::FromIterator;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|boarding_pass| {
            u16::from_str_radix(
                &boarding_pass
                    .replace("F", "0")
                    .replace("B", "1")
                    .replace("L", "0")
                    .replace("R", "1"),
                2,
            )
            .unwrap()
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[u16]) -> Option<u16> {
    input.iter().max().cloned()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[u16]) -> Option<u16> {
    let ids: HashSet<u16> = HashSet::from_iter(input.iter().cloned());
    (0..1024).find(|id| !ids.contains(id) && ids.contains(&(id - 1)) && ids.contains(&(id + 1)))
}
