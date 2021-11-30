use std::collections::{hash_map, VecDeque};
use std::convert::TryFrom;
use std::iter::FromIterator;

use anyhow::Context;
use once_cell::sync::Lazy;
use regex::Regex;

use aoc::prelude::*;

struct Input {
    card_pubkey: i64,
    door_pubkey: i64,
}

fn parser(path: &Path) -> anyhow::Result<Input> {
    let file = BufReader::new(File::open(path)?);
    let mut lines = file.lines();

    let card_pubkey = lines.next().unwrap()?.parse::<i64>()?;
    let door_pubkey = lines.next().unwrap()?.parse::<i64>()?;
    Ok(Input {
        card_pubkey,
        door_pubkey
    })
}

fn transform(subject_number: i64, mut loop_size: usize) -> i64 {
    let mut v: i64 = 1;
    let mut need_mod = 0;

    let e = 2048;
    let subj_e = {
        let mut v: i64 = 1;
        for _ in 0..e {
            v = v.checked_mul(subject_number).unwrap();
            v = v % 20201227;
        }
        v
    };
    while loop_size > e {
        v = v.checked_mul(subj_e).unwrap();
        v = v % 20201227;  // ~20m
        loop_size -= e;
    }

    for n in 0..loop_size {
        v = v.checked_mul(subject_number).unwrap();
        v = v % 20201227;  // ~20m
        /*
        if need_mod == 10 {
            v = v % 20201227;  // ~20m
            need_mod = 0;
        } else {
            need_mod += 1;
        }
        */
    }
    v
    //v % 20201227
}

fn simple_transform(subject_number: i64, loop_size: usize) -> i64 {
    let mut v = 1;
    for n in 0..loop_size {
        v *= subject_number;
        //if n & 0x03 == 0 {
            v = v % 20201227;  // ~20m
        //}
    }
    v % 20201227
}

/*
fn find_privkey(pubkey: i64) -> usize {
    let mut solved_privkey = None;

    for privkey in 0..200000 {
        let maybe_pubkey = transform(7, privkey);
        if maybe_pubkey == pubkey {
            solved_privkey = Some(privkey);
            break;
        }
        if privkey & 0xff == 0 {
            print!("\r\x1b[Lfind: {}", privkey);
            use std::io::Write;
            std::io::stdout().flush().unwrap();
        }
    }
    println!();

    solved_privkey.unwrap()
}
*/

fn find_privkey(pubkey: i64) -> usize {
    use rayon::prelude::*;
    let found = (0..100_000_000usize).into_par_iter().find_any(|pk| {
        let privkey = *pk;
        let maybe_pubkey = transform(7, privkey);
        if maybe_pubkey == pubkey {
            return true;
        }
        if privkey & 0xfff == 0 {
            print!("\r\x1b[Lfind: {}", privkey);
            use std::io::Write;
            std::io::stdout().flush().unwrap();
        }
        false
    });
    println!();

    found.unwrap()
}

fn part_a(input: &Input) -> i64 {
    /*
    let door_privkey = find_privkey(input.door_pubkey);
    println!("Door private key: {}", door_privkey);
    simple_transform(input.card_pubkey, door_privkey)
    */
    let card_privkey = find_privkey(input.card_pubkey);
    println!("Card private key: {}", card_privkey);
    simple_transform(input.door_pubkey, card_privkey)

}

fn part_b(input: &Input) -> usize {
    2
}

aoc::aoc!(parser, part_a, part_b, Some(14897079), None);
