use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::HashSet;
use std::iter::FromIterator;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .chars()
        .map(|c| if c == 'F' || c == 'L' { '0' } else if c == 'B' || c == 'R' { '1' } else { c })
        .collect::<String>()
        .lines()
        .map(|n| u32::from_str_radix(n, 2).unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[u32]) -> Option<u32> {
    input.iter().max().cloned()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[u32]) -> Option<u32> {
    let ids: HashSet<u32> = HashSet::from_iter(input.iter().cloned());
    (1..1023).find(|id| !ids.contains(id) && ids.contains(&(id - 1)) && ids.contains(&(id + 1)))
}
