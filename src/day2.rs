use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[derive(Debug)]
pub struct PasswordPolicy {
    a: usize,
    b: usize,
    c: char,
}

pub type Password = String;

impl PasswordPolicy {
    pub fn new(a: usize, b: usize, c: char) -> PasswordPolicy {
        PasswordPolicy { a, b, c }
    }

    pub fn satisfied_by_old(&self, password: &Password) -> bool {
        let count = password.chars().filter(|&c| c == self.c).count();
        count >= self.a && count <= self.b
    }

    pub fn satisfied_by(&self, password: &Password) -> bool {
        let c1 = password.chars().nth(self.a - 1).unwrap();
        let c2 = password.chars().nth(self.b - 1).unwrap();
        (c1 == self.c) ^ (c2 == self.c)
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(PasswordPolicy, Password)> {
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
pub fn solve_part1(input: &[(PasswordPolicy, Password)]) -> usize {
    input
        .iter()
        .filter(|(policy, password)| policy.satisfied_by_old(password))
        .count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[(PasswordPolicy, Password)]) -> usize {
    input
        .iter()
        .filter(|(policy, password)| policy.satisfied_by(password))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_policy_satisfied_by_true() {
        let policy = PasswordPolicy::new(1, 3, 'c');
        let password = Password::from("abecdec");
        assert!(policy.satisfied_by_old(&password))
    }

    #[test]
    fn password_policy_satisfied_by_false() {
        let policy = PasswordPolicy::new(1, 3, 'c');
        let password = Password::from("aaa");
        assert!(!policy.satisfied_by(&password))
    }
}
