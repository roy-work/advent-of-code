use std::collections::HashSet;

use once_cell::sync::Lazy;
use regex::Regex;

use aoc::prelude::*;

/*
static BAG_RE: Lazy<Regex> = Lazy::new(|| Regex::new("^(.+) bags contain (.+).$").unwrap());
static INNER_BAG_RE: Lazy<Regex> = Lazy::new(|| Regex::new("^(\\d+) (.+) bags?$").unwrap());
*/
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
