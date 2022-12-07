use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};

use anyhow::Context;
use structopt::StructOpt;

mod autofetch;
pub mod map;
pub mod term;
mod text;
pub mod to_tuple;

pub mod prelude {
    use std::collections;
    use std::fs;
    use std::io;
    use std::path;

    pub use collections::{HashMap, HashSet, VecDeque};
    pub use fs::File;
    pub use io::{BufRead, BufReader};
    pub use path::Path;
    pub use std::convert::{TryFrom, TryInto};

    pub use crate::to_tuple::{IterExtToTuple, ToTuple};
}

pub use text::{char_to_relative_ord, ord};

/// The input for an AoC problem.
pub struct InputFile<'a> {
    path: &'a Path,
}

impl InputFile<'_> {
    /// The path to the input file.
    pub fn path(&self) -> &Path {
        self.path
    }

    /// Open the input file and enumerate the lines in it.
    pub fn lines(&self) -> io::Result<std::io::Lines<BufReader<File>>> {
        Ok(BufReader::new(File::open(self.path)?).lines())
    }

    /// The input file has one item of some type per line.
    ///
    /// `parse` is a function to parse each line.
    pub fn one_item_per_line<F, T>(&self, parse: F) -> anyhow::Result<Vec<T>>
    where
        F: for<'a> Fn(&'a str) -> anyhow::Result<T>,
    {
        let rdr = BufReader::new(
            File::open(self.path)
                .with_context(|| format!("failed to open {}", self.path.display()))?,
        );
        let mut items = Vec::new();
        for (idx, line) in rdr.lines().enumerate() {
            let line = line.context("failed to read line from file")?;
            let item = parse(&line).with_context(|| format!("failed to parse line {}", idx + 1))?;
            items.push(item);
        }
        Ok(items)
    }

    /// The input file has one integer, per line.
    pub fn one_number_per_line(&self) -> anyhow::Result<Vec<i64>> {
        let rdr = BufReader::new(
            File::open(self.path)
                .with_context(|| format!("failed to open {}", self.path.display()))?,
        );
        let mut items = Vec::new();
        for (idx, line) in rdr.lines().enumerate() {
            let line = line.context("failed to read line from file")?;
            let n = line.parse().with_context(|| format!("failed to parse line {}", idx + 1))?;
            items.push(n);
        }
        Ok(items)
    }

    /// The input file is a single line, the input.
    pub fn single_line_input(&self) -> io::Result<String> {
        let reader = BufReader::new(File::open(self.path)?);
        let mut lines = reader.lines();
        let line = lines
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "no lines in input file"))??;
        match lines.next() {
            Some(_) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "multiple lines in input file",
            )),
            None => Ok(line),
        }
    }
}

impl AsRef<Path> for InputFile<'_> {
    fn as_ref(&self) -> &Path {
        self.path
    }
}

impl<'a> From<&'a Path> for InputFile<'a> {
    fn from(path: &'a Path) -> InputFile<'a> {
        InputFile {
            path,
        }
    }
}

pub fn comma_split<F: Fn(&str) -> T, T>(s: &str, f: F) -> Vec<T> {
    s.split(',').map(f).collect()
}

pub fn comma_try_split<F: Fn(&str) -> Result<T, E>, T, E>(s: &str, f: F) -> Result<Vec<T>, E> {
    s.split(',').map(f).collect::<Result<Vec<T>, E>>()
}

