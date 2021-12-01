use std::collections::{hash_map, VecDeque};
use std::convert::TryFrom;
use std::iter::FromIterator;

use anyhow::Context;
use once_cell::sync::Lazy;
use regex::Regex;

use aoc::prelude::*;

fn parser(path: &Path) -> anyhow::Result<Vec<i64>> {
    Ok(aoc::file_o_numbers(path)?)
}

fn part_a(input: &Vec<i64>) -> i64 {
    let mut last = input[0];
    let mut larger = 0;
    for measure in &input[1..] {
        if *measure > last {
            larger += 1;
        }
        last = *measure;
    }
    larger
}

fn part_b(input: &Vec<i64>) -> usize {
    let mut larger = 0;
    let mut iter = input.windows(3).map(|window| window.iter().sum());
    let mut last: i64 = iter.next().unwrap();
    for sum in iter {
        if sum> last {
            larger += 1;
        }
        last = sum;
    }
    larger
}

aoc::aoc!(parser, part_a, part_b, Some(7), Some(5));
