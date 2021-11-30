use std::collections::{hash_map, VecDeque};
use std::convert::TryFrom;
use std::iter::FromIterator;

use anyhow::Context;
use once_cell::sync::Lazy;
use regex::Regex;

use aoc::prelude::*;

aoc::lazy_regex!(FOOD_RE, "^([a-z]+(:? [a-z]+)*) \\(contains ([^)]+)\\)$");

#[derive(Clone)]
struct Input {
    cups: VecDeque<u8>,
    moves: u32,
}

fn parser(path: &Path) -> anyhow::Result<Input> {
    let file = BufReader::new(File::open(path)?);
    let mut lines = file.lines();

    let cups = lines.next().unwrap()?.chars().map(|c| format!("{}", c).parse::<u8>().unwrap()).collect::<VecDeque<u8>>();
    let moves = lines.next().unwrap()?.parse::<u32>().unwrap();

    Ok(Input {
        cups,
        moves,
    })
}

fn part_a(input: &Input) -> String {
    let mut cups = input.cups.clone();
    let highest_cup = input.cups.iter().copied().max().unwrap();
    for _ in 0..input.moves {
        let current_cup = *cups.front().unwrap();
        let removed_cups = remove_three(&mut cups);

        let insert_index = find_insert(current_cup, highest_cup, &cups);
        cups = insert_removed_cups(cups, removed_cups, insert_index);
        cups.rotate_left(1);
    }

    while *cups.front().unwrap() != 1 {
        cups.rotate_left(1);
    }

    cups.pop_front();
    let mut result = String::new();
    for cup in cups {
        result.push_str(&format!("{}", cup));
    }
    result
}

fn remove_three<T>(cups: &mut VecDeque<T>) -> Vec<T> {
    cups.rotate_left(1);
    let mut result = Vec::new();
    result.push(cups.pop_front().unwrap());
    result.push(cups.pop_front().unwrap());
    result.push(cups.pop_front().unwrap());
    cups.rotate_right(1);
    result
}

fn find_insert(mut current_cup: u8, highest_cup: u8, cups: &VecDeque<u8>) -> usize {
    let cup_to_index = cups.iter().enumerate().map(|(idx, c)| (c, idx)).collect::<HashMap<_, _>>();

    let cup_mod = highest_cup + 1;
    loop {
        current_cup = (current_cup + (cup_mod) - 1) % cup_mod;
        if let Some(idx) = cup_to_index.get(&current_cup) {
            return idx + 1;
        }
    }
}

fn insert_removed_cups<T>(mut cups: VecDeque<T>, removed_cups: Vec<T>, index: usize) -> VecDeque<T> {
    let mut result = VecDeque::new();
    for _ in 0..index {
        result.push_back(cups.pop_front().unwrap());
    }
    for cup in removed_cups {
        result.push_back(cup);
    }
    for cup in cups {
        result.push_back(cup);
    }

    result
}

fn part_b(input: &Input) -> i64 {
    let mut cups = input.cups.iter().map(|c| *c as i32).collect::<VecDeque<i32>>();

    // Extend
    let mut max_cup = cups.iter().copied().max().unwrap();
    while cups.len() < 1_000_000 {
        max_cup += 1;
        cups.push_back(max_cup);
    }
    let mut removed_cups = [0i32; 3];
    let moves = 10_000_000;

    let highest_cup = cups.iter().copied().max().unwrap();
    for this_move in 0..moves {
        let current_cup = *cups.front().unwrap();
        remove_three_b(&mut cups, &mut removed_cups);

        let insert_index = find_insert_b(current_cup, highest_cup, &removed_cups, &cups);
        insert_removed_cups_b(&mut cups, &removed_cups, insert_index);
        cups.rotate_left(1);
        if this_move & 0xff == 0 {
            print!("\r\x1b[KMove {}", this_move);
            use std::io::Write;
            std::io::stdout().flush().unwrap();
        }
    }

    while *cups.front().unwrap() != 1 {
        cups.rotate_left(1);
    }

    cups.pop_front();

    let a = cups.pop_front().unwrap();
    let b = cups.pop_front().unwrap();
    (a as i64) * (b as i64)
}

