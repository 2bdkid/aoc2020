use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Position {
    Floor,
    Empty,
    Occupied,
}

impl TryFrom<char> for Position {
    type Error = String;
    fn try_from(c: char) -> Result<Position, String> {
        match c {
            '.' => Ok(Position::Floor),
            'L' => Ok(Position::Empty),
            '#' => Ok(Position::Occupied),
            _ => Err(format!("Invalid Position {}", c)),
        }
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> HashMap<(i32, i32), Position> {
    input
        .lines()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(move |(x, p)| ((x as i32, y as i32), Position::try_from(p).unwrap()))
        })
        .flatten()
        .collect()
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &HashMap<(i32, i32), Position>) -> usize {
    let mut positions = input.clone();

    loop {
        let changes: Vec<((i32, i32), Position)> = positions
            .iter()
            .filter_map(|((x, y), p)| {
                if *p != Position::Floor {
                    let occupied = [
                        (x - 1, y - 1),
                        (*x, y - 1),
                        (x + 1, y - 1),
                        (x - 1, *y),
                        (x + 1, *y),
                        (x - 1, y + 1),
                        (*x, y + 1),
                        (x + 1, y + 1),
                    ]
                    .iter()
                    .filter(|(a, b)| {
                        positions
                            .get(&(*a, *b))
                            .filter(|p| **p == Position::Occupied)
                            .is_some()
                    })
                    .count();

                    if *p == Position::Empty && occupied == 0 {
                        return Some(((*x, *y), Position::Occupied));
                    }

                    if *p == Position::Occupied && occupied >= 4 {
                        return Some(((*x, *y), Position::Empty));
                    }
                }

                None
            })
            .collect();

        if changes.is_empty() {
            break;
        }

        positions.extend(changes.iter().copied());
    }

    positions
        .values()
        .filter(|p| **p == Position::Occupied)
        .count()
}
