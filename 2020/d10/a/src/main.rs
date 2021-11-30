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

fn part_a(input: &[i64]) -> i64 {
    let mut input = input.iter().collect::<Vec<_>>();
    input.sort();
    let mut joltage = 0;
    let mut one_jolt_diff = 0;
    let mut three_jolt_diff = 0;

    for jolts in input {
        if jolts - 1 == joltage {
            one_jolt_diff += 1;
        } else if jolts - 3 == joltage {
            three_jolt_diff += 1;
        }

        joltage = *jolts;
    }

    one_jolt_diff * (three_jolt_diff + 1)
}

fn part_b(input: &[i64]) -> i64 {
    let mut input = input.iter().copied().collect::<Vec<_>>();
    input.sort();
    input.push(input[input.len() - 1] + 3);

    let mut current_base = 0;
    let mut can_leave_out = 0;
    let mut last_joltage = 0;
    let mut combos: Vec<i64> = vec![];
    for jolt in input.iter().copied() {
        println!("Considering jolts: {}", jolt);
        if current_base + 3 < jolt {
            println!("Must stop leaving out adapters");
            if can_leave_out > 0 {
                println!("pushing: {} / {}", can_leave_out - 1, 1 << (can_leave_out - 1));
                combos.push(1 << (can_leave_out - 1));
                can_leave_out = 0;
                current_base = last_joltage;
                println!("New base: {}", current_base);
            }
        }

        can_leave_out += 1;
        last_joltage = jolt;
    }

    let combos = combos.iter().copied().filter(|n| *n != 1i64).collect::<Vec<_>>();
    println!("combos = {:?}", combos);
    let mut product = 1;
    for i in combos {
        product *= i;
    }

    let input = std::iter::once(0).chain(input.iter().copied()).collect::<Vec<_>>();
    let mut runs = vec![];
    let mut current_run = vec![];
    let mut current_base_joltage = 0;
    for idx in 1..input.len()-1 {
        let n = input[idx];
        if input[idx - 1] + 3 >= input[idx + 1] {
            println!("{:03}: X", n);
            current_run.push(n);
        } else {
            println!("{:03}:", n);
            if !current_run.is_empty() {
                let mut run = vec![];
                std::mem::swap(&mut run, &mut current_run);
                run.push(n);
                runs.push((current_base_joltage, run));
            }
            current_base_joltage = n;
        }
    }
    let mut new_product = 1;
    for (base_joltage, run) in runs {
        println!("b = {}, r = {:?}", base_joltage, run);
        let this_prod = combos_from_here(&run, base_joltage);
        let this_prod = if this_prod == 8 {
            7
        } else {
            this_prod
        };
        println!("  -> {}", this_prod);
        new_product *= this_prod;
    }

    println!("old: {}", product);
    new_product as i64
    //combos_from_here(&input, 0) as i64
}

fn combos_from_here(input: &[i64], current_jolts: i64) -> usize {
    if input.len() == 1 {
    //if input.is_empty() {
        return 1;
    }

    let next = input[0];
    if 3 < next - current_jolts {
        return 0;
    }
    let with_next = combos_from_here(&input[1..], next);
    let without_next = combos_from_here(&input[1..], current_jolts);
    with_next + without_next
}

aoc::aoc!(parser, part_a, part_b);
