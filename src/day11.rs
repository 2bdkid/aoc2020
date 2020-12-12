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

fn count_adjacent_occupied_seats(pt: (i32, i32), grid: &HashMap<(i32, i32), Position>) -> u32 {
    let (x, y) = pt;
    let mut acc = 0;

    // up
    for i in 1.. {
        if let Some(p) = grid.get(&(x, y - i)) {
            if *p != Position::Floor {
                if *p == Position::Occupied {
                    acc += 1;
                }
                break;
            }
        } else {
            break;
        }
    }

    // down
    for i in 1.. {
        if let Some(p) = grid.get(&(x, y + i)) {
            if *p != Position::Floor {
                if *p == Position::Occupied {
                    acc += 1;
                }
                break;
            }
        } else {
            break;
        }
    }

    // left
    for i in 1.. {
        if let Some(p) = grid.get(&(x - i, y)) {
            if *p != Position::Floor {
                if *p == Position::Occupied {
                    acc += 1;
                }
                break;
            }
        } else {
            break;
        }
    }

    // right
    for i in 1.. {
        if let Some(p) = grid.get(&(x + i, y)) {
            if *p != Position::Floor {
                if *p == Position::Occupied {
                    acc += 1;
                }
                break;
            }
        } else {
            break;
        }
    }

    // top-right
    for i in 1.. {
        if let Some(p) = grid.get(&(x + i, y - i)) {
            if *p != Position::Floor {
                if *p == Position::Occupied {
                    acc += 1;
                }
                break;
            }
        } else {
            break;
        }
    }

    // top-left
    for i in 1.. {
        if let Some(p) = grid.get(&(x - i, y - i)) {
            if *p != Position::Floor {
                if *p == Position::Occupied {
                    acc += 1;
                }
                break;
            }
        } else {
            break;
        }
    }

    // bottom-left
    for i in 1.. {
        if let Some(p) = grid.get(&(x - i, y + i)) {
            if *p != Position::Floor {
                if *p == Position::Occupied {
                    acc += 1;
                }
                break;
            }
        } else {
            break;
        }
    }

    // bottom-right
    for i in 1.. {
        if let Some(p) = grid.get(&(x + i, y + i)) {
            if *p != Position::Floor {
                if *p == Position::Occupied {
                    acc += 1;
                }
                break;
            }
        } else {
            break;
        }
    }

    acc
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &HashMap<(i32, i32), Position>) -> usize {
    let mut positions = input.clone();

    loop {
        let adj: HashMap<(i32, i32), u32> = positions
            .iter()
            .filter_map(|(pt, p)| {
                if *p != Position::Floor {
                    Some(*pt)
                } else {
                    None
                }
            })
            .map(|p| (p, count_adjacent_occupied_seats(p, &positions)))
            .collect();

        let mut changes: Vec<((i32, i32), Position)> = Vec::new();

        for (pt, occupied) in adj {
            if *positions.get(&pt).unwrap() == Position::Empty && occupied == 0 {
                changes.push((pt, Position::Occupied));
            } else if *positions.get(&pt).unwrap() == Position::Occupied && occupied >= 5 {
                changes.push((pt, Position::Empty));
            }
        }

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
