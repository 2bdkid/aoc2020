use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[derive(Debug)]
pub struct Field {
    name: String,
    range1: (u32, u32),
    range2: (u32, u32),
}

impl Field {
    fn in_range(&self, value: u32) -> bool {
        let (a, b) = self.range1;
        let (x, y) = self.range2;
        value >= a && value <= b || value >= x && value <= y
    }
}

pub type Ticket = Vec<u32>;

#[aoc_generator(day16)]
fn input_generator(input: &str) -> (Vec<Field>, Ticket, Vec<Ticket>) {
    let fields: Vec<Field> = input
        .lines()
        .filter_map(|l| {
            if l.contains("or") {
                let name: String = l.chars().take_while(|c| *c != ':').collect();
                let ranges: Vec<u32> = l
                    .replace("-", " ")
                    .split_ascii_whitespace()
                    .filter_map(|t| t.parse::<u32>().ok())
                    .collect();
                Some(Field {
                    name,
                    range1: (ranges[0], ranges[1]),
                    range2: (ranges[2], ranges[3]),
                })
            } else {
                None
            }
        })
        .collect();

    let tickets: Vec<Ticket> = input
        .lines()
        .filter_map(|l| {
            if l.contains(",") {
                Some(
                    l.split(",")
                        .map(|t| t.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>(),
                )
            } else {
                None
            }
        })
        .collect();

    (fields, tickets[0].clone(), tickets[1..].to_vec())
}

fn sum_completely_invalid_fields(fields: &[Field], ticket: &Ticket) -> u32 {
    ticket
        .iter()
        .filter(|&&value| fields.iter().all(|f| !f.in_range(value)))
        .sum()
}

#[aoc(day16, part1)]
pub fn solve_part1((fields, _, nearby_tickets): &(Vec<Field>, Ticket, Vec<Ticket>)) -> u32 {
    nearby_tickets
        .iter()
        .map(|t| sum_completely_invalid_fields(&fields, t))
        .sum()
}
