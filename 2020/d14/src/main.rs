use std::collections::HashSet;
use std::convert::TryFrom;

use anyhow::Context;
use once_cell::sync::Lazy;
use regex::Regex;

use aoc::prelude::*;

aoc::lazy_regex!(MASK_RE, "^mask = ([X10]+)$");
aoc::lazy_regex!(WRITE_RE, "^mem\\[(\\d+)\\] = (\\d+)$");

enum Instruction {
    Mask(String),
    Write {
        addr: i64,
        value: i64,
    }
}

fn parser(path: &Path) -> anyhow::Result<Vec<Instruction>> {
    aoc::file_item_per_line(path, |line| -> anyhow::Result<Instruction> {
        let captures = MASK_RE.captures(line);
        if let Some(captures) = captures {
            let mask = captures.get(1).unwrap().as_str().to_owned();
            Ok(Instruction::Mask(mask))
        } else {
            let captures = WRITE_RE.captures(line).unwrap();
            let addr = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let value = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
            Ok(Instruction::Write { addr, value })
        }
    })
    .context("failed to parse file")
}

fn part_a(input: &Vec<Instruction>) -> i64 {
    let mem = run(&input);
    mem.values().sum()
}

fn part_b(input: &Vec<Instruction>) -> i64 {
    let mem = run_v2(&input);
    mem.values().sum()
}

fn run(instructions: &[Instruction]) -> HashMap<i64, i64> {
    let mut mem = HashMap::<i64, i64>::new();
    let mut current_mask = None;
    for instruction in instructions {
        match instruction {
            Instruction::Mask(m) => current_mask = Some(m),
            Instruction::Write { addr, value } => {
                let mut value: i64 = *value;
                let mask = current_mask.unwrap();
                for (shift, mc) in mask.chars().rev().enumerate() {
                    match mc {
                        '1' => {
                            value |= 1 << shift;
                        },
                        '0' => {
                            value ^= ((value >> shift) & 1) << shift;
                        },
                        'X' => (),
                        _ => panic!(),
                    }
                }
                mem.insert(*addr, value);
            }
        }
    }
    mem
}

fn run_v2(instructions: &[Instruction]) -> HashMap<i64, i64> {
    let mut mem = HashMap::<i64, i64>::new();
    let mut current_mask = None;
    for instruction in instructions {
        match instruction {
            Instruction::Mask(m) => current_mask = Some(m),
            Instruction::Write { addr, value } => {
                let mut value: i64 = *value;
                let mask = current_mask.unwrap();
                let addr_iter = AddrIter::new(mask, *addr);
                for ea in addr_iter {
                    println!("Write: {} <- {}", ea, value);
                    mem.insert(ea, value);
                }
            }
        }
    }
    mem
}

struct AddrIter<'a> {
    base_addr: i64,
    next_val: i64,
    stop_val: i64,
    mask: &'a str,
}

impl<'a> AddrIter<'a> {
    fn new(mask: &'a str, base_addr: i64) -> AddrIter<'a> {
        let xs = mask.chars().filter(|c| *c == 'X').count() as i64;
        AddrIter {
            base_addr,
            next_val: 0,
            stop_val: 1 << xs,
            mask,
        }
    }
}

impl Iterator for AddrIter<'_> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_val == self.stop_val {
            return None;
        }

        let mut x_shift = 0;
        let mut ea = 0;
        for idx in 0..36 {
            ea = ea << 1;
            let addr_bit = (self.base_addr >> (36 - idx - 1)) & 1;
            let c = self.mask.as_bytes()[idx] as char;
            let this_bit = match c {
                '0' => addr_bit,
                '1' => 1,
                'X' => {
                    let floating_bit = (self.next_val >> x_shift) & 1;
                    x_shift += 1;
                    floating_bit
                }
                _ => panic!(),
            };
            ea = ea | this_bit;
        }
        self.next_val += 1;
        Some(ea)
    }
}


aoc::aoc!(parser, part_a, part_b, None, Some(208));
