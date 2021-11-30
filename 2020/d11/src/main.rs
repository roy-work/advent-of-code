use std::collections::HashSet;
use std::convert::TryFrom;

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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Floor,
    Seat(bool),
}

impl Tile {
    fn is_occupied(&self) -> bool {
        use Tile::*;
        match self {
            Floor => false,
            Seat(o) => *o,
        }
    }
}

fn parser() -> anyhow::Result<Vec<Vec<Tile>>> {
    aoc::file_item_per_line("input", |line| {
        let row = line
            .chars()
            .map(|c| match c {
                '.' => Tile::Floor,
                'L' => Tile::Seat(false),
                _ => panic!(),
            })
            .collect::<Vec<_>>();
        Ok::<_, &'static str>(row)
    })
    .context("failed to parse file")
}

fn vec_to_dims<T>(v: &Vec<Vec<T>>) -> (usize, usize) {
    (v[0].len(), v.len())
}

const OFF: &[(i64, i64)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn adj(_map: &Vec<Vec<Tile>>, coord: (usize, usize), dims: (usize, usize)) -> Vec<(usize, usize)> {
    let mut adj = vec![];

    let x = i64::try_from(coord.0).unwrap();
    let y = i64::try_from(coord.1).unwrap();

    for off in OFF {
        let new_x = x + off.0;
        let new_y = y + off.1;
        if 0 <= new_x && new_x < (dims.0 as i64) {
            if 0 <= new_y && new_y < (dims.1 as i64) {
                adj.push((
                    usize::try_from(new_x).unwrap(),
                    usize::try_from(new_y).unwrap(),
                ));
            }
        }
    }

    adj
}

fn is_on_map(coord: (i64, i64), dims: (usize, usize)) -> bool {
    if 0 <= coord.0 && coord.0 < (dims.0 as i64) {
        if 0 <= coord.1 && coord.1 < (dims.1 as i64) {
            return true;
        }
    }
    false
}

fn adj_harder(map: &Vec<Vec<Tile>>, coord: (usize, usize), dims: (usize, usize)) -> Vec<(usize, usize)> {
    let mut adj = vec![];

    let x = i64::try_from(coord.0).unwrap();
    let y = i64::try_from(coord.1).unwrap();

    for off in OFF {
        for step in 1..1000 {
            let new_x = x + off.0 * step;
            let new_y = y + off.1 * step;
            if !is_on_map((new_x, new_y), dims) {
                break;
            }
            if let Tile::Seat(_) = map[new_y as usize][new_x as usize] {
                adj.push((
                    usize::try_from(new_x).unwrap(),
                    usize::try_from(new_y).unwrap(),
                ));
                break;
            }
        }
    }

    adj
}

fn step<F: Fn(&Vec<Vec<Tile>>, (usize, usize), (usize, usize)) -> Vec<(usize, usize)>>(
    map: &Vec<Vec<Tile>>,
    adj_fn: F,
    max_occ: usize,
) -> Vec<Vec<Tile>> {
    let mut new_map = map.clone();
    let dims = vec_to_dims(map);

    for row_idx in 0..map.len() {
        for col_idx in 0..map[0].len() {
            match map[row_idx][col_idx] {
                Tile::Floor => (),
                Tile::Seat(false) => {
                    let all_empty = !adj_fn(map, (col_idx, row_idx), dims)
                        .iter()
                        .copied()
                        .any(|(c, r)| map[r][c].is_occupied());
                    let new_tile = if all_empty {
                        Tile::Seat(true)
                    } else {
                        Tile::Seat(false)
                    };
                    new_map[row_idx][col_idx] = new_tile;
                }
                Tile::Seat(true) => {
                    let occ_count = adj_fn(map, (col_idx, row_idx), dims)
                        .iter()
                        .copied()
                        .filter(|(c, r)| map[*r][*c].is_occupied())
                        .count();
                    /*
                    if row_idx == 0 && col_idx == 0 {
                        println!("occ count; result = {}", occ_count);
                        adj_fn(map, (col_idx, row_idx), dims)
                        .iter()
                        .copied()
                        .for_each(|(c, r)| {
                            println!("r = {}, c = {}, o = {}", r, c, map[r][c].is_occupied());
                        });
                    }
                    */
                    let occ = occ_count < max_occ;
                    new_map[row_idx][col_idx] = Tile::Seat(occ);
                }
            }
        }
    }

    new_map
}

fn print_map(map: &Vec<Vec<Tile>>) {
    for row in map.iter() {
        for tile in row.iter() {
            let c = match tile {
                Tile::Floor => '.',
                Tile::Seat(false) => 'L',
                Tile::Seat(true) => '#',
            };
            print!("{}", c);
        }
        println!();
    }
}

fn part_a(input: &Vec<Vec<Tile>>) -> i64 {
    let mut map_now = input.clone();
    let mut steps = 0;
    loop {
        //println!("Step");
        //print_map(&map_now);
        /*
        if steps == 3 {
            panic!();
        }
        */
        let new_map = step(&map_now, adj, 4);
        if new_map == map_now {
            break;
        }
        map_now = new_map;
        steps += 1;
    }
    println!("Done after {} steps", steps);

    let mut occ = 0;
    for row in map_now.iter() {
        for tile in row.iter() {
            if let Tile::Seat(true) = tile {
                occ += 1;
            }
        }
    }

    occ as i64
}

fn part_b(input: &Vec<Vec<Tile>>) -> i64 {
    let mut map_now = input.clone();
    let mut steps = 0;
    loop {
        /*
        println!("Step");
        print_map(&map_now);
        */
        if steps == 1000 {
            panic!();
        }
        let new_map = step(&map_now, adj_harder, 5);
        if new_map == map_now {
            break;
        }
        map_now = new_map;
        steps += 1;
    }

    let mut occ = 0;
    for row in map_now.iter() {
        for tile in row.iter() {
            if let Tile::Seat(true) = tile {
                occ += 1;
            }
        }
    }

    occ as i64
}

aoc::aoc!(parser, part_a, part_b);
