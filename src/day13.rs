use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> (u128, Vec<(u128, Option<u128>)>) {
    let estimate: u128 = input.lines().nth(0).unwrap().parse().unwrap();
    let ids = input
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .map(|(u, id)| (u as u128, id.parse().ok()))
        .collect();
    (estimate, ids)
}

#[aoc(day13, part1)]
pub fn solve_part1((estimate, ids): &(u128, Vec<(u128, Option<u128>)>)) -> Option<u128> {
    ids.iter()
        .filter_map(|(_, period)| {
            period.map(|period| (period, (period - estimate % period) % period))
        })
        .min_by(|(_, w1), (_, w2)| w1.cmp(w2))
        .map(|(id, wait)| id * wait)
}

#[aoc(day13, part2)]
pub fn solve_part2((_, ids): &(u128, Vec<(u128, Option<u128>)>)) -> u128 {
    let mut t = 0;
    let mut mul = 1;

    for (offset, id) in ids
        .iter()
        .filter_map(|(offset, id)| id.map(|id| (offset, id)))
    {
        while (t + offset) % id != 0 {
            t += mul;
        }
        mul *= id;
    }

    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "123\n7,13,x,x,59,x,31,19";
        let input = input_generator(input);
        assert_eq!(solve_part2(&input), 1068781);
    }
}
