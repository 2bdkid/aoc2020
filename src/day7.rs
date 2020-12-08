use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Rule {
    color: String,
    quantities: Vec<(u32, String)>,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<(String, Vec<(u32, String)>)> {
    input
        .lines()
        .map(|line| {
            let line = line
                .to_owned()
                .replace(" bags contain no other bags.", "")
                .replace(" bags contain ", " ")
                .replace(" bags.", "")
                .replace(" bags, ", " ")
                .replace(" bag, ", " ")
                .replace(" bag.", "");
            let mut tokens = line.split_ascii_whitespace();
            let color = tokens
                .take_while_ref(|t| t.parse::<u32>().is_err())
                .join(" ");
            let quantities: Vec<(u32, String)> = tokens
                .batching(|it| {
                    it.next().map(|n| {
                        let n: u32 = n.parse().unwrap();
                        let color = it.take_while_ref(|t| t.parse::<u32>().is_err()).join(" ");
                        (n, color)
                    })
                })
                .collect();
            (color, quantities)
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[(String, Vec<(u32, String)>)]) -> usize {
    let adj_matrix: HashMap<String, Vec<String>> = input
        .iter()
        .map(|(color, quantities)| {
            (
                color.clone(),
                quantities.iter().map(|(_, color)| color).cloned().collect(),
            )
        })
        .collect();

    let has_path_to_shiny_gold = |color: &String| {
        let mut discovered = HashSet::new();
        let mut stack = vec![color];

        while let Some(color) = stack.pop() {
            if color == "shiny gold" {
                return true;
            }

            if !discovered.contains(color) {
                discovered.insert(color.clone());
                stack.extend(
                    adj_matrix
                        .get(color)
                        .unwrap()
                        .iter()
                        .filter(|c| !discovered.contains(*c)),
                );
            }
        }

        false
    };

    adj_matrix
        .keys()
        .cloned()
        .filter(|color| has_path_to_shiny_gold(color))
        .count()
        - 1
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[(String, Vec<(u32, String)>)]) -> u32 {
    let count_matrix: HashMap<String, Vec<(u32, String)>> = input
        .iter()
        .map(|(color, quantities)| (color.clone(), quantities.clone()))
        .collect();
    let mut memo: HashMap<String, u32> = input
        .iter()
        .filter_map(|(color, quantities)| {
            if quantities.is_empty() {
                Some((color.clone(), 0))
            } else {
                None
            }
        })
        .collect();

    fn bag_count(
        color: &str,
        count_matrix: &HashMap<String, Vec<(u32, String)>>,
        memo: &mut HashMap<String, u32>,
    ) -> u32 {
        if let Some(count) = memo.get(color) {
            return *count;
        }

        let count = count_matrix
            .get(color)
            .unwrap()
            .iter()
            .map(|(n, col)| n + n * bag_count(col, count_matrix, memo))
            .sum::<u32>();
        memo.insert(color.to_owned(), count);
        count
    }

    bag_count("shiny gold", &count_matrix, &mut memo)
}
