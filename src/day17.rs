use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Point {
        Point { x, y, z }
    }

    fn neighbors(&self) -> Vec<Point> {
        (-1..2)
            .cartesian_product(-1..2)
            .cartesian_product(-1..2)
            .filter_map(|((i, j), k)| {
                if i != 0 || j != 0 || k != 0 {
                    Some(Point::new(self.x + i, self.y + j, self.z + k))
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Cube {
    Active,
    Inactive,
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cube::Active => '#',
                Cube::Inactive => '.',
            }
        )
    }
}

impl TryFrom<char> for Cube {
    type Error = String;
    fn try_from(c: char) -> Result<Cube, String> {
        match c {
            '#' => Ok(Cube::Active),
            '.' => Ok(Cube::Inactive),
            _ => Err(format!("Invalid Character for Cube {}", c)),
        }
    }
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> HashMap<Point, Cube> {
    input
        .lines()
        .enumerate()
        .map(|(x, row)| {
            row.chars().enumerate().map(move |(y, cube)| {
                (
                    Point::new(x as i64, y as i64, 0),
                    Cube::try_from(cube).unwrap(),
                )
            })
        })
        .flatten()
        .collect()
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &HashMap<Point, Cube>) -> usize {
    let mut space = input.clone();

    for _ in 0..6 {
        let reach: HashSet<Point> = space
            .keys()
            .map(|point| point.neighbors())
            .flatten()
            .collect();
        space = reach
            .iter()
            .map(|point| {
                let active_neighbors = point
                    .neighbors()
                    .iter()
                    .filter(|point| *space.get(point).unwrap_or(&Cube::Inactive) == Cube::Active)
                    .count();
                let cube = *space.get(point).unwrap_or(&Cube::Inactive);
                if cube == Cube::Active && active_neighbors == 2 || active_neighbors == 3 {
                    (*point, Cube::Active)
                } else if cube == Cube::Active {
                    (*point, Cube::Inactive)
                } else if cube == Cube::Inactive && active_neighbors == 3 {
                    (*point, Cube::Active)
                } else {
                    (*point, Cube::Inactive)
                }
            })
            .collect();
    }

    space.values().filter(|cube| **cube == Cube::Active).count()
}
