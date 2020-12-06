use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use itertools::Itertools;
use std::collections::HashSet;

pub type GroupAnswers = Vec<String>;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<GroupAnswers> {
    input
        .split("\n\n")
        .map(|group| group.lines().map(|l| l.to_owned()).collect())
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[GroupAnswers]) -> usize {
    input
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|person| person.chars())
                .flatten()
                .collect::<HashSet<char>>()
                .len()
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[GroupAnswers]) -> usize {
    input
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|person| person.chars().collect::<HashSet<char>>())
                .fold1(|acc, g| g.intersection(&acc).copied().collect())
                .unwrap()
                .len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
        let input = input_generator(&input);
        assert_eq!(solve_part2(&input), 6);
    }
}
