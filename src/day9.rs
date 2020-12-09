use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::HashSet;
use itertools::Itertools;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.lines().map(|x| x.parse::<u64>().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[u64]) -> Option<u64> {
    let valid = |prev: &[u64], n: u64| {
        let prev: HashSet<u64> = prev.iter().copied().collect();
        prev.iter().any(|x| {
            let target = n - *x;
            if target == *x {
                false
            } else {
                prev.contains(&target)
            }
        })
    };

    input.windows(26).find_map(|w| {
        let prev = &w[..25];
        let n = w[25];
        if !valid(prev, n) {
            Some(n)
        } else {
            None
        }
    })
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[u64]) -> Option<u64> {
    let invalid_number = solve_part1(input).unwrap();

    for i in 1..input.len() - 1 {
        for j in i + 1..input.len() {
            let region = &input[i..j];
            if region.iter().sum::<u64>() == invalid_number {
                let (min, max)= region.iter().minmax().into_option().unwrap();
                return Some(min + max);
            }
        }
    }

    None
}

#[aoc(day9, part2, CATERPILLAR)]
pub fn solve_part2_caterpillar(input: &[u64]) -> Option<u64> {
    let invalid_number = solve_part1(input).unwrap();

    let mut a = 0;
    let mut b = 1;
    let mut sum = input[0] + input[1];

    while sum != invalid_number {
        if sum < invalid_number {
            b += 1;
            sum += input[b];
        } else {
            sum -= input[a];
            a += 1
        }
    }

    input[a..b + 1].iter().minmax().into_option().map(|(min, max)| min + max)
}
