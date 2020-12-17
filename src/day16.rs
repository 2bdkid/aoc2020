use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::collections::{HashMap, HashSet};

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

fn completely_invalid_value(fields: &[Field], value: u32) -> bool {
    fields.iter().all(|f| !f.in_range(value))
}

#[aoc(day16, part1)]
pub fn solve_part1((fields, _, nearby_tickets): &(Vec<Field>, Ticket, Vec<Ticket>)) -> u32 {
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

fn fill_index_mapping(
    index: usize,
    mapping: HashMap<String, usize>,
    valid_fields_for_index: HashMap<usize, HashSet<String>>,
) -> Option<HashMap<String, usize>> {
    if index > 19 {
        return Some(mapping.clone());
    }

    let valid_fields: HashSet<String> = valid_fields_for_index.get(&index).cloned().unwrap();

    for field in valid_fields {
        let mut new_mapping = mapping.clone();
        new_mapping.insert(field.clone(), index);

        let new_valid_fields_for_index: HashMap<usize, HashSet<String>> = valid_fields_for_index
            .iter()
            .map(|(index, fields)| {
                let mut new_fields = fields.clone();
                new_fields.remove(&field);
                (*index, new_fields)
            })
            .collect();

        if let Some(completed_mapping) =
            fill_index_mapping(index + 1, new_mapping, new_valid_fields_for_index)
        {
            return Some(completed_mapping);
        }
    }

    None
}

#[aoc(day16, part2)]
pub fn solve_part2(
    (fields, your_ticket, nearby_tickets): &(Vec<Field>, Ticket, Vec<Ticket>),
) -> u32 {
    let tickets: Vec<Ticket> = nearby_tickets
        .iter()
        .chain(vec![your_ticket.clone()].iter())
        .filter(|t| {
            !t.iter()
                .any(|value| completely_invalid_value(&fields, *value))
        })
        .cloned()
        .collect();

    let valid_fields_for_index: HashMap<usize, HashSet<String>> = (0..your_ticket.len())
        .map(|index| {
            (
                index,
                fields
                    .iter()
                    .filter_map(|field| {
                        if field_is_valid_for_index(field, index, &tickets) {
                            Some(field.name.clone())
                        } else {
                            None
                        }
                    })
                    .collect(),
            )
        })
        .collect();

    let mapping = fill_index_mapping(0, HashMap::new(), valid_fields_for_index).unwrap();
    println!("{:?}", mapping);

    your_ticket[*mapping.get("departure location").unwrap()]
        * your_ticket[*mapping.get("departure station").unwrap()]
        * your_ticket[*mapping.get("departure platform").unwrap()]
        * your_ticket[*mapping.get("departure track").unwrap()]
        * your_ticket[*mapping.get("departure date").unwrap()]
        * your_ticket[*mapping.get("departure time").unwrap()]
}
