use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::{HashMap, HashSet};

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.lines().map(|l| l.parse::<i64>().unwrap()).collect()
}

fn search_adapter(
    at: i64,
    target: i64,
    mut remaining: HashSet<i64>,
    path: Vec<i64>,
    counts: (i64, i64),
) -> Option<(Vec<i64>, (i64, i64))> {
    if at == target {
        // sanity check
        return if remaining.is_empty() {
            Some((path, counts))
        } else {
            None
        };
    }

    if remaining.remove(&(at + 1)) {
        if let Some((mut path, (ones, threes))) =
            search_adapter(at + 1, target, remaining.clone(), path.clone(), counts)
        {
            path.push(at + 1);
            return Some((path, (ones + 1, threes)));
        }
    }

    if remaining.remove(&(at + 2)) {
        if let Some((mut path, (ones, threes))) =
            search_adapter(at + 2, target, remaining.clone(), path.clone(), counts)
        {
            path.push(at + 2);
            return Some((path, (ones, threes)));
        }
    }

    if remaining.remove(&(at + 3)) {
        if let Some((mut path, (ones, threes))) =
            search_adapter(at + 3, target, remaining.clone(), path.clone(), counts)
        {
            path.push(at + 3);
            return Some((path, (ones, threes + 1)));
        }
    }

    None
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[i64]) -> Option<i64> {
    let target_joltage: i64 = input.iter().max().unwrap() + 3;
    let mut remaining: HashSet<i64> = input.iter().copied().collect();
    remaining.insert(target_joltage);
    search_adapter(0, target_joltage, remaining.clone(), Vec::new(), (0, 0))
        .map(|(_, (ones, threes))| ones * threes)
}

fn count_paths(path: &[i64]) -> Option<i64> {
    let mut dp: HashMap<i64, i64> = HashMap::new();
    dp.insert(0, 1);

    for n in path.iter().rev() {
        dp.insert(
            *n,
            dp.get(&(n - 1)).unwrap_or(&0)
                + dp.get(&(n - 2)).unwrap_or(&0)
                + dp.get(&(n - 3)).unwrap_or(&0),
        );
    }

    dp.get(path.iter().max().unwrap()).copied()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[i64]) -> Option<i64> {
    let target_joltage: i64 = input.iter().max().unwrap() + 3;
    let mut remaining: HashSet<i64> = input.iter().copied().collect();
    remaining.insert(target_joltage);
    let path: Vec<i64> =
        search_adapter(0, target_joltage, remaining.clone(), Vec::new(), (0, 0))
            .map(|(path, _)| path)
            .unwrap();
    count_paths(&path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2_example() {
        let input = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";
        let input = input_generator(input);
        assert_eq!(solve_part2(&input), Some(8));
    }

    #[test]
    fn part_2_example_2() {
        let input = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3";
        let input = input_generator(input);
        assert_eq!(solve_part2(&input), Some(19208));
    }
}