pub fn histogram<T: Copy + Eq + Hash>(input: impl IntoIterator<Item = T>) -> HashMap<T, usize> {
    let mut counts = HashMap::new();
    for item in input.into_iter() {
        *counts.entry(item).or_insert(0) += 1;
    }
    counts
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
*/

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
    I: Fn(&InputFile<'_>) -> Result<IT, IE>,
    A: for<'r> Fn(&'r IT) -> AT,
    B: for<'r> Fn(&'r IT) -> BT,
    IE: Debug,
    AT: Display + Eq,
    BT: Display + Eq,
{
    let args = Args::from_args();

    match tokio::runtime::Runtime::new() {
        Ok(runtime) => {
            let result = runtime.block_on(autofetch::autofetch());
            match result {
                Ok(()) => (),
                Err(err) => {
                    eprintln!("\x1b[1;91mAuto-fetch failed:\x1b[0m {err}");
                }
            }
        }
        Err(err) => eprintln!("\x1b[1;91mAuto-fetch failed:\x1b[0m {err}"),
    }

    let test_input_a = args.test_input_a.as_deref().unwrap_or(Path::new("test-a"));
    if test_input_a.exists() {
        println!("\x1b[96m── Part A: Test ────\x1b[0m");
        println!("Test input A exists. Running test…");
        let input = input_parser(&test_input_a.into()).expect("failed to parse test input");
        println!("(input parsed; running part A test…)");
        let part_a_answer = part_a(&input);
        match test_vec_a {
            Some(known_answer) => {
                if part_a_answer == known_answer {
                    println!("\x1b[92mPART \x1b[4mA\x1b[24m TEST VEC PASSED!\x1b[0m");
                } else {
                    println!(
                        "\x1b[1;91mPART A TEST VEC FAILED! ({} (actual) != {} (expected))\x1b[0m",
                        part_a_answer, known_answer
                    );
                }
            }
            None => {
                println!("Test A output: {}", part_a_answer);
                println!("  (but there was no expect answer to check against)");
            }
        }
    }
    let test_input_b = args.test_input_b.as_deref().unwrap_or(Path::new("test-b"));
    if test_input_b.exists() {
        println!("\x1b[96m── Part B: Test ────\x1b[0m");
        println!("Test input B exists. Running test…");
        let input = input_parser(&test_input_b.into()).expect("failed to parse test input");
        println!("(input parsed; running part B test…)");
        let part_b_answer = part_b(&input);
        match test_vec_b {
            Some(known_answer) => {
                if part_b_answer == known_answer {
                    println!("\x1b[92mPART \x1b[4mB\x1b[24m TEST VEC PASSED!\x1b[0m");
                } else {
                    println!(
                        "\x1b[1;91mPART B TEST VEC FAILED! ({} (actual) != {} (expected))\x1b[0m",
                        part_b_answer, known_answer
                    );
                }
            }
            None => {
                println!("Test B output: {}", part_b_answer);
                println!("  (but there was no expect answer to check against)");
            }
        }
    }

    println!("\x1b[1m── Running on the real input ────\x1b[0m");
    let input_path = args.input.as_deref().unwrap_or(Path::new("input"));
    print!("(parsing the input file)");
    let input = match input_parser(&input_path.into()) {
        Ok(i) => {
            println!("\rInput parsed successfully.");
            i
        }
        Err(err) => {
            println!();
            panic!("failed to parse input: {:?}", err);
        }
    };
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
macro_rules! quick_regex_parser {
    ($fname:ident, $regex:expr, { $($group:expr => $ty:ty , )* }) => {
        fn $fname(input: &str) -> Result<($($ty,)*), $crate::HotParseError> {
            use once_cell::sync::Lazy;
            use regex::Regex;
            static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new($regex).expect("regex failed to compile"));
            let capture = REGEX.captures(input).ok_or_else(|| {
                eprintln!("Failed on: {:?}", input);
                $crate::HotParseError::RegexMatchFailed
            })?;
            let t = ($(
                capture
                    .get($group)
                    .unwrap_or_else(|| panic!("failed to unwrap capture group {}", $group))
                    .as_str()
                    .parse::<$ty>()
                    .map_err(|err| $crate::HotParseError::ParseError(Box::new(err)))?,
            )*);
            Ok(t)
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
    fn test_quick_regex_parser() {
        quick_regex_parser!(parser, "([0-9]+) (foo|bar)", { 1 => u8, 2 => String, });
        let result = parser("123 foo").unwrap();
        assert!(result == (123, "foo".to_owned()));
    }
}
