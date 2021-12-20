use aoc::map::{FreeCoord, Map};
use aoc::prelude::*;

type Input = Map<i64>;

fn parser(path: &Path) -> anyhow::Result<Input> {
    let reader = BufReader::new(File::open(path)?);
    let mut input = Vec::new();
    for line in reader.lines() {
        let line = line?;
        input.push(line.trim_end().chars().map(|c| (c as i64) - ('0' as i64)).collect());
    }
    Ok(Map(input))
}

fn step(input: &mut Map<i64>) -> i64 {
    let mut has_flashed = HashSet::new();
    let mut needs_to_flash = HashSet::new();

    for y in 0..input.height() {
        for x in 0..input.width() {
            let cell = input.at_mut((x, y)).unwrap();
            *cell += 1;
            if 9 < *cell {
                needs_to_flash.insert((x, y));
            }
        }
    }

    loop {
        let (x, y) = {
            let v = match needs_to_flash.iter().next() {
                Some(v) => v.clone(),
                None => break,
            };
            needs_to_flash.remove(&v);
            v
        };
        has_flashed.insert((x, y));
        //println!("Processed: {:?}", (x, y));

        let coord = FreeCoord { x: x as i64, y: y as i64 }.bind(input).unwrap();
        for nearby in coord.adj_diags() {
            let cell = input.at_mut(&nearby).unwrap();
            *cell += 1;
            //println!("Cell {:?} now {}", nearby, cell);
            if 9 < *cell {
                if !has_flashed.contains(&(nearby.x, nearby.y)) {
                    needs_to_flash.insert((nearby.x, nearby.y));
                    //println!("added to needs_to_flash");
                }
            }
            //println!("</cell>");
        }
    }

    //println!("has_flashed = {:?}", has_flashed);
    for (x, y) in has_flashed.iter().copied() {
        //println!("Reset: {:?}", (x, y));
        *input.at_mut((x, y)).unwrap() = 0;
    }
    has_flashed.len() as i64
}

fn print_map(input: &Input) {
    for (_, row) in input.rows() {
        for (_, cell) in row {
            if *cell == 0 {
                print!("\x1b[1m0\x1b[0m");
            } else {
                print!("{}", cell);
            }
        }
        println!();
    }
}

fn part_a(input: &Input) -> i64 {
    let mut input = input.clone();
    let mut flashes = 0;
    //println!("Before:");
    //print_map(&input);
    for sn in 0..100 {
        flashes += step(&mut input);

        //println!("After step {}:", sn);
        //print_map(&input);
    }
    flashes
}

fn part_b(input: &Input) -> i64 {
    let mut input = input.clone();
    let desired_flashes = (input.width() * input.height()) as i64;
    //println!("Before:");
    //print_map(&input);
    for sn in 0..300 {
        let flashes = step(&mut input);
        if flashes == desired_flashes {
            return sn + 1;
        }

        //println!("After step {}:", sn);
        //print_map(&input);
    }
    panic!("Didn't solve.");
}

aoc::aoc!(parser, part_a, part_b, Some(1656), Some(195));
