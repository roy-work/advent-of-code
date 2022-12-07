#[allow(unused_imports)]
use aoc::prelude::*;

struct Input {
    rucksacks: Vec<Vec<char>>,
}

fn parser(input_file: &aoc::InputFile<'_>) -> anyhow::Result<Input> {
    let rucksacks = input_file.one_item_per_line(|line| Ok(line.chars().collect()))?;

    Ok(Input {
        rucksacks,
    })
}

fn item_priority(ch: char) -> i64 {
    let ich = ch as i64;
    match ch {
        'a' ..= 'z' => {
            (ich - ('a' as i64)) + 1
        }
        'A' ..= 'Z' => {
            (ich - ('A' as i64)) + 27
        }
        _ => panic!(),
    }
}

fn part_a(input: &Input) -> i64 {
    let mut ipsum = 0;
    'outer: for sack in input.rucksacks.iter() {
        let (ca, cb) = sack.split_at(sack.len() / 2);
        let cb = cb.iter().collect::<HashSet<_>>();
        for ch in ca {
            if cb.contains(ch) {
                ipsum += item_priority(*ch);
                continue 'outer;
            }
        }
        panic!()
    }
    ipsum
}

fn part_b(input: &Input) -> i64 {
    let mut ipsum = 0;
    'outer: for sacks in input.rucksacks.chunks(3) {
        let mut counts = HashMap::<char, i64>::new();
        assert!(3 == sacks.len());
        for sack in sacks {
            for ch in sack.iter().collect::<HashSet<_>>() {
                *counts.entry(*ch).or_insert(0) += 1;
            }
        }
        for (ch, count) in counts {
            if count == 3 {
                //println!("ch: {}", ch);
                ipsum += item_priority(ch);
                continue 'outer;
            }
        }
        panic!()
    }
    ipsum
}

aoc::aoc!(parser, part_a, part_b, Some(157), Some(70));

#[cfg(test)]
mod tests {
    use super::*;
}
