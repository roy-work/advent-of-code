use std::collections::HashSet;

use once_cell::sync::Lazy;
use regex::Regex;

use aoc::prelude::*;

fn parse_fbbr(line: &str) -> anyhow::Result<u32> {
    let mut row = 0;
    for c in line[0..7].chars() {
        let bit = match c {
            'F' => 0,
            'B' => 1,
            _ => return Err(anyhow::anyhow!("Bad row")),
        };
        row = (row << 1) | bit;
    }
    let mut col = 0;
    for c in line[7..10].chars() {
        let bit = match c {
            'L' => 0,
            'R' => 1,
            _ => return Err(anyhow::anyhow!("Bad col")),
        };
        col = (col << 1) | bit;
    }

    Ok(row * 8 + col)
}


fn main() {
    let seat_ids = aoc::file_item_per_line("input", parse_fbbr).unwrap();

    println!("part 1 = {}", seat_ids.iter().max().unwrap());
    let min_id = seat_ids.iter().min().unwrap();
    let max_id = seat_ids.iter().max().unwrap();
    let seat_ids = seat_ids.iter().collect::<HashSet<_>>();

    for id in 0..1024 {
        if seat_ids.contains(&id) {
            continue;
        }
        if id < *min_id || *max_id < id {
            continue;
        } else {
            println!("{}", id);
        }
    }
}
