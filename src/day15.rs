use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| l.split(","))
        .flatten()
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}

fn nth_number_spoken(input: &[u32], n: u32) -> u32 {
    let mut turn = input.len() as u32 + 1;
    let mut prev: HashMap<u32, (u32, Option<u32>)> = input
        .iter()
        .copied()
        .enumerate()
        .map(|(i, x)| (x, (i as u32 + 1, None)))
        .collect();
    let mut prev_spoken = *input.last().unwrap();

    while turn <= n {
        if let Some((t, Some(s))) = prev.get(&prev_spoken) {
            let next_spoken = t - s;
            if let Some((t, _)) = prev.get(&next_spoken).copied() {
                prev.insert(next_spoken, (turn, Some(t)));
            } else {
                prev.insert(next_spoken, (turn, None));
            }
            prev_spoken = next_spoken;
        } else {
            if let Some((t, _)) = prev.get(&0).copied() {
                prev.insert(0, (turn, Some(t)));
            } else {
                prev.insert(0, (turn, None));
            }
            prev_spoken = 0;
        }
        turn += 1;
    }

    prev_spoken
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    nth_number_spoken(input, 2020)
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    nth_number_spoken(input, 30000000)
}
