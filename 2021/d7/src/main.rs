use aoc::prelude::*;

type Input = Vec<i64>;

fn parser(path: &Path) -> anyhow::Result<Input> {
    Ok(aoc::comma_try_split(&aoc::single_line_input(path)?, |p| p.parse())?)
}

fn calc_dist(input: &Input, selected_pos: i64) -> i64 {
    input.iter().map(|crab| (crab - selected_pos).abs()).sum()
}

fn part_a(input: &Input) -> i64 {
    let mut input = input.clone();
    input.sort();

    let selected_index = input.len() / 2;
    println!("median = {}", calc_dist(&input, input[selected_index]));
    println!("median = {} + 1", calc_dist(&input, input[selected_index + 1]));
    println!("median = {} - 1", calc_dist(&input, input[selected_index - 1]));

    calc_dist(&input, input[selected_index])
}

fn calc_dist_b(input: &Input, selected_pos: i64) -> i64 {
    input
        .iter()
        .map(|crab| (crab - selected_pos).abs())
        .map(|moves| moves * (moves + 1) / 2)
        .sum()
}

fn part_b(input: &Input) -> i64 {
    /*
    let mut input_counts = HashMap::<i64, i64>::new();
    for crab in input.iter().copied() {
        *input_counts.entry(crab).or_insert(0) += 1;
    }

    let mode_pos = *input_counts.iter().max_by_key(|(_, c)| *c).unwrap().0;
    */
    let average = (input.iter().sum::<i64>() as f32) / (input.len() as f32);
    let average = average.round() as i64;

    let mut min_pos = None;

    fn if_min(value: i64, save_to: &mut Option<i64>) {
        match save_to {
            None => *save_to = Some(value),
            Some(v) => {
                if value < *v {
                    *save_to = Some(value)
                }
            }
        }
    }

    for offset in 0 .. 100 {
        let v = calc_dist_b(&input, average + offset);
        if_min(v, &mut min_pos);
        let v = calc_dist_b(&input, average - offset);
        if_min(v, &mut min_pos);
    }

    min_pos.unwrap()
}

aoc::aoc!(parser, part_a, part_b, Some(37), Some(168));
