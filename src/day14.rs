use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Clone)]
pub struct Mask {
    mask: String,
    overwrite: u64,
    keep: u64,
}

impl TryFrom<&str> for Mask {
    type Error = String;
    fn try_from(mask: &str) -> Result<Mask, String> {
        if mask.chars().any(|c| c != 'X' && c != '1' && c != '0') {
            Err(format!("Invalid character in mask {}", mask))
        } else if mask.chars().count() != 36 {
            Err(format!("Mask length is not 36 bits {}", mask))
        } else {
            let overwrite = mask.replace("X", "0");
            let overwrite = u64::from_str_radix(&overwrite, 2).map_err(|e| e.to_string())?;
            let keep = mask.replace("1", "0").replace("X", "1");
            let keep = u64::from_str_radix(&keep, 2).map_err(|e| e.to_string())?;
            Ok(Mask {
                mask: mask.to_string().clone(),
                overwrite,
                keep,
            })
        }
    }
}

impl Mask {
    fn apply(&self, value: u64) -> u64 {
        value & self.keep | self.overwrite
    }

    fn addresses(&self, addr: usize) -> Vec<usize> {
        let stupid_or: String = self
            .mask
            .chars()
            .zip(format!("{:0>36b}", addr).chars())
            .map(|(m, a)| {
                if m == 'X' {
                    'X'
                } else if m == '1' || a == '1' {
                    '1'
                } else {
                    '0'
                }
            })
            .collect();

        let mut addrs = vec![stupid_or.clone()];

        for _ in 0..stupid_or.chars().filter(|c| *c == 'X').count() {
            let replaced: Vec<Vec<String>> = addrs
                .iter()
                .map(|addr| vec![addr.replacen("X", "0", 1), addr.replacen("X", "1", 1)])
                .collect();
            addrs = replaced.iter().flatten().cloned().collect();
        }

        addrs
            .iter()
            .map(|addr| usize::from_str_radix(addr, 2).unwrap())
            .collect()
    }
}

pub enum Instr {
    Mask(Mask),
    Mem(usize, u64),
}

impl TryFrom<&str> for Instr {
    type Error = String;
    fn try_from(instr: &str) -> Result<Instr, String> {
        if instr.contains("mask") {
            let mask = instr.replace("mask = ", "");
            let mask = Mask::try_from(mask.as_ref())?;
            Ok(Instr::Mask(mask))
        } else {
            let trimmed = instr.replace("mem[", "").replace("] = ", " ");
            let addr: usize = trimmed
                .split_ascii_whitespace()
                .nth(0)
                .unwrap()
                .parse::<usize>()
                .map_err(|e| e.to_string())?;
            let value: u64 = trimmed
                .split_ascii_whitespace()
                .nth(1)
                .unwrap()
                .parse::<u64>()
                .map_err(|e| e.to_string())?;
            Ok(Instr::Mem(addr, value))
        }
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Instr> {
    input.lines().map(|x| Instr::try_from(x).unwrap()).collect()
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &[Instr]) -> u64 {
    let mut memory = HashMap::new();
    let mut current_mask = None;

    for instr in input {
        match instr {
            Instr::Mask(mask) => current_mask = Some(mask.clone()),
            Instr::Mem(addr, value) => {
                memory.insert(*addr, current_mask.as_ref().unwrap().apply(*value));
            }
        }
    }

    memory.values().sum()
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &[Instr]) -> u64 {
    let mut memory = HashMap::new();
    let mut current_mask = None;

    for instr in input {
        match instr {
            Instr::Mask(mask) => current_mask = Some(mask.clone()),
            Instr::Mem(addr, value) => memory.extend(
                current_mask
                    .as_ref()
                    .unwrap()
                    .addresses(*addr)
                    .iter()
                    .map(|addr| (addr, value)),
            ),
        }
    }

    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask1() {
        let mask = Mask::try_from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap();
        assert_eq!(mask.apply(11), 73);
    }

    #[test]
    fn mask2() {
        let mask = Mask::try_from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap();
        assert_eq!(mask.apply(101), 101);
    }

    #[test]
    fn mask3() {
        let mask = Mask::try_from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap();
        assert_eq!(mask.apply(0), 64);
    }

    #[test]
    fn mask4() {
        let mask = Mask::try_from("000000000000000000000000000000X1001X").unwrap();
        let addr = 42;
        assert_eq!(mask.addresses(addr), vec![26, 27, 58, 59]);
    }
}
