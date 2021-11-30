use std::collections::{hash_map, VecDeque};
use std::convert::TryFrom;
use std::iter::FromIterator;

use anyhow::Context;
use once_cell::sync::Lazy;
use regex::Regex;

use aoc::prelude::*;

aoc::lazy_regex!(CLASS_RE, "^([^:]+): (\\d+)-(\\d+) or (\\d+)-(\\d+)$");

struct Range {
    lo: i64,
    hi: i64,
}

impl Range {
    fn contains(&self, val: i64) -> bool {
        self.lo <= val && val <= self.hi
    }
}

struct Input {
    classes: Vec<(String, Range, Range)>,
    your_ticket: Vec<i64>,
    nearby_tickets: Vec<Vec<i64>>,
}

fn parser(path: &Path) -> anyhow::Result<Input> {
    let file = BufReader::new(File::open(path)?);

    let mut lines = file.lines();

    let mut classes = Vec::new();

    loop {
        let line = lines.next().unwrap()?;
        if line.is_empty() {
            break;
        }
        let captures = CLASS_RE.captures(&line).unwrap();
        let class_name = captures.get(1).unwrap().as_str().to_owned();
        let r1_lo = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let r1_hi = captures.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let r2_lo = captures.get(4).unwrap().as_str().parse::<i64>().unwrap();
        let r2_hi = captures.get(5).unwrap().as_str().parse::<i64>().unwrap();

        classes.push((
            class_name,
            Range {
                lo: r1_lo,
                hi: r1_hi,
            },
            Range {
                lo: r2_lo,
                hi: r2_hi,
            },
        ));
    }

    lines.next().unwrap()?; // "your ticket:"

    let your_ticket = comma_line(&lines.next().unwrap()?)?;

    lines.next().unwrap()?; // empty line
    lines.next().unwrap()?; //"nearby tickets:"

    let mut nearby_tickets = Vec::new();
    for line in lines {
        let line = line?;
        let ticket = comma_line(&line)?;
        nearby_tickets.push(ticket);
    }

    Ok(Input {
        classes,
        your_ticket,
        nearby_tickets,
    })
}

fn comma_line(line: &str) -> anyhow::Result<Vec<i64>> {
    Ok(line
        .split(',')
        .map(|p| p.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?)
}

fn part_a(input: &Input) -> i64 {
    let mut invalid_sum = 0;
    for ticket in &input.nearby_tickets {
        for value in ticket {
            let all_invalid = input
                .classes
                .iter()
                .all(|(_, r1, r2)| !r1.contains(*value) && !r2.contains(*value));
            if all_invalid {
                invalid_sum += value;
            }
        }
    }
    invalid_sum
}

fn part_b(input: &Input) -> i64 {
    let mut valid_tickets = Vec::<&Vec<i64>>::new();
    for ticket in &input.nearby_tickets {
        let mut has_invalid_value = false;
        for value in ticket {
            let all_invalid = input
                .classes
                .iter()
                .all(|(_, r1, r2)| !r1.contains(*value) && !r2.contains(*value));
            if all_invalid {
                has_invalid_value = true;
                break;
            }
        }

        if !has_invalid_value {
            valid_tickets.push(ticket);
        }
    }

    let mut queue = VecDeque::new();
    for class in &input.classes {
        queue.push_back(class);
    }

    let mut col_map = HashMap::<usize, &str>::new();
    let mut remaining_indexes = (0..input.your_ticket.len()).collect::<HashSet<usize>>();
    let mut steps_since_last_assign = 0;
    while !queue.is_empty() {
        let entry = queue.pop_front().unwrap();
        let (cls_name, r1, r2) = entry;
        let mut found_indexes: Vec<usize> = Vec::new();
        for index in remaining_indexes.iter() {
            if does_class_work_as_column((r1, r2), *index, &valid_tickets) {
                found_indexes.push(*index);
            }
        }

        match found_indexes.as_slice() {
            [] => panic!(),
            [index] => {
                println!("Assigning {} -> {}", index, cls_name);
                steps_since_last_assign = 0;
                col_map.insert(*index, cls_name);
                remaining_indexes.remove(index);
            },
            _ => {
                queue.push_back(entry);
            }
        }

        if steps_since_last_assign > 1000 {
            panic!();
        }
        steps_since_last_assign += 1;
    }

    let mut product = 1;
    for (idx, name) in col_map.iter() {
        if !name.starts_with("departure ") {
            continue;
        }

        let value = input.your_ticket[*idx];
        product *= value;
    }

    product
}

fn does_class_work_as_column(
    ranges: (&Range, &Range),
    col_idx: usize,
    tickets: &Vec<&Vec<i64>>,
) -> bool {
    for ticket in tickets {
        let value = ticket[col_idx];
        if !(ranges.0.contains(value) || ranges.1.contains(value)) {
            return false;
        }
    }

    return true;
}

aoc::aoc!(parser, part_a, part_b, None, None);
