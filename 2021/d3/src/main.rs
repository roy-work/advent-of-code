use aoc::prelude::*;

type Input = String;

fn parser(path: &Path) -> anyhow::Result<Vec<Input>> {
    Ok(aoc::file_item_per_line(path, |l| -> anyhow::Result<_> { Ok(l.to_owned()) })?)
}

fn part_a(input: &Vec<Input>) -> i64 {
    let bit_length = input[0].len();
    let mut gamma: i64 = 0;
    let mut epsilon : i64 = 0;
    for bit in 0 .. bit_length {
        let mut zeros = 0;
        for n in input.iter() {
            match n.as_bytes()[bit] {
                b'0' => zeros += 1,
                b'1' => (),
                _ => panic!(),
            }
        }
        let ones = input.len() - zeros;
        let bit_to_shift_in = if zeros < ones {
            1
        } else {
            0
        };
        gamma = (gamma << 1) | bit_to_shift_in;
        epsilon = (epsilon << 1) | (bit_to_shift_in ^ 1);
    }
    gamma * epsilon
}

fn filter<F: Fn(usize, usize) -> u8>(input: &Vec<Input>, position: usize, f: F) -> Vec<Input> {
    let mut zeros = 0;
    for bitstring in input {
        match bitstring.as_bytes()[position] {
            b'0' => zeros += 1,
            b'1' => (),
            _ => panic!(),
        }
    }
    let ones = input.len() - zeros;
    let keep = f(zeros, ones);
    let mut new_input = Vec::new();
    for bitstring in input {
        if bitstring.as_bytes()[position] == keep {
            new_input.push(bitstring.to_owned());
        }
    }
    new_input
}

fn part_b(input: &Vec<Input>) -> i64 {
    let mut current_input: Vec<Input> = input.clone();
    for pos in 0..input[0].len() {
        let new_input = filter(&current_input, pos, |z, o| {
            if z < o { b'1' }
            else if o < z { b'0' }
            else { b'1' }
        });
        current_input = new_input;
        if current_input.len() == 1 {
            break;
        }
    }
    let oxygen = i64::from_str_radix(&current_input[0], 2).unwrap();

    let mut current_input: Vec<Input> = input.clone();
    for pos in 0..input[0].len() {
        let new_input = filter(&current_input, pos, |z, o| {
            if z < o { b'0' }
            else if o < z { b'1' }
            else { b'0' }
        });
        current_input = new_input;
        if current_input.len() == 1 {
            break;
        }
    }
    let co2 = i64::from_str_radix(&current_input[0], 2).unwrap();

    oxygen * co2
}

aoc::aoc!(parser, part_a, part_b, Some(198), Some(230));
