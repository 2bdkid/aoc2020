use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|l| String::from(l)).collect()
}

pub fn count_specific_slope(input: &[String], dx: usize, dy: usize) -> usize {
    input
        .iter()
        .enumerate()
        .filter_map(|(i, line)| {
            if i % dy == 0 {
                Some(line)
            } else {
                None
            }
        })
        .enumerate()
        .filter(|(y, line)| line.chars().nth((y * dx) % line.len()).unwrap() == '#')
        .count()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[String]) -> usize {
    count_specific_slope(&input, 3, 1)
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[String]) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(dx, dy)| count_specific_slope(&input, *dx, *dy))
        .product()
}
