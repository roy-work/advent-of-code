use std::fmt::{self, Display};
use std::str::FromStr;

use aoc::map::Map;
use aoc::prelude::*;

type Input = Map<Tile>;

#[derive(Clone, Copy, Debug)]
enum Tile {
    Empty,
    E, // >
    S, // v
}

impl Tile {
    fn to_char(&self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::E => '>',
            Tile::S => 'v',
        }
    }
}

impl Tile {
    fn from_char(ch: char) -> Option<Tile> {
        Some(match ch {
            '.' => Tile::Empty,
            '>' => Tile::E,
            'v' => Tile::S,
            _ => return None,
        })
    }
}

fn parser(path: &Path) -> anyhow::Result<Input> {
    let reader = BufReader::new(File::open(path)?);
    let lines = reader.lines();

    let mut map = Vec::new();
    for line in lines {
        let line = line?;
        let row = line.chars().map(|c| Tile::from_char(c).unwrap()).collect::<Vec<_>>();
        map.push(row);
    }

    Ok(Map(map))
}

fn step(input: &Input) -> (bool, Input) {
    let mut new_map = Map({
        let mut m = Vec::new();
        for _ in 0..input.height() {
            let mut r = Vec::new();
            for _ in 0..input.width() {
                r.push(Tile::Empty);
            }
            m.push(r);
        }
        m
    });

    let mut did_move = false;

    for (_, r) in input.rows() {
        for (coord, t) in r {
            let new_coord = match t {
                Tile::Empty => continue,
                Tile::E => {
                    let mut new_x = coord.x + 1;
                    if input.width() <= new_x {
                        new_x = 0;
                    }
                    let mut new_coord = coord.clone();
                    new_coord.x = new_x;
                    let set_coord = if matches!(input.at(&new_coord).unwrap(), Tile::Empty) {
                        did_move = true;
                        new_coord
                    } else {
                        coord
                    };
                    *new_map.at_mut(set_coord).unwrap() = *t;
                }
                Tile::S => {
                    *new_map.at_mut(coord).unwrap() = Tile::S;
                }
            };
        }
    }

    let input = new_map;
    let mut new_map = Map({
        let mut m = Vec::new();
        for _ in 0..input.height() {
            let mut r = Vec::new();
            for _ in 0..input.width() {
                r.push(Tile::Empty);
            }
            m.push(r);
        }
        m
    });

    for (_, r) in input.rows() {
        for (coord, t) in r {
            let new_coord = match t {
                Tile::Empty => continue,
                Tile::E => {
                    *new_map.at_mut(coord).unwrap() = Tile::E;
                }
                Tile::S => {
                    let mut new_y = coord.y + 1;
                    if input.height() <= new_y {
                        new_y = 0;
                    }
                    let mut new_coord = coord.clone();
                    new_coord.y = new_y;
                    let set_coord = if matches!(input.at(&new_coord).unwrap(), Tile::Empty) {
                        did_move = true;
                        new_coord
                    } else {
                        coord
                    };
                    *new_map.at_mut(set_coord).unwrap() = *t;
                }
            };
        }
    }
    (did_move, new_map)
}

fn print_map(m: &Input) {
    for (_, r) in m.rows() {
        for (_, t) in r {
            print!("{}", t.to_char());
        }
        println!();
    }
}

fn part_a(input: &Input) -> i64 {
    let mut steps = 0;
    let mut map = input.clone();
    //print_map(&map);
    //println!();
    loop {
        let (did_move, new_map) = step(&map);
        map = new_map;
        steps += 1;
        /*
        println!("After {} steps", steps);
        print_map(&map);
        println!();
        */
        if did_move == false {
            break;
        }
        if steps > 1000 {
            panic!();
        }
    }
    steps
}

fn part_b(input: &Input) -> i64 {
    2
}

aoc::aoc!(parser, part_a, part_b, Some(58), Some(444356092776315));

#[cfg(test)]
mod tests {
    use super::*;
}