fn remove_three_b(cups: &mut VecDeque<i32>, result: &mut [i32; 3]) {
    cups.rotate_left(1);
    if cups.len() < 3 {
        panic!();
    }
    result[0] = cups.pop_front().unwrap();
    result[1] = cups.pop_front().unwrap();
    result[2] = cups.pop_front().unwrap();
    cups.rotate_right(1);
}

fn find_insert_b(mut current_cup: i32, highest_cup: i32, removed_cups: &[i32], cups: &VecDeque<i32>) -> usize {
    let cup_mod = highest_cup + 1;
    loop {
        current_cup = (current_cup + (cup_mod) - 1) % cup_mod;
        if current_cup == 0 {
            current_cup = (current_cup + (cup_mod) - 1) % cup_mod;
        }
        if !removed_cups.contains(&current_cup) {
            break;
        }
    }

    cups.iter().copied().enumerate().filter(|(idx, c)| *c == current_cup).map(|(idx, c)| idx).next().unwrap()
}

fn insert_removed_cups_b<T: Copy>(cups: &mut VecDeque<T>, removed_cups: &[T], mut index: usize) {
    for cup in removed_cups {
        cups.insert(index, *cup);
        index += 1;
    }
}
/*/
fn part_b(input: &Input) -> i64 {
    let mut cups = input.cups.iter().map(|c| *c as i32).collect::<Vec<i32>>();

    // Extend
    let mut max_cup = cups.iter().copied().max().unwrap();
    while cups.len() < 1_000_000 {
        max_cup += 1;
        cups.push(max_cup);
    }
    let mut removed_cups = [0i32; 3];
    let moves = 10_000_000;

    let highest_cup = cups.iter().copied().max().unwrap();
    for this_move in 0..moves {
        let current_cup = cups[0];
        remove_three_b(&mut cups, &mut removed_cups);

        let insert_index = find_insert_b(current_cup, highest_cup, &removed_cups, &cups[4..]) + 4;
        schooch(&mut cups, insert_index, &removed_cups);
        if this_move & 0xff == 0 {
            print!("\r\x1b[KMove {}", this_move);
            use std::io::Write;
            std::io::stdout().flush().unwrap();
        }
    }

    let mut cups = cups.iter().copied().collect::<VecDeque<_>>();
    while *cups.front().unwrap() != 1 {
        cups.rotate_left(1);
    }

    cups.pop_front();

    let a = cups.pop_front().unwrap();
    let b = cups.pop_front().unwrap();
    (a as i64) * (b as i64)
}

fn remove_three_b(cups: &[i32], result: &mut [i32; 3]) {
    if cups.len() < 4 {
        panic!();
    }
    result[0] = cups[1];
    result[1] = cups[2];
    result[2] = cups[3];
}

fn find_insert_b(mut current_cup: i32, highest_cup: i32, removed_cups: &[i32], cups: &[i32]) -> usize {
    let cup_mod = highest_cup + 1;
    loop {
        current_cup = (current_cup + (cup_mod) - 1) % cup_mod;
        if current_cup == 0 {
            current_cup = (current_cup + (cup_mod) - 1) % cup_mod;
        }
        if !removed_cups.contains(&current_cup) {
            break;
        }
    }

    cups.iter().copied().enumerate().filter(|(idx, c)| *c == current_cup).map(|(idx, c)| idx).next().unwrap()
}

fn insert_removed_cups_b(cups: &mut VecDeque<i32>, removed_cups: &[i32], mut index: usize) {
    for cup in removed_cups {
        cups.insert(index, *cup);
        index += 1;
    }
}

fn schooch(cups: &mut [i32], index: usize, removed: &[i32; 3]) {
    cups.copy_within(4..index, 1);
    let subcups = &mut (&mut cups[index - 3..])[..3];
    subcups.copy_from_slice(removed);
}
*/

//aoc::aoc!(parser, part_a, part_b, Some("92658374".to_string()), Some(149245887792));
aoc::aoc!(parser, part_a, part_b, Some("92658374".to_string()), None);
