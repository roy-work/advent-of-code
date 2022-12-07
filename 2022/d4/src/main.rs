#[allow(unused_imports)]
use aoc::prelude::*;

struct Input {
    pairings: Vec<(Range, Range)>,
}

struct Range {
    start: i64,
    end: i64,
}

aoc::quick_regex_parser!(parse_line, "(\\d+)-(\\d+),(\\d+)-(\\d+)", { 1 => i64, 2 => i64, 3 => i64, 4 => i64, });

fn parser(input_file: &aoc::InputFile<'_>) -> anyhow::Result<Input> {
    let pairs = input_file.one_item_per_line(|line| {
        let (a, b, c, d) = parse_line(&line).unwrap();
        let elf_a = Range {
            start: a,
            end: b,
        };
        let elf_b = Range {
            start: c,
            end: d,
        };
        Ok((elf_a, elf_b))
    })?;

    Ok(Input {
        pairings: pairs,
    })
}

fn is_range_in_range(small: &Range, big: &Range) -> bool {
    big.start <= small.start && small.end <= big.end
}

fn part_a(input: &Input) -> i64 {
    let mut n = 0;
    for (elf_a, elf_b) in input.pairings.iter() {
        let contained = is_range_in_range(&elf_a, &elf_b) || is_range_in_range(&elf_b, &elf_a);
        if contained {
            n += 1;
        }
    }
    n
}

fn any_overlap(a: &Range, b: &Range) -> bool {
    (b.start <= a.start && a.start <= b.end)
        || (b.start <= a.end && a.end <= b.end)
}

fn part_b(input: &Input) -> i64 {
    let mut n = 0;
    for (elf_a, elf_b) in input.pairings.iter() {
        let contained = any_overlap(&elf_a, &elf_b) || any_overlap(&elf_b, &elf_a);
        if contained {
            n += 1;
        }
    }
    n
}

aoc::aoc!(parser, part_a, part_b, Some(2), Some(4));

#[cfg(test)]
mod tests {
    use super::*;
}
