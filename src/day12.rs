use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::convert::TryFrom;

pub type Angle = i32;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn angle(&self) -> Angle {
        match *self {
            Direction::East => 0,
            Direction::North => 90,
            Direction::West => 180,
            Direction::South => 270,
        }
    }
}

impl TryFrom<Angle> for Direction {
    type Error = String;
    fn try_from(angle: Angle) -> Result<Direction, String> {
        match angle % 360 {
            0 => Ok(Direction::East),
            90 => Ok(Direction::North),
            180 => Ok(Direction::West),
            270 => Ok(Direction::South),
            _ => Err(format!("Invalid Angle {}", angle)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(Angle),
    Right(Angle),
    Forward(i32),
}

impl TryFrom<&str> for Instruction {
    type Error = String;
    fn try_from(s: &str) -> Result<Instruction, String> {
        if let Some(i) = s.chars().nth(0) {
            if let Ok(units) = s[1..].parse::<i32>() {
                return match i {
                    'N' => Ok(Instruction::North(units)),
                    'S' => Ok(Instruction::South(units)),
                    'E' => Ok(Instruction::East(units)),
                    'W' => Ok(Instruction::West(units)),
                    'L' => Ok(Instruction::Left(units)),
                    'R' => Ok(Instruction::Right(units)),
                    'F' => Ok(Instruction::Forward(units)),
                    _ => Err(format!("Invalid Action {}", s)),
                };
            }
            Err(format!("Invalid Units {}", s))
        } else {
            Err("Empty String".to_string())
        }
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| Instruction::try_from(l).unwrap())
        .collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[Instruction]) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut direction = Direction::East;

    for instr in input {
        match instr {
            Instruction::North(units) => y += units,
            Instruction::South(units) => y -= units,
            Instruction::East(units) => x += units,
            Instruction::West(units) => x -= units,
            Instruction::Left(angle) => {
                direction = Direction::try_from(direction.angle() + angle).unwrap()
            }
            Instruction::Right(angle) => {
                direction = Direction::try_from(direction.angle() - angle + 360).unwrap()
            }
            Instruction::Forward(units) => match direction {
                Direction::East => x += units,
                Direction::West => x -= units,
                Direction::North => y += units,
                Direction::South => y -= units,
            },
        }
    }

    x.abs() + y.abs()
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &[Instruction]) -> i32 {
    let mut x = 0;
    let mut y = 0;

    // relative to ship
    let mut wx = 10;
    let mut wy = 1;

    for instr in input {
        match instr {
            Instruction::North(units) => wy += units,
            Instruction::South(units) => wy -= units,
            Instruction::East(units) => wx += units,
            Instruction::West(units) => wx -= units,
            Instruction::Left(angle) => {
                let angle = 360 - *angle;
                for _ in 0..angle / 90 {
                    let t = wx;
                    wx = wy;
                    wy = -t;
                }
            }
            Instruction::Right(angle) => {
                for _ in 0..angle / 90 {
                    let t = wx;
                    wx = wy;
                    wy = -t;
                }
            }
            Instruction::Forward(units) => {
                x += wx * units;
                y += wy * units;
            }
        }
    }

    x.abs() + y.abs()
}
