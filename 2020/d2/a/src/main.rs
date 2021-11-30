use std::collections::HashSet;
use aoc::prelude::*;

use once_cell::sync::Lazy;
use regex::Regex;

fn main() {
    let items = aoc::file_item_per_line("input", parse_line).unwrap_or_else(|err| {
        panic!("failed to load input");
    });

    let mut valid = 0;
    for (lo, hi, ch, pass) in items {
        if is_valid_2(&pass, lo, hi, ch) {
            valid += 1;
        }
    }
    println!("Valid: {}", valid);
    /*
    let numbers = aoc::file_o_numbers("input").unwrap();

    let numbers = numbers.iter().map(|n| *n).collect::<HashSet<_>>();

    //let mut the_match = None;
    for a in numbers.iter() {
        for b in numbers.iter() {
            let c = 2020 - *a - *b;
            if numbers.contains(&c) {
                println!("-> {}", a * b * c);
                return;
            }
            /*
            for c in numbers.iter() {
                if a + b + c == 2020 {
                    the_match = Some((a, b, c));
                    break 'outer;
                }
            }
            */
        }
    }

    //let (a, b, c) = the_match.unwrap();

    //println!("-> {}", a * b * c);
    */
}

static LINE_RE: Lazy<Regex> = Lazy::new(|| Regex::new("(\\d+)-(\\d+) (.): (.*)$").unwrap());

fn parse_line(line: &str) -> Result<(u32, u32, char, String), &'static str> {
    let capture = LINE_RE.captures(line).ok_or("regex failed")?;
    let lo = capture.get(1).unwrap().as_str().parse::<u32>().map_err(|_| "lo bad")?;
    let hi = capture.get(2).unwrap().as_str().parse::<u32>().map_err(|_| "hi bad")?;
    let c = capture.get(3).unwrap().as_str().chars().next().unwrap();
    let pass = capture.get(4).unwrap().as_str().to_string();
    Ok((lo, hi, c, pass))
}

//aoc::hot_parse!(parse_line, LINE_RE, { 1 => u32 , 2 => u32, 3 => char, 4 => String , }, rrr);

fn is_valid(pass: &str, lo: u32, hi: u32, ch: char) -> bool {
    let count = pass.chars().filter(|c| *c == ch).count() as u32;
    lo <= count && count <= hi
}

fn is_valid_2(pass: &str, lo: u32, hi: u32, ch: char) -> bool {
    let lo = (lo - 1) as usize;
    let hi = (hi - 1) as usize;

    let pos_a = pass.chars().nth(lo) == Some(ch);
    let pos_b = pass.chars().nth(hi) == Some(ch);
    pos_a ^ pos_b
}
