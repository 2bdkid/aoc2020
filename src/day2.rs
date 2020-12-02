use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[derive(Debug)]
pub struct PasswordPolicy {
    a: u32,
    b: u32,
    c: char,
}

type Password = String;

impl PasswordPolicy {
    pub fn new(a: u32, b: u32, c: char) -> PasswordPolicy {
        PasswordPolicy { a, b, c }
    }

    pub fn satisfied_by_old(&self, password: &Password) -> bool {
        let count = password.chars().filter(|&c| c == self.c).count() as u32;
        count >= self.a && count <= self.b
    }

    pub fn satisfied_by(&self, password: &Password) -> bool {
        let c1 = password.chars().nth(self.a as usize - 1).unwrap();
        let c2 = password.chars().nth(self.b as usize - 1).unwrap();
        return (c1 == self.c) ^ (c2 == self.c);
    }
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<(PasswordPolicy, Password)> {
    input
        .lines()
        .map(|l| {
            let mut it = l.split(&['-', ' ', ':'][..]);
            let a = it.next().unwrap().parse().unwrap();
            let b = it.next().unwrap().parse().unwrap();
            let c = it.next().unwrap().chars().nth(0).unwrap();
            it.next().unwrap();
            let password = Password::from(it.next().unwrap());
            (PasswordPolicy::new(a, b, c), password)
        })
        .collect()
}

#[aoc(day2, part1)]
fn solve_part1(input: &[(PasswordPolicy, Password)]) -> u32 {
    input
        .iter()
        .filter(|(policy, password)| policy.satisfied_by_old(password))
        .count() as u32
}

#[aoc(day2, part2)]
fn solve_part2(input: &[(PasswordPolicy, Password)]) -> u32 {
    input
        .iter()
        .filter(|(policy, password)| policy.satisfied_by(password))
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_policy_satisfied_by_true() {
        let policy = PasswordPolicy::new(1, 3, 'c');
        let password = Password::from("abecdec");
        assert!(policy.satisfied_by(&password))
    }

    #[test]
    fn password_policy_satisfied_by_false() {
        let policy = PasswordPolicy::new(1, 3, 'c');
        let password = Password::from("aaa");
        assert!(!policy.satisfied_by(&password))
    }
}
