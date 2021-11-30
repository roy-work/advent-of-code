use std::collections::{hash_map, VecDeque};
use std::convert::TryFrom;
use std::iter::FromIterator;

use anyhow::Context;
use once_cell::sync::Lazy;
use regex::Regex;

use aoc::prelude::*;

aoc::lazy_regex!(CLASS_RE, "^([^:]+): (\\d+)-(\\d+) or (\\d+)-(\\d+)$");

enum Op {
    Add,
    Multiply,
}

#[derive(Debug)]
enum Expression {
    Add {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Mul {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Num(i64),
}

impl Expression {
    fn eval(&self) -> i64 {
        match self {
            Expression::Add { lhs, rhs } => {
                lhs.eval() + rhs.eval()
            }
            Expression::Mul { lhs, rhs } => {
                lhs.eval() * rhs.eval()
            }
            Expression::Num(n) => *n,
        }
    }
}

enum State {
    Start,
    ParseLhsInt,
    WantOp,
    ParseRhsInt,
}

fn get_int(line: &str) -> (i64, &str) {
    for (idx, ch) in line.char_indices() {
        match ch {
            '0' ..= '9' => continue,
            _ => return (line[..idx].parse::<i64>().unwrap(), &line[idx..]),
        }
    }
    (line.parse::<i64>().unwrap(), "")
}

fn space(line: &str) -> &str {
    if !line.starts_with(' ') {
        panic!("Can't space on: {:?}", line);
    }
    &line[1..]
}

fn parse_expr(mut line: &str, depth: i16) -> (Expression, &str) {
    let mut muls = Vec::<Expression>::new();
    let mut state = State::Start;

    if line.starts_with('(') {
        let (new_lhs, rem) = parse_expr(&line[1..], depth + 1);
        muls.push(new_lhs);
        line = rem;
    } else {
        let (n, rem) = get_int(line);
        muls.push(Expression::Num(n));
        line = rem;
    }

    loop {
        println!("d = {}: loop start: {:?}", depth, line);
        if line.starts_with(')') {
            line = &line[1..];
            break;
        }

        line = space(line);
        let op = match line.chars().next().unwrap() {
            '+' => Op::Add,
            '*' => Op::Multiply,
            _ => panic!(),
        };
        line = &line[1..];
        line = space(line);

        let rhs = if line.starts_with('(') {
            let (new_rhs, rem) = parse_expr(&line[1..], depth + 1);
            line = rem;
            new_rhs
        } else {
            let (n, rem) = get_int(line);
            line = rem;
            Expression::Num(n)
        };

        let new_expr = match op {
            Op::Add => {
                let lhs = muls.pop().unwrap();
                let expr = Expression::Add {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };
                muls.push(expr);
            },
            Op::Multiply => muls.push(rhs),
        };
    }

    let mut draining_iter = muls.drain(..);
    let mut expr = draining_iter.next().unwrap();
    for mul in draining_iter {
        expr = Expression::Mul {
            lhs: Box::new(expr),
            rhs: Box::new(mul),
        };
    }

    (expr, line)
}

fn parser(path: &Path) -> anyhow::Result<Vec<Expression>> {
    Ok(aoc::file_item_per_line(path, |line| -> anyhow::Result<Expression> {
        let line = format!("{})", line);
        let (expr, _) = parse_expr(&line, 0);
        println!("Parse: {}", line);
        println!(" ->\n{:#?}", expr);
        Ok(expr)
    })?)
}

fn part_a(input: &Vec<Expression>) -> i64 {
    input.iter().map(|e| e.eval()).sum()
}

fn part_b(input: &Vec<Expression>) -> i64 {
    2
}

aoc::aoc!(parser, part_a, part_b, None, None);
