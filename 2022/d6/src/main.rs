#[allow(unused_imports)]
use aoc::prelude::*;

type Input = String;

fn parser(input_file: &aoc::InputFile<'_>) -> anyhow::Result<Input> {
    Ok(input_file.single_line_input()?)
}

fn find_start_marker(input: &str, unique_len: usize) -> i64 {
    let mut buffer = VecDeque::new();
    for (idx, ch) in input.chars().enumerate() {
        buffer.push_back(ch);
        if buffer.len() > unique_len {
            buffer.pop_front();
        }
        let mut counts = HashMap::<char, usize>::new();
        for ch in buffer.iter().copied() {
            *counts.entry(ch).or_insert(0) += 1;
        }
        if aoc::histogram(buffer.iter().copied()).len() == unique_len {
            return (idx as i64) + 1;
        }
    }
    panic!()
}

fn part_a(input: &Input) -> i64 {
    find_start_marker(&input, 4)
}

fn part_b(input: &Input) -> i64 {
    find_start_marker(&input, 14)
}

aoc::aoc!(parser, part_a, part_b, Some(7), Some(19));

#[cfg(test)]
mod tests {
    use super::*;
}
