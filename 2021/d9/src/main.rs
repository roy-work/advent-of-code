use aoc::prelude::*;
use aoc::map::{Coord, Map};

type Input = Map<i64>;

fn parser(path: &Path) -> anyhow::Result<Input> {
    let mut reader = BufReader::new(File::open(path)?);
    let mut input: Vec<Vec<i64>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let mut row = Vec::new();
        for c in line.trim_end().chars() {
            match c {
                '0' ..= '9' => {
                    let height = (c as i64) - ('0' as i64);
                    row.push(height);
                }
                _ => panic!(),
            }
        }
        input.push(row);
    }
    Ok(Map(input))
}

fn adj_tiles(input: &Input, center: Coord) -> Vec<Coord> {
    const OFFSETS: &[(i64, i64)] = &[
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
    ];

    let mut results = Vec::new();
    for offset in OFFSETS {
        let new_x = (center.x as i64) + offset.0;
        let new_y = (center.y as i64) + offset.1;

        if new_x < 0 || new_y < 0 {
            continue;
        }
        let new_x = new_x as usize;
        let new_y = new_y as usize;
        if input.0.len() <= new_y || input.0[0].len() <= new_x {
            continue;
        }
        results.push(Coord {
            x: new_x,
            y: new_y,
        });
    }
    results
}

fn part_a(input: &Input) -> i64 {
    let mut low_points = Vec::new();
    for (y, row) in input.0.iter().enumerate() {
        'looking: for (x, cell) in row.iter().enumerate() {
            let center = Coord { x, y };
            let adj = adj_tiles(input, center);
            for adj_coord in adj {
                if input.at(adj_coord.x, adj_coord.y).unwrap() <= cell {
                    continue 'looking;
                }
            }
            //println!("Low point at line {}, col {}, val = {}", y+1, x+1, cell);
            low_points.push(1 + cell);
        }
    }
    low_points.iter().sum()
}

fn find_basin_size(input: &Input, basin_start: Coord) -> i64 {
    let mut visited = HashSet::<Coord>::new();
    let mut tiles = 0;
    let mut need_to_scan = Vec::new();
    visited.insert(basin_start);
    need_to_scan.push(basin_start);

    while let Some(coord) = need_to_scan.pop() {
        tiles += 1;
        let adj = adj_tiles(input, coord);
        for adj_coord in adj {
            if !visited.contains(&adj_coord) {
                visited.insert(adj_coord);
                if *input.at(adj_coord.x, adj_coord.y).unwrap() < 9 {
                    need_to_scan.push(adj_coord);
                }
            }
        }
    }
    tiles
}

fn part_b(input: &Input) -> i64 {
    let mut low_points = Vec::new();
    for (y, row) in input.0.iter().enumerate() {
        'looking: for (x, cell) in row.iter().enumerate() {
            let center = Coord { x, y };
            let adj = adj_tiles(input, center);
            for adj_coord in adj {
                if input.at(adj_coord.x, adj_coord.y).unwrap() <= cell {
                    continue 'looking;
                }
            }
            //println!("Low point at line {}, col {}, val = {}", y+1, x+1, cell);
            low_points.push(center);
        }
    }

    let mut basin_sizes = Vec::new();

    for low_point in low_points {
        let size = find_basin_size(input, low_point);
        basin_sizes.push(size);
    }
    basin_sizes.sort();
    basin_sizes.iter().rev().take(3).product()
}

aoc::aoc!(parser, part_a, part_b, Some(15), Some(1134));
