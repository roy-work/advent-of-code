#[allow(unused_imports)]
use aoc::prelude::*;

struct Input {
    elves: Vec<Vec<i64>>,
}

fn parser(input_file: &aoc::InputFile<'_>) -> anyhow::Result<Input> {
    let mut current_elf: Option<Vec<i64>> = None;
    let mut input = Input {
        elves: Vec::new(),
    };
    for line in input_file.lines()? {
        let line = line?;
        if line == "" {
            let elf = current_elf.take().unwrap();
            input.elves.push(elf);
        } else {
            let cals: i64 = line.parse()?;
            current_elf.get_or_insert_with(Vec::new).push(cals);
        }
    }
    if let Some(v) = current_elf.take() {
        input.elves.push(v);
    }

    Ok(input)
}

fn part_a(input: &Input) -> i64 {
    input.elves.iter().map(|v| v.iter().sum()).max().unwrap()
}

fn part_b(input: &Input) -> i64 {
    let mut elves = input.elves.clone();
    elves.sort_by_key::<i64, _>(|v| v.iter().sum::<i64>());
    let slice = &elves[elves.len() - 3 ..];
    slice.iter().map(|v| v.iter().sum::<i64>()).sum::<i64>()
}

aoc::aoc!(parser, part_a, part_b, Some(24000), Some(45000));

#[cfg(test)]
mod tests {
    use super::*;
}
