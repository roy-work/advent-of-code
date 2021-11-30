use std::collections::HashSet;

use anyhow::Context;
use once_cell::sync::Lazy;
use regex::Regex;

use aoc::prelude::*;

/*
static BAG_RE: Lazy<Regex> = Lazy::new(|| Regex::new("^(.+) bags contain (.+).$").unwrap());
static INNER_BAG_RE: Lazy<Regex> = Lazy::new(|| Regex::new("^(\\d+) (.+) bags?$").unwrap());
*/
/*
static INST_RE: Lazy<Regex> = Lazy::new(|| Regex::new("^(acc|jmp|nop) ([-+]\\d+)$").unwrap());

#[derive(Clone, Copy)]
enum OpCode {
    Acc,
    Jmp,
    Nop,
}

#[derive(Clone, Copy)]
struct Instruction {
    op_code: OpCode,
    arg: i32,
}

fn parse(line: &str) -> anyhow::Result<Instruction> {
    let captures = INST_RE.captures(line).ok_or_else(|| anyhow::anyhow!("no regex"))?;
    let op_code = match captures.get(1).unwrap().as_str() {
        "acc" => OpCode::Acc,
        "jmp" => OpCode::Jmp,
        "nop" => OpCode::Nop,
        _ => panic!(),
    };
    let arg = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
    Ok(Instruction {
        op_code,
        arg,
    })
}

fn run(program: &Vec<Instruction>) -> (bool, i32) {
    let mut instructions_run = HashSet::new();
    let mut ip_idx = 0;
    let mut acc = 0;
    loop {
        if program.len() == ip_idx {
            return (true, acc);
        }
        if instructions_run.contains(&ip_idx) {
            return (false, acc);
        }
        let instruction = &program[ip_idx];
        instructions_run.insert(ip_idx);
        match instruction.op_code {
            OpCode::Acc => {
                acc += instruction.arg;
                ip_idx += 1;
            },
            OpCode::Jmp => {
                ip_idx = ((ip_idx as i64) + (instruction.arg as i64)) as usize;
            }
            OpCode::Nop => ip_idx += 1,
        }
    }
}

fn main() {
    let program = aoc::file_item_per_line("input", parse).unwrap();

    let (_, acc) = run(&program);
    println!("ACC at break: {}", acc);

    for idx in 0..program.len() {
        println!("Trying index {}", idx);
        let new_op_code = match program[idx].op_code {
            OpCode::Acc => continue,
            OpCode::Jmp => OpCode::Nop,
            OpCode::Nop => OpCode::Jmp,
        };
        let mut new_program = program.clone();
        new_program[idx].op_code = new_op_code;
        let (term, acc) = run(&new_program);
        if term == true {
            println!("WINNER WINNER: {}", acc);
        }
    }
}
*/

fn parser() -> anyhow::Result<Vec<i64>> {
    aoc::file_item_per_line("input", |line| {
        line.parse::<i64>()
    }).context("failed to parse file")
}

fn is_sum_in_window(n: i64, window: &[i64]) -> bool {
    let window = window.iter().copied().collect::<HashSet<_>>();
    for a in window.iter() {
        if window.contains(&(n - a)) {
            return true;
        }
    }
    false
}

fn part_a(input: &[i64]) -> i64 {
    for idx in 25..input.len() {
        let n = input[idx];
        let window = &input[idx-25..idx];
        if !is_sum_in_window(n, window) {
            return n;
        }
    }
    panic!("failed");
}

fn part_b(input: &[i64]) -> i64 {
    let part_a_answer: i64 = 1038347917;

    for first_idx in 0..(input.len() - 2) {
        'outer: for second_idx in first_idx+2..input.len() {
            let window = &input[first_idx..second_idx];
            let mut tot = 0;
            let mut found = false;
            for n in window.iter() {
                tot += n;
                if tot == part_a_answer {
                    found = true;
                    break;
                } else if part_a_answer < tot {
                    continue 'outer;
                }
            }
            if !found {
                continue;
            }
            println!("First index: {}", first_idx);
            println!("Second index: {}", second_idx);
            println!("window: {:?}", window);
            return window.iter().min().unwrap() + window.iter().max().unwrap();
        }
    }
    panic!("failed");
}

aoc::aoc!(parser, part_a, part_b);
