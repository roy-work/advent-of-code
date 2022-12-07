#[allow(unused_imports)]
use aoc::prelude::*;

#[derive(Debug)]
struct Input {
    crate_stacks: Vec<Vec<char>>,
    instructions: Vec<Move>,
}

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

aoc::quick_regex_parser!(parse_move, "^move (\\d+) from (\\d+) to (\\d+)$", { 1 => usize, 2 => usize, 3 => usize, });

fn parser(input_file: &aoc::InputFile<'_>) -> anyhow::Result<Input> {
    let mut lines = input_file.lines()?;

    let mut crate_lines = Vec::new();
    let crates = loop {
        let line = lines.next().unwrap()?;

        if line.starts_with(" 1 ") {
            // This is the "how many crates do we have" line:
            let crates = line.split_whitespace().last().unwrap().parse::<usize>().unwrap();
            break crates;
        } else {
            crate_lines.push(line.chars().collect::<Vec<char>>());
        }
    };

    let mut crate_stacks = Vec::new();
    for idx in 0..crates {
        crate_stacks.push(Vec::new());
        for line in crate_lines.iter().rev() {
            let item = line[idx * 4 + 1];
            if item == ' ' {
                break;
            }
            crate_stacks[idx].push(item);
        }
    }

    let blank = lines.next().unwrap()?;
    assert!(blank.is_empty());

    let mut moves = Vec::new();
    for line in lines {
        let line = line?;
        let (count, from, to) = parse_move(&line).unwrap();
        moves.push(Move {
            count,
            from,
            to,
        });
    }
    Ok(Input {
        crate_stacks,
        instructions: moves,
    })
}

fn execute_move(crates: &mut Vec<Vec<char>>, mv: &Move) {
    for _ in 0 .. mv.count {
        let item = crates[mv.from - 1].pop().unwrap();
        crates[mv.to - 1].push(item);
    }
}

fn part_a(input: &Input) -> i64 {
    let mut crates = input.crate_stacks.clone();
    for mv in input.instructions.iter() {
        execute_move(&mut crates, mv);
    }

    for stack in crates {
        print!("{}", stack.last().unwrap());
    }
    println!();
    2
}

fn execute_move_b(crates: &mut Vec<Vec<char>>, mv: &Move) {
    let mut temp = Vec::new();
    for _ in 0 .. mv.count {
        let item = crates[mv.from - 1].pop().unwrap();
        temp.push(item);
    }
    for item in temp.iter().rev().copied() {
        crates[mv.to - 1].push(item);
    }
}

fn part_b(input: &Input) -> i64 {
    let mut crates = input.crate_stacks.clone();
    for mv in input.instructions.iter() {
        execute_move_b(&mut crates, mv);
    }

    for stack in crates {
        print!("{}", stack.last().unwrap());
    }
    println!();
    2
}

aoc::aoc!(parser, part_a, part_b, Some(2), Some(4));

#[cfg(test)]
mod tests {
    use super::*;
}
