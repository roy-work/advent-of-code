use std::collections::{hash_map, VecDeque};
use std::convert::TryFrom;
use std::iter::FromIterator;

use anyhow::Context;
use once_cell::sync::Lazy;
use regex::Regex;

use aoc::prelude::*;

#[derive(Copy, Clone)]
enum Dir {
    E,
    W,
    Ne,
    Nw,
    Se,
    Sw,
}

fn parse_line(mut line: &str) -> Vec<Dir> {
    let mut result = Vec::new();

    static dirs: &[(Dir, &str)] = &[
        (Dir::E, "e"),
        (Dir::W, "w"),
        (Dir::Ne, "ne"),
        (Dir::Nw, "nw"),
        (Dir::Se, "se"),
        (Dir::Sw, "sw"),
    ];
    'outer: while !line.is_empty() {
        for (dir, dir_name) in dirs {
            if line.starts_with(dir_name) {
                result.push(*dir);
                line = &line[dir_name.len()..];
                continue 'outer;
            }
        }
        panic!("??: {:?}", line)
    }

    result
}

type Input = Vec<Vec<Dir>>;

fn parser(path: &Path) -> anyhow::Result<Input> {
    let file = BufReader::new(File::open(path)?);
    let mut lines = file.lines();

    let mut result = Vec::new();
    for line in lines {
        let line = line?;
        result.push(parse_line(&line));
    }

    Ok(result)
}

// Grid is modeled like:
//  0   1
//   a   b   c
//    \   \
//     d   e   f
//    /   /
//   g   h   i
//    \   \   \
//     j   k   l
//
//  0   1
//   a   b   c
//    \   \
//     d   e   f
// -1   \   \
//   g   h   i
//    \   \   \
//     j   k   l

#[derive(Clone, Eq, PartialEq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

fn move_dir(coord: &Coord, dir: Dir) -> Coord {
    let (x, y) = match dir {
        Dir::E => (coord.x + 1, coord.y),
        Dir::W => (coord.x - 1, coord.y),
        Dir::Ne => (coord.x + 1, coord.y - 1),
        Dir::Nw => (coord.x, coord.y - 1),
        Dir::Se => (coord.x, coord.y + 1),
        Dir::Sw => (coord.x - 1, coord.y + 1),
    };
    Coord { x, y }
}

fn calc_floor_a(input: &Input) -> HashSet<Coord> {
    let mut black_tiles = HashSet::<Coord>::new();

    for instr in input {
        let mut coord = Coord { x: 0, y: 0 };
        for dir in instr {
            coord = move_dir(&coord, *dir);
        }
        if black_tiles.contains(&coord) {
            black_tiles.remove(&coord);
        } else {
            black_tiles.insert(coord);
        }
    }

    black_tiles
}

fn part_a(input: &Input) -> usize {
    let black_tiles = calc_floor_a(input);
    black_tiles.len()
}

fn part_b(input: &Input) -> usize {
    let mut black_tiles = calc_floor_a(input);

    for _ in 0..100 {
        let mut new_black_tiles = HashSet::new();
        let mut white_adj_black: HashMap<Coord, u8> = HashMap::new();
        for coord in black_tiles.iter() {
            let mut adj_black = 0;
            let adj_coords = adjacent(&coord);
            for adj_coord in adj_coords {
                if black_tiles.contains(&adj_coord) {
                    adj_black += 1;
                } else {
                    *white_adj_black.entry(adj_coord).or_insert(0) += 1;
                }
            }

            if !(adj_black == 0 || adj_black > 2) {
                new_black_tiles.insert(coord.clone());
            }
        }
        for (coord, adj_black) in white_adj_black.drain() {
            if adj_black == 2 {
                new_black_tiles.insert(coord);
            }
        }
        black_tiles = new_black_tiles;
    }
    black_tiles.len()
}

fn adjacent(coord: &Coord) -> Vec<Coord> {
    static all_dirs: &[Dir] = &[
        Dir::E,
        Dir::W,
        Dir::Ne,
        Dir::Nw,
        Dir::Se,
        Dir::Sw,
    ];

    all_dirs.iter().map(|d| move_dir(coord, *d)).collect()
}

aoc::aoc!(parser, part_a, part_b, Some(10), Some(2208));
