use std::collections::{VecDeque, hash_map};
use std::convert::TryFrom;
use std::iter::FromIterator;

use anyhow::Context;
use once_cell::sync::Lazy;
use regex::Regex;

use aoc::prelude::*;

aoc::lazy_regex!(MASK_RE, "^mask = ([X10]+)$");
aoc::lazy_regex!(WRITE_RE, "^mem\\[(\\d+)\\] = (\\d+)$");

enum Instruction {
    Mask(String),
    Write { addr: i64, value: i64 },
}

fn parser(path: &Path) -> anyhow::Result<Vec<i64>> {
    let file = BufReader::new(File::open(path)?);
    Ok(file
        .lines()
        .next()
        .unwrap()?
        .split(',')
        .map(|p| p.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?)
}

fn part_a(input: &Vec<i64>) -> i64 {
    let queue = VecDeque::from_iter(input.iter());
    let mut speak_ages = HashMap::<i64, i64>::new();

    let mut turn: i64 = 1;
    let mut last_spoken: i64 = 0;
    let mut spoken_before_on: Option<i64> = None;
    for number in queue {
        let before = speak_ages.insert(*number, turn);
        spoken_before_on = before;
        last_spoken = *number;
        turn += 1;
    }
    loop {
        let speak = if let Some(spoke_turn) = spoken_before_on {
            //println!("Spoke {} on turn {}", last_spoken, spoke_turn);
            (turn - 1) - spoke_turn
        } else {
            0
        };
        let before = speak_ages.insert(speak, turn);
        //println!("Turn: {}, last spoke = {}, speaking: {}", turn, last_spoken, speak);
        spoken_before_on = before;
        last_spoken = speak;

        if turn == 2020 {
            break;
        }

        turn += 1;
    }

    last_spoken
}

fn part_b(input: &Vec<i64>) -> i64 {
    let queue = VecDeque::from_iter(input.iter());
    let mut speak_ages = HashMap::<i64, i64>::new();

    let mut turn: i64 = 1;
    let mut last_spoken: i64 = 0;
    let mut spoken_before_on: Option<i64> = None;
    for number in queue {
        let before = speak_ages.insert(*number, turn);
        spoken_before_on = before;
        last_spoken = *number;
        turn += 1;
    }
    loop {
        let speak = if let Some(spoke_turn) = spoken_before_on {
            //println!("Spoke {} on turn {}", last_spoken, spoke_turn);
            (turn - 1) - spoke_turn
        } else {
            0
        };
        let before = speak_ages.insert(speak, turn);
        //println!("Turn: {}, last spoke = {}, speaking: {}", turn, last_spoken, speak);
        spoken_before_on = before;
        last_spoken = speak;

        if turn == 30_000_000 {
            break;
        }

        turn += 1;
    }

    last_spoken
}

aoc::aoc!(parser, part_a, part_b, Some(436), Some(208));
