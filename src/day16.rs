use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use pathfinding::kuhn_munkres;
use pathfinding::matrix::Matrix;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Field {
    name: String,
    range1: (u64, u64),
    range2: (u64, u64),
}

impl Field {
    fn in_range(&self, value: u64) -> bool {
        let (a, b) = self.range1;
        let (x, y) = self.range2;
        value >= a && value <= b || value >= x && value <= y
    }
}

pub type Ticket = Vec<u64>;

#[aoc_generator(day16)]
fn input_generator(input: &str) -> (Vec<Field>, Ticket, Vec<Ticket>) {
    let fields: Vec<Field> = input
        .lines()
        .filter_map(|l| {
            if l.contains("or") {
                let name: String = l.chars().take_while(|c| *c != ':').collect();
                let ranges: Vec<u64> = l
                    .replace("-", " ")
                    .split_ascii_whitespace()
                    .filter_map(|t| t.parse::<u64>().ok())
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
                        .map(|t| t.parse().unwrap())
                        .collect::<Vec<u64>>(),
                )
            } else {
                None
            }
        })
        .collect();

    (fields, tickets[0].clone(), tickets[1..].to_vec())
}

fn completely_invalid_value(fields: &[Field], value: u64) -> bool {
    fields.iter().all(|f| !f.in_range(value))
}

#[aoc(day16, part1)]
pub fn solve_part1((fields, _, nearby_tickets): &(Vec<Field>, Ticket, Vec<Ticket>)) -> u64 {
    nearby_tickets
        .iter()
        .map(|t| t.iter())
        .flatten()
        .filter(|value| completely_invalid_value(&fields, **value))
        .sum()
}

fn field_is_valid_for_index(field: &Field, index: usize, tickets: &[Ticket]) -> bool {
    tickets
        .iter()
        .map(|t| t[index])
        .all(|value| field.in_range(value))
}

#[aoc(day16, part2)]
pub fn solve_part2(
    (fields, your_ticket, nearby_tickets): &(Vec<Field>, Ticket, Vec<Ticket>),
) -> u64 {
    let tickets: Vec<Ticket> = nearby_tickets
        .iter()
        .chain(vec![your_ticket.clone()].iter())
        .filter(|t| {
            !t.iter()
                .any(|value| completely_invalid_value(&fields, *value))
        })
        .cloned()
        .collect();

    let costs: Matrix<i32> = Matrix::from_rows(
        (0..your_ticket.len())
            .map(|index| {
                fields
                    .iter()
                    .map(|field| field_is_valid_for_index(field, index, &tickets) as i32)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
    .unwrap();

    let (_, assignments) = kuhn_munkres::kuhn_munkres(&costs);

    let mapping: HashMap<String, usize> = assignments
        .iter()
        .enumerate()
        .map(|(row, column)| (fields[*column].name.clone(), row))
        .collect();

    your_ticket[*mapping.get("departure location").unwrap()]
        * your_ticket[*mapping.get("departure station").unwrap()]
        * your_ticket[*mapping.get("departure platform").unwrap()]
        * your_ticket[*mapping.get("departure track").unwrap()]
        * your_ticket[*mapping.get("departure date").unwrap()]
        * your_ticket[*mapping.get("departure time").unwrap()]
}
