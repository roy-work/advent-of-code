use aoc::prelude::*;
use aoc::map::{Coord, Map};

type Line = (Coord, Coord);
type Input = Vec<Line>;

fn parser(path: &Path) -> anyhow::Result<Input> {
    Ok(aoc::file_item_per_line(path, |l| -> anyhow::Result<_> {
        aoc::hot_parse!(pline, "([0-9]+),([0-9]+) -> ([0-9]+),([0-9]+)", { 1 => usize, 2 => usize, 3 => usize, 4 => usize, }, |t| t);
        let (x1, y1, x2, y2) = pline(l).unwrap();
        Ok((
            Coord { x: x1, y: y1 },
            Coord { x: x2, y: y2 },
        ))
    })?)
}

fn map_for_lines(lines: &Input) -> Map<i64> {
    let mut max_x = 0;
    let mut max_y = 0;

    use std::cmp::max;

    for line in lines {
        max_x = max(max_x, max(line.0.x, line.1.x));
        max_y = max(max_y, max(line.0.y, line.1.y));
    }

    let mut map: Map<_> = Map(Vec::new());
    for _ in 0 ..= max_y {
        map.0.push([0].repeat(max_x + 1));
    }

    map
}

fn render_line(map: &mut Map<i64>, line: &Line, do_diag: bool) {
    use std::cmp::{min, max};

    let x1 = min(line.0.x, line.1.x);
    let x2 = max(line.0.x, line.1.x);
    let y1 = min(line.0.y, line.1.y);
    let y2 = max(line.0.y, line.1.y);

    if line.0.x == line.1.x {
        // vert
        for y in y1 ..= y2 {
            *map.at_mut(line.0.x, y).unwrap() += 1;
        }
    } else if line.0.y == line.1.y {
        // horz
        for x in x1 ..= x2 {
            *map.at_mut(x, line.0.y).unwrap() += 1;
        }
    } else if do_diag {
        // diag
        let x1 = line.0.x as i64;
        let y1 = line.0.y as i64;
        let x2 = line.1.x as i64;
        let y2 = line.1.y as i64;

        let dist = (x2 - x1).abs();
        let x_pos = x2 > x1;
        let y_pos = y2 > y1;
        for d in 0 ..= dist {
            let x = x1 + d * (if x_pos { 1 } else { -1 });
            let y = y1 + d * (if y_pos { 1 } else { -1 });
            *map.at_mut(x.try_into().unwrap(), y.try_into().unwrap()).unwrap() += 1;
        }
    }
}

fn render_map(map: &Map<i64>) {
    map.render_single_char(|c| {
        match c {
            0 => '.',
            n => char::from(b'0' + u8::try_from(*n).unwrap()),
        }
    });
}

fn part_a(input: &Input) -> i64 {
    let mut map = map_for_lines(input);
    for line in input {
        render_line(&mut map, line, false);
        //println!();
        //render_map(&map);
    }
    let mut count = 0;
    for tile in map.iter_tiles() {
        if 1 < *tile {
            count += 1;
        }
    }
    count
}

fn part_b(input: &Input) -> i64 {
    let mut map = map_for_lines(input);
    for line in input {
        render_line(&mut map, line, true);
        //println!();
        //render_map(&map);
    }
    let mut count = 0;
    for tile in map.iter_tiles() {
        if 1 < *tile {
            count += 1;
        }
    }
    count
}

aoc::aoc!(parser, part_a, part_b, Some(5), Some(12));
