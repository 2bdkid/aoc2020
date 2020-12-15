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
    let mut first: HashMap<u32, u32> = input
        .iter()
        .copied()
        .enumerate()
        .map(|(i, x)| (x, i as u32 + 1))
        .collect();
    let mut second: HashMap<u32, u32> = HashMap::new();
    let mut prev_spoken = *input.last().unwrap();

    while turn <= n {
        if first.contains_key(&prev_spoken) && !second.contains_key(&prev_spoken) {
            if let Some(t) = first.get(&0) {
                second.insert(0, *t);
            }
            first.insert(0, turn);
            prev_spoken = 0;
        } else {
            let t1 = *first.get(&prev_spoken).unwrap();
            let t2 = *second.get(&prev_spoken).unwrap();
            prev_spoken = t1 - t2;
            if let Some(t) = first.get(&prev_spoken) {
                second.insert(prev_spoken, *t);
            }
            first.insert(prev_spoken, turn);
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
