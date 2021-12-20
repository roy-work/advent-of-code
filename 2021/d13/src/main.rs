use aoc::prelude::*;
use aoc::hot_parse;

struct Input {
    dots: Vec<(i64, i64)>,
    folds: Vec<(Axis, i64)>,
}

#[derive(Clone, Copy, Debug)]
enum Axis {
    X,
    Y,
}

fn parser(path: &Path) -> anyhow::Result<Input> {
    let reader = BufReader::new(File::open(path)?);
    let mut lines = reader.lines();

    let mut dots = Vec::new();
    while let Some(line) = lines.next() {
        let line = line?;
        if line.trim_end() == "" {
            break;
        }
        let (a, b) = line.trim_end().split_once(',').unwrap();
        let a: i64 = a.parse()?;
        let b: i64 = b.parse()?;
        dots.push((a, b));
    }

    let mut folds = Vec::new();
    hot_parse!(fparse, "^fold along ([xy])=([0-9]+)$", { 1 => String, 2 => i64, }, |t| t);
    for line in lines {
        let line = line?;
        let (axis, n) = fparse(&line).unwrap();
        let axis = match axis.as_str() {
            "x" => Axis::X,
            "y" => Axis::Y,
            _ => panic!(),
        };
        folds.push((axis, n));
    }
    Ok(Input { dots, folds })
}

fn fold(dots: &[(i64, i64)], fold: (Axis, i64)) -> Vec<(i64 ,i64)> {
    let (axis, amt) = fold;
    let mut new_dots = Vec::new();
    for dot in dots.iter().copied() {
        let new_dot = match axis {
            Axis::X => {
                if dot.0 < amt {
                    dot
                } else if amt < dot.0 {
                    let dist = dot.0 - amt;
                    let new_x = amt - dist;
                    (new_x, dot.1)
                } else {
                    panic!()
                }
            },
            Axis::Y => {
                if dot.1 < amt {
                    dot
                } else if amt < dot.1 {
                    let dist = dot.1 - amt;
                    let new_y = amt - dist;
                    (dot.0, new_y)
                } else {
                    panic!()
                }
            }
        };
        new_dots.push(new_dot);
    }
    new_dots
}

fn render_dots(dots: &[(i64, i64)]) {
    if dots.is_empty() {
        println!("No dots.");
    }
    let width = dots.iter().map(|(x, _)| x).max().unwrap() + 1;
    let height = dots.iter().map(|(_, y)| y).max().unwrap() + 1;
    let dots = dots.iter().collect::<HashSet<_>>();
    for y in 0..height {
        for x in 0..width {
            if dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part_a(input: &Input) -> i64 {
    let mut current_dots: Vec<(i64, i64)> = input.dots.clone();
    for a_fold in input.folds.iter().take(1) {
        current_dots = fold(&current_dots, *a_fold);
    }
    let current_dots = current_dots.iter().copied().collect::<HashSet<_>>();
    render_dots(current_dots.iter().copied().collect::<Vec<_>>().as_slice());
    current_dots.len() as i64
}

fn part_b(input: &Input) -> i64 {
    let mut current_dots: Vec<(i64, i64)> = input.dots.clone();
    for a_fold in input.folds.iter() {
        current_dots = fold(&current_dots, *a_fold);
    }
    let current_dots = current_dots.iter().copied().collect::<HashSet<_>>();
    render_dots(current_dots.iter().copied().collect::<Vec<_>>().as_slice());
    current_dots.len() as i64
}

aoc::aoc!(parser, part_a, part_b, Some(17), Some(36));
