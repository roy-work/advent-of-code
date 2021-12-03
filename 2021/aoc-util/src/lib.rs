use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};

use structopt::StructOpt;

pub mod map;
pub mod to_tuple;

pub mod prelude {
    use std::collections;
    use std::fs;
    use std::io;
    use std::path;

    pub use collections::{HashMap, HashSet};
    pub use fs::File;
    pub use io::{BufRead, BufReader};
    pub use path::Path;

    pub use crate::to_tuple::{ToTuple, IterExtToTuple};
}

pub fn file_o_numbers<P: AsRef<Path>>(path: P) -> io::Result<Vec<i64>> {
    let file = BufReader::new(File::open(path)?);

    let mut numbers = Vec::new();
    for line in file.lines() {
        let line = line?;
        numbers.push(line.parse().expect("could not parse line as number"));
    }
    Ok(numbers)
}

/*
pub fn one_line_of_ints(path: &Path) -> anyhow::Result<Vec<i64>> {
    let file = BufReader::new(File::open(path)?);
    Ok(file
        .lines()
        .next()
        .unwrap()?
        .split(',')
        .map(|p| p.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?)
}

pub fn one_line(path: &Path) -> anyhow::Result<String> {
    let file = BufReader::new(File::open(path)?);
    Ok(file
        .lines()
        .next()
        .unwrap()?
    )
}
*/

pub fn file_item_per_line<P: AsRef<Path>, F: FnMut(&str) -> Result<T, E>, T, E: Display>(
    path: P,
    mut parser: F,
) -> io::Result<Vec<T>> {
    let file = BufReader::new(File::open(path)?);

    let mut data = Vec::<T>::new();
    for (idx, line) in file.lines().enumerate() {
        let line = line?;
        data.push(
            parser(&line).unwrap_or_else(|err| panic!("failed to parse line {}: {}", idx + 1, err)),
        );
    }
    Ok(data)
}

#[derive(StructOpt)]
pub struct Args {
    #[structopt(long)]
    test_input_a: Option<PathBuf>,
    #[structopt(long)]
    test_input_b: Option<PathBuf>,
    #[structopt(long)]
    input: Option<PathBuf>,
}

pub fn main_stub<I, A, B, IT, IE, AT, BT>(
    input_parser: I,
    part_a: A,
    part_b: B,
    test_vec_a: Option<AT>,
    test_vec_b: Option<BT>,
) where
    I: Fn(&Path) -> Result<IT, IE>,
    A: for<'r> Fn(&'r IT) -> AT,
    B: for<'r> Fn(&'r IT) -> BT,
    IE: Debug,
    AT: Display + Eq,
    BT: Display + Eq,
{
    let args = Args::from_args();

    let test_input_a = args.test_input_a.as_deref().unwrap_or(Path::new("test-a"));
    if test_input_a.exists() {
        println!("Test input A exists. Testing…");
        let input = input_parser(test_input_a).expect("failed to parse test input");
        println!("(running part a on test input)");
        let part_a_answer = part_a(&input);
        if let Some(known_answer) = test_vec_a {
            if part_a_answer == known_answer {
                println!("\x1b[92mA TEST VEC PASSED!\x1b[0m");
            } else {
                println!(
                    "\x1b[1;91mA TEST VEC FAILED! ({} (actual) != {} (expected))\x1b[0m",
                    part_a_answer, known_answer
                );
            }
        }
    }
    let test_input_b = args.test_input_b.as_deref().unwrap_or(Path::new("test-b"));
    if test_input_b.exists() {
        println!("Test input b exists. Testing…");
        let input = input_parser(test_input_b).expect("failed to parse test input");
        println!("(running part b on test input)");
        let part_b_answer = part_b(&input);
        println!("Test vec B: {}", part_b_answer);
        if let Some(known_answer) = test_vec_b {
            if part_b_answer == known_answer {
                println!("\x1b[92mB TEST VEC PASSED!\x1b[0m");
            } else {
                println!(
                    "\x1b[1;91mB TEST VEC FAILED! ({} (actual) != {} (expected))\x1b[0m",
                    part_b_answer, known_answer
                );
            }
        }
    }

    println!("\x1b[1mRunning on the real input…\x1b[0m");
    let input_path = args.input.as_deref().unwrap_or(Path::new("input"));
    println!("(parsing the input file)");
    let input = input_parser(input_path).expect("failed to parse input");
    println!("(running part a)");
    let part_a_answer = part_a(&input);
    println!("Part A: {}", part_a_answer);
    println!("(running part b)");
    let part_b_answer = part_b(&input);
    println!("Part B: {}", part_b_answer);
}

#[macro_export]
macro_rules! aoc {
    ($parser:ident, $part_a:ident, $part_b:ident, $test_vec_a:expr, $test_vec_b:expr) => {
        fn main() {
            $crate::main_stub($parser, $part_a, $part_b, $test_vec_a, $test_vec_b);
        }
    };
}

#[macro_export]
macro_rules! lazy_regex {
    ($name:ident, $re:expr) => {
        static $name: Lazy<Regex> = Lazy::new(|| Regex::new($re).expect("regex failed to compile"));
    };
    ($re:expr) => {
        Lazy::new(|| Regex::new($re).expect("regex failed to compile"))
    };
}

#[macro_export]
macro_rules! hot_parse {
    ($fname:ident, $regex:expr, { $($group:expr => $ty:ty , )* }, $post:expr) => {
        fn $fname(input: &str) -> Result<($($ty,)*), $crate::HotParseError> {
            use once_cell::sync::Lazy;
            use regex::Regex;
            static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new($regex).expect("regex failed to compile"));
            let capture = REGEX.captures(input).ok_or($crate::HotParseError::RegexMatchFailed)?;
            let t = ($(
                capture
                    .get($group)
                    .unwrap_or_else(|| panic!("failed to unwrap capture group {}", $group))
                    .as_str()
                    .parse::<$ty>()
                    .map_err(|err| $crate::HotParseError::ParseError(Box::new(err)))?,
            )*);
            Ok(($post)(t))
        }
    }
}

#[derive(Debug)]
pub enum HotParseError {
    RegexMatchFailed,
    ParseError(Box<dyn std::fmt::Debug>),
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hot_parse() {
        hot_parse!(parser, "([0-9]+) (foo|bar)", { 1 => u8, 2 => String, }, |t| t);
        let result = parser("123 foo").unwrap();
        assert!(result == (123, "foo".to_owned()));
    }
}
