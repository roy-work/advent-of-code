use std::collections::HashSet;
use std::convert::TryFrom;

use anyhow::Context;
use once_cell::sync::Lazy;
use regex::Regex;

use aoc::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Instruction {
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    Right(i64),
    Left(i64),
    Forward(i64),
}

fn parser() -> anyhow::Result<Vec<Instruction>> {
    aoc::file_item_per_line("input", |line| -> Result<_, std::num::ParseIntError> {
        let code = line.chars().next().unwrap();
        let num = (&line[1..]).parse::<u32>()? as i64;
        use Instruction::*;
        Ok(match code {
            'N' => North(num),
            'S' => South(num),
            'E' => East(num),
            'W' => West(num),
            'R' => Right(num),
            'L' => Left(num),
            'F' => Forward(num),
            _ => panic!(),
        })
    }).context("failed to parse input")
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Coord {
    x: i64,
    y: i64,
}

enum Dir {
    North,
    South,
    East,
    West,
}

impl Coord {
    fn do_move(&self, dir: Dir, amount: i64) -> Coord {
        use Dir::*;
        match dir {
            North => Coord { x: self.x, y: self.y + amount },
            South => Coord { x: self.x, y: self.y - amount },
            East => Coord { x: self.x + amount, y: self.y },
            West => Coord { x: self.x - amount, y: self.y },
        }
    }
}

fn do_move(loc: Coord, mut ship_dir: i64, instr: Instruction) -> (Coord, i64) {
    use Instruction::*;
    let (m_dir, m_amt) = match instr {
        North(a) => (Dir::North, a),
        South(a) => (Dir::South, a),
        East(a) => (Dir::East, a),
        West(a) => (Dir::West, a),
        Right(a) => {
            ship_dir = (360 + ship_dir + a) % 360;
            (Dir::North, 0)
        },
        Left(a) => {
            ship_dir = (360 + ship_dir - a) % 360;
            (Dir::North, 0)
        }
        Forward(a) => {
            match ship_dir {
                0 => (Dir::North, a),
                90 => (Dir::East, a),
                180 => (Dir::South, a),
                270 => (Dir::West, a),
                _ => panic!(),
            }
        }
    };

    let new_coord = loc.do_move(m_dir, m_amt);
    (new_coord, ship_dir)
}

fn part_a(input: &Vec<Instruction>) -> i64 {
    let mut ship_coord = Coord { x: 0, y: 0 };
    let mut ship_dir = 90;
    for instr in input {
        println!("At: {:?}, facing: {:?}", ship_coord, ship_dir);
        println!("Doing: {:?}", instr);
        let (c, d) = do_move(ship_coord, ship_dir, *instr);
        ship_coord = c;
        ship_dir = d;
    }
    println!("At: {:?}, facing: {:?}", ship_coord, ship_dir);
    abs(ship_coord.x) + abs(ship_coord.y)
}

fn abs(n: i64)->i64 {
    if n < 0 {
        -n
    } else { n}

}

fn part_b(input: &Vec<Instruction>) -> i64 {
    let mut ship_coord = Coord { x: 0, y: 0 };
    let mut waypoint = Coord { x: 10, y: 1 };
    for instr in input {
        let (c, w) = do_part2_move(ship_coord, waypoint, *instr);
        ship_coord = c;
        waypoint = w;
    }
    abs(ship_coord.x) + abs(ship_coord.y)
}

fn do_part2_move(mut ship_coord: Coord, mut waypoint: Coord, instr: Instruction) -> (Coord, Coord) {
    use Instruction::*;
    let (d, a) = match instr {
        North(a) => (Dir::North, a),
        South(a) => (Dir::South, a),
        East(a) => (Dir::East, a),
        West(a) => (Dir::West, a),
        Left(a) => {
            match a {
                90 => {
                    waypoint = Coord {
                        x: -waypoint.y,
                        y: waypoint.x,
                    };
                },
                180 => {
                    waypoint = Coord {
                        x: -waypoint.x,
                        y: -waypoint.y,
                    };
                },
                270 => {
                    // So we got smarter here...
                    waypoint = Coord {
                        x: -waypoint.y,
                        y: waypoint.x,
                    };
                    waypoint = Coord {
                        x: -waypoint.x,
                        y: -waypoint.y,
                    };
                },
                _ => panic!(),
            }
            (Dir::North, 0)
        }
        Right(a) => {
            match a {
                90 => {
                    waypoint = Coord {
                        x: waypoint.y,
                        y: -waypoint.x,
                    };
                }
                180 => {
                    waypoint = Coord {
                        x: waypoint.y,
                        y: -waypoint.x,
                    };
                    waypoint = Coord {
                        x: waypoint.y,
                        y: -waypoint.x,
                    };
                }
                270 => {
                    // ... and now we're big braining it.
                    // "Also known as 'lazy brain'." — fiancée
                    waypoint = Coord {
                        x: waypoint.y,
                        y: -waypoint.x,
                    };
                    waypoint = Coord {
                        x: waypoint.y,
                        y: -waypoint.x,
                    };
                    waypoint = Coord {
                        x: waypoint.y,
                        y: -waypoint.x,
                    };
                }
                _ => panic!(),
            }
            (Dir::North, 0)
        }
        Forward(a) => {
            ship_coord = Coord {
                x: ship_coord.x + waypoint.x * a,
                y: ship_coord.y + waypoint.y * a,
            };
            (Dir::North, 0)
        }
    };

    let waypoint = waypoint.do_move(d, a);
    (ship_coord, waypoint)
}

aoc::aoc!(parser, part_a, part_b);
