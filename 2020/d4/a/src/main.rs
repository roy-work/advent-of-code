#![feature(str_split_once)]

use std::collections::HashSet;

use once_cell::sync::Lazy;
use regex::Regex;

use aoc::prelude::*;

struct Passport(HashMap<String, String>);

fn parse_input() -> anyhow::Result<Vec<Passport>> {
    let input = BufReader::new(File::open("input")?);

    let mut passports = Vec::new();

    let mut lines = input.lines();

    'outer: loop {
        let mut this_passport = HashMap::new();
        let mut have_lines = false;
        loop {
            let line = match lines.next() {
                Some(Ok(line)) => line,
                Some(Err(err)) => return Err(err.into()),
                None => {
                    if have_lines {
                        passports.push(Passport(this_passport));
                    }
                    break 'outer;
                }
            };

            if line.is_empty() {
                passports.push(Passport(this_passport));
                break;
            }

            have_lines = true;
            for piece in line.split(' ') {
                let (k, v) = piece.split_once(':').unwrap();
                this_passport.insert(k.to_owned(), v.to_owned());
            }
        }
    }
    Ok(passports)
}

fn main() {
    let passports = parse_input().unwrap();

    let count = passports.iter().filter(|p| is_valid(p)).count();
    println!("{}", count);
}

/*
hgt (Height) - a number followed by either cm or in:

    If cm, the number must be at least 150 and at most 193.
    If in, the number must be at least 59 and at most 76.
*/
static HGT_RE: Lazy<Regex> = Lazy::new(|| Regex::new("^(\\d+)(cm|in)$").unwrap());

// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
static HCL_RE: Lazy<Regex> = Lazy::new(|| Regex::new("^#[0-9a-f]{6}$").unwrap());

// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
static ECL_RE: Lazy<Regex> = Lazy::new(|| Regex::new("^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap());

// pid (Passport ID) - a nine-digit number, including leading zeroes.
static PID_RE: Lazy<Regex> = Lazy::new(|| Regex::new("^\\d{9}$").unwrap());


fn is_valid(passport: &Passport) -> bool {
    let all_attrs = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    if !all_attrs.iter().all(|k| passport.0.contains_key(*k)) {
        return false;
    }

    let byr = is_ranged(passport, "byr", 1920, 2002);
    let iyr = is_ranged(passport, "iyr", 2010, 2020);
    let eyr = is_ranged(passport, "eyr", 2020, 2030);
    let hgt = is_hgt(&passport.0["hgt"]);
    let hcl = is_regex(&passport.0["hcl"], &HCL_RE);
    let ecl = is_regex(&passport.0["ecl"], &ECL_RE);
    let pid = is_regex(&passport.0["pid"], &PID_RE);
    let valid = byr && iyr && eyr && hgt && hcl && ecl && pid;
    print_pp(passport);
    println!("--> VALID: {}", valid);
    println!();
    valid
}

fn is_ranged(p: &Passport, k: &str, lo: u32, hi: u32) -> bool {
    p.0[k].parse::<u32>().map(|v| lo <= v && v <= hi).unwrap_or(false)
}

fn is_regex(v: &str, regex: &Regex) -> bool {
    regex.is_match(v)
}

fn is_hgt(v: &str) -> bool {
    let captures = match HGT_RE.captures(v) {
        Some(c) => c,
        None => return false,
    };

    let height = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let unit = captures.get(2).unwrap().as_str();
    if unit == "in" {
        59 <= height && height <= 76
    } else {
        150 <= height && height <= 193
    }
}

fn print_pp(passport: &Passport) {
    let mut kvs = passport.0.iter().collect::<Vec<_>>();
    kvs.sort();

    for (k, v) in kvs {
        println!("{}: {}", k, v);
    }
}

fn tf(b: bool) -> char {
    match b {
        true => 't',
        false => 'f',
    }
}

#[cfg(test)]
mod tests {
    use super::HashMap;

    #[test]
    fn test_stuff() {
        let mut passport = HashMap::<String, String>::new();
        passport.insert("byr".to_owned(), "2000".to_owned());

        let passport = super::Passport(passport);

        assert!(super::is_ranged(&passport, "byr", 1920, 2002));
    }
}
