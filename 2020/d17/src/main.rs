use std::collections::{hash_map, VecDeque};
use std::convert::TryFrom;
use std::iter::FromIterator;

use anyhow::Context;
use once_cell::sync::Lazy;
use regex::Regex;

use aoc::prelude::*;

aoc::lazy_regex!(CLASS_RE, "^([^:]+): (\\d+)-(\\d+) or (\\d+)-(\\d+)$");

#[derive(Clone)]
struct Map(HashSet<Coord>);

impl Map {
    fn is_active(&self, coord: &Coord) -> bool {
        self.0.contains(&coord)
    }

    fn activate(&mut self, coord: Coord) {
        self.0.insert(coord);
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Coord {
    fn adjacent(&self) -> Vec<Coord> {
        let mut adj = Vec::new();
        for x_off in -1..=1 {
            for y_off in -1..=1 {
                for z_off in -1..=1 {
                    for w_off in -1..=1 {
                        if x_off == 0 && y_off == 0 && z_off == 0 && w_off == 0 {
                            continue;
                        }
                        adj.push(Coord {
                            x: self.x + x_off,
                            y: self.y + y_off,
                            z: self.z + z_off,
                            w: self.w + w_off,
                        });
                    }
                }
            }
        }
        adj
    }
}

fn parser(path: &Path) -> anyhow::Result<Map> {
    let file = BufReader::new(File::open(path)?);

    let mut map = Map(HashSet::new());
    for (y, line) in file.lines().enumerate() {
        let line = line?;
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '.' => (),
                '#' => {
                    map.activate(Coord {
                        x: x as i64,
                        y: y as i64,
                        z: 0,
                        w: 0,
                    });
                }
                _ => panic!(),
            }
        }
    }

    Ok(map)
}

fn part_a(input: &Map) -> i64 {
    let mut map: Map = input.clone();
    for cycle in 0..6 {
        map = single_cycle(&map);
        println!("== after CYCLE {} ==", cycle);
        //print_map(&map);
    }

    map.0.len() as i64
}

/*
fn print_map(map: &Map) {
    let min_x = map.0.iter().map(|c| c.x).min().unwrap();
    let max_x = map.0.iter().map(|c| c.x).max().unwrap();
    let min_y = map.0.iter().map(|c| c.y).min().unwrap();
    let max_y = map.0.iter().map(|c| c.y).max().unwrap();
    let min_z = map.0.iter().map(|c| c.z).min().unwrap();
    let max_z = map.0.iter().map(|c| c.z).max().unwrap();

    for z in min_z..=max_z {
        println!("z={}", z);
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let active = map.is_active(&Coord { x, y, z });
                let ch = if active { '#' } else { '.' };
                print!("{}", ch);
            }
            println!();
        }
        println!();
    }
}
*/

fn single_cycle(input: &Map) -> Map {
    let mut output = Map(HashSet::new());

    let mut empty_cells = HashMap::<Coord, i64>::new();

    for active_cell in input.0.iter() {
        let adj = active_cell.adjacent();
        let mut nearby_active = 0;
        for cell in adj {
            if input.is_active(&cell) {
                nearby_active += 1;
            } else {
                *empty_cells.entry(cell).or_insert(0) += 1;
            }
        }

        if nearby_active == 2 || nearby_active == 3 {
            output.activate(active_cell.clone());
        }
    }

    for (inactive_cell, active_neighbors) in empty_cells.iter() {
        if *active_neighbors == 3 {
            output.activate(inactive_cell.clone());
        }
    }

    output
}

fn part_b(input: &Map) -> i64 {
    2
}

aoc::aoc!(parser, part_a, part_b, Some(848), None);
