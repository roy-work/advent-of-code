use std::collections::{hash_map, VecDeque};
use std::convert::TryFrom;
use std::iter::FromIterator;

use anyhow::Context;
use once_cell::sync::Lazy;
use regex::Regex;

use aoc::prelude::*;

aoc::lazy_regex!(TILE_RE, "^Tile (\\d+):$");

type Input = HashMap<i64, Tile>;

struct Tile {
    id: i64,
    data: Vec<Vec<char>>,
}

fn parser(path: &Path) -> anyhow::Result<Input> {
    let file = BufReader::new(File::open(path)?);
    let mut lines = file.lines();

    let mut tiles = Input::new();
    loop {
        let tile_line = match lines.next() {
            Some(l) => l?,
            None => break,
        };
        let captures = TILE_RE.captures(&tile_line).unwrap();
        let tile_id = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();

        let mut data = Vec::new();
        loop {
            let line = lines.next().unwrap()?;
            if line.is_empty() {
                break;
            }

            data.push(line.chars().collect::<Vec<_>>());
        }

        tiles.insert(tile_id, Tile { id: tile_id, data });
    }

    Ok(tiles)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Dir {
    Top,
    Bottom,
    Left,
    Right,
}

impl Dir {
    fn opposite_side(self) -> Dir {
        use Dir::*;
        match self {
            Left => Right,
            Right => Left,
            Top => Bottom,
            Bottom => Top,
        }
    }

    fn number(self) -> i64 {
        use Dir::*;
        match self {
            Top => 0,
            Right => 1,
            Bottom => 2,
            Left => 3,
        }
    }

    fn from_number(n: i64) -> Self {
        use Dir::*;
        match n {
            0 => Top,
            1 => Right,
            2 => Bottom,
            3 => Left,
            _ => panic!(),
        }
    }
}

struct Side(Vec<char>);

impl Side {
    fn is_match(&self, other: &Side) -> Option<bool> {
        if self.0 == other.0 {
            Some(false)
        } else if self.0 == other.0.iter().copied().rev().collect::<Vec<_>>() {
            Some(true)
        } else {
            None
        }
    }
}

fn full_square_dim(tile_count: usize) -> usize {
    let dim = (tile_count as f32).sqrt() as usize;
    assert!(dim * dim == tile_count);
    dim
}

fn compute_sides(tile: &Tile) -> Vec<(Dir, Side)> {
    let mut result = vec![];

    result.push((Dir::Top, tile.data[0].clone()));
    result.push((Dir::Bottom, tile.data[tile.data.len() - 1].clone()));

    result.push((
        Dir::Left,
        tile.data.iter().map(|r| r[0]).collect::<Vec<char>>(),
    ));
    result.push((
        Dir::Right,
        tile.data.iter().map(|r| r[r.len() - 1]).collect::<Vec<char>>(),
    ));

    result.drain(..).map(|(d, s)| (d, Side(s))).collect()
}

fn part_a(input: &Input) -> i64 {
    let mut connections = HashMap::<(i64, Dir), (i64, Dir, bool)>::new();
    let mut unmatched_tile_sides = HashMap::<(i64, Dir), Side>::new();

    for tile in input.values() {
        for (dir, side) in compute_sides(tile) {
            println!("{}/{:?}: {:?}", tile.id, dir, side.0.iter().collect::<String>());
            unmatched_tile_sides.insert((tile.id, dir), side);
        }
    }

    let dim = full_square_dim(input.len());
    let needed_matches = (dim - 1) * dim * 2;
    println!("Input is {0}x{0}", dim);
    println!("Need to match: {}", needed_matches);
    let mut matches_made = 0;

    while matches_made < needed_matches {
        let mut matched: Option<(i64, Dir, i64, Dir, bool)> = None;
        'outer: for ((id, dir), side) in unmatched_tile_sides.iter() {
            /*
            if *dir == Dir::Top || *dir == Dir::Left {
                continue;
            }
            */
            /*
            let opposite_dir = dir.opposite_side();
            let candidate_tiles = unmatched_tile_sides
                .iter()
                .filter(|((_, d), _)| *d == opposite_dir);
            */
            let candidate_tiles = unmatched_tile_sides
                .iter()
                .filter(|((i, d), _)| (i, d) != (id, dir));

            let mut matching_ids = Vec::<(i64, Dir, bool)>::new();
            for ((cid, cdir), cside) in candidate_tiles {
                if let Some(flipped) = cside.is_match(side) {
                    matching_ids.push((*cid, *cdir, flipped));
                }
            }

            match matching_ids.as_slice() {
                [(cid, cdir, flipped)] => {
                    matched = Some((*id, *dir, *cid, *cdir, *flipped));
                    matches_made += 1;
                    break;
                }
                [] => {
                    println!("No matches for {} {:?}", id, dir);
                }
                _ => {
                    println!("Multiple matches for {} {:?}", id, dir);
                }
            }
        }
        let matched = matched.expect("didnt match");
        println!("Matched: {:?}", matched);
        let (id, dir, cid, cdir, flipped) = matched;
        unmatched_tile_sides.remove(&(id, dir));
        unmatched_tile_sides.remove(&(cid, cdir));
        connections.insert((id, dir), (cid, cdir, flipped));
        connections.insert((cid, cdir), (id, dir, flipped));
    }

    let mut assigned_coords = HashMap::<i64, (i64, i64)>::new();
    let mut map = HashMap::<(i64, i64), i64>::new();

    let start = input.keys().copied().next().unwrap();
    assigned_coords.insert(start, (0, 0));
    map.insert((0, 0), start);

    let mut explores = VecDeque::new();
    explores.push_back(((0, -1), (start, Dir::Top), Dir::Bottom));
    explores.push_back(((0, 1), (start, Dir::Bottom), Dir::Top));
    explores.push_back(((-1, 0), (start, Dir::Left), Dir::Right));
    explores.push_back(((1, 0), (start, Dir::Right), Dir::Left));

    let mut cycles = 0;
    println!("Start tile: {}", start);
    while let Some((coord, key_conn, enter)) = explores.pop_front() {
        println!("Exploring: {:?}, {:?}, {:?}", coord, key_conn, enter);
        if map.contains_key(&coord) {
            continue;
        }

        let (cid, cdir, flipped) = match connections.get(&key_conn) {
            Some(conn) => conn,
            None => continue,
        };
        let flipped = *flipped;

        println!("{} on the {:?} connects to {}", key_conn.0, enter.opposite_side(), cid);

        let rotate = enter.number() - cdir.number();
        let dir_number = if flipped { rotate } else { -rotate };
        let top_dir = Dir::from_number((8 + dir_number) % 4);

        static OFFSETS: &[(i64, i64, Dir, i64)] = &[
            (0, -1, Dir::Bottom, 0),
            (-1, 0, Dir::Left, 1),
            (0, 1, Dir::Top, 2),
            (-1, 0, Dir::Right, 3),
        ];

        for (dx, dy, new_enter, dir_off) in OFFSETS {
            let new_dir_number = top_dir.number() + {
                if flipped { -dir_off } else { *dir_off }
            };
            let new_dir = Dir::from_number((4 + new_dir_number) % 4);
            let new_coord = (coord.0 + dx, coord.1 + dy);
            explores.push_back((new_coord, (*cid, new_dir), *new_enter));
        }

        cycles += 1;
        if cycles == 5 {
            panic!();
        }

        //map.insert(coord, *cid);
        //for (coord, enter)
        //explores.push(
    }
    /*
    let mut upper_left_corner: Option<i64> = None;
    for id in input.keys().copied() {
        if !connections.contains_key(&(id, Dir::Bottom)) || !connections.contains_key(&(id, Dir::Right)) {
            continue;
        }
        if connections.values().any(|(v, _)| *v == id) {
            continue;
        }
        match upper_left_corner {
            Some(_) => panic!(),
            None => upper_left_corner = Some(id),
        }
    }
    let upper_left_corner = upper_left_corner.unwrap();

    let mut layout: Vec<Vec<i64>> = Vec::new();
    */

    1
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Enter {
    Top,
    Bottom,
    Left,
    Right,
}

impl Enter {
    fn opposite(self) -> Enter {
        use Enter::*;
        match self {
            Top => Bottom,
            Bottom => Top,
            Left => Right,
            Right => Left,
        }
    }
}

fn from_here(coord: (i64, i64), last_enter: Enter) -> Vec<((i64, i64), Enter)> {
    let mut result = vec![];
    if last_enter != Enter::Top {
        result.push(((coord.0, coord.1 - 1), Enter::Bottom));
    }
    if last_enter != Enter::Bottom {
        result.push(((coord.0, coord.1 + 1), Enter::Top));
    }
    if last_enter != Enter::Left {
        result.push(((coord.0 - 1, coord.1), Enter::Right));
    }
    if last_enter != Enter::Right{
        result.push(((coord.0 + 1, coord.1), Enter::Left));
    }
    result
}

fn part_b(input: &Input) -> i64 {
    2
}

aoc::aoc!(parser, part_a, part_b, Some(20899048083289), None);
