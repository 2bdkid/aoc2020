use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::convert::TryFrom;
use std::iter;

#[derive(Copy, Clone)]
pub enum Instr {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl Instr {
    fn toggle(&mut self) -> bool {
        match *self {
            Instr::Acc(_) => false,
            Instr::Jmp(arg) => {
                *self = Instr::Nop(arg);
                true
            }
            Instr::Nop(arg) => {
                *self = Instr::Jmp(arg);
                true
            }
        }
    }
}

impl TryFrom<&str> for Instr {
    type Error = String;
    fn try_from(s: &str) -> Result<Instr, String> {
        let mut tokens = s.split_ascii_whitespace();
        let op = tokens.next().ok_or("No op")?;
        let arg: i32 = tokens
            .next()
            .ok_or("No arg")?
            .parse::<i32>()
            .map_err(|e| e.to_string())?;
        match op {
            "acc" => Ok(Instr::Acc(arg)),
            "jmp" => Ok(Instr::Jmp(arg)),
            "nop" => Ok(Instr::Nop(arg)),
            _ => Err("Invalid op".to_owned()),
        }
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Instr> {
    input
        .lines()
        .map(|instr| Instr::try_from(instr).unwrap())
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[Instr]) -> i32 {
    let mut memory: Vec<Option<Instr>> = input.iter().copied().map(|instr| Some(instr)).collect();
    let mut pc: i32 = 0;
    let mut accumulator = 0;

    while let Some(instr) = memory.get_mut(pc as usize).unwrap().take() {
        match instr {
            Instr::Acc(arg) => {
                accumulator += arg;
                pc += 1;
            }
            Instr::Jmp(arg) => {
                pc += arg;
            }
            Instr::Nop(_) => {
                pc += 1;
            }
        }
    }

    accumulator
}

fn try_bootcode(memory: &Vec<Instr>) -> Option<i32> {
    let mut memory: Vec<Option<Instr>> = memory.iter().copied().map(|i| Some(i)).collect();
    let mut pc = 0;
    let mut accumulator = 0;

    while let Some(instr) = memory.get_mut(pc as usize) {
        if let Some(instr) = instr.take() {
            match instr {
                Instr::Acc(arg) => {
                    accumulator += arg;
                    pc += 1;
                }
                Instr::Jmp(arg) => {
                    pc += arg;
                }
                Instr::Nop(_) => {
                    pc += 1;
                }
            }
        } else {
            return None;
        }
    }

    Some(accumulator)
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[Instr]) -> Option<i32> {
    let memory = input.to_vec();

    for (i, mut memory) in iter::repeat(memory).enumerate() {
        if let Some(instr) = memory.get_mut(i) {
            if instr.toggle() {
                let acc = try_bootcode(&memory);
                if acc.is_some() {
                    return acc;
                }
            }
        } else {
            break;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";
        let input = input_generator(input);
        assert_eq!(solve_part1(&input), 5);
    }
}
