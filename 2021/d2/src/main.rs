use aoc::prelude::*;

enum Direction {
    Forward,
    Down,
    Up,
}

type Input = (Direction, i64);

fn parser(path: &Path) -> anyhow::Result<Vec<Input>> {
    Ok(aoc::file_item_per_line(path, |l| -> anyhow::Result<_> {
        let parts = l.split(' ').collect::<Vec<_>>();
        //println!("{:?}", l);
        match parts.as_slice() {
            [dstr, nstr] => {
                let dir = match *dstr {
                    "forward" => Direction::Forward,
                    "up" => Direction::Up,
                    "down" => Direction::Down,
                    _ => panic!(),
                };
                let amount: i64 = nstr.parse()?;
                Ok((dir, amount))
            }
            _ => panic!("bad line: {:?}", parts),
        }
    })?)
}

fn part_a(input: &Vec<Input>) -> i64 {
    let mut horz = 0;
    let mut depth = 0;

    for line in input {
        match line {
            (Direction::Forward, amt) => horz += amt,
            (Direction::Down, amt) => depth += amt,
            (Direction::Up, amt) => depth -= amt,
        }
    }
    horz * depth
}

fn part_b(input: &Vec<Input>) -> i64 {
    let mut horz = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input {
        match line {
            (Direction::Forward, amt) => {
                horz += amt;
                depth += aim * amt;
            }
            (Direction::Down, amt) => aim += amt,
            (Direction::Up, amt) => aim -= amt,
        }
    }
    horz * depth
}

aoc::aoc!(parser, part_a, part_b, Some(150), Some(900));
