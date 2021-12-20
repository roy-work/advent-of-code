use aoc::map::{BoundCoord, FreeCoord, Map};
use aoc::prelude::*;

type Input = Vec<String>;

fn parser(path: &Path) -> anyhow::Result<Input> {
    let mut reader = BufReader::new(File::open(path)?);
    let mut input = Vec::new();
    for line in reader.lines() {
        let line = line?;
        input.push(line.trim_end().to_owned());
    }
    Ok(input)
}

#[derive(Debug)]
enum Error {
    WrongMatch(char),
    Incomplete(Vec<char>),
}

fn flip(c: char) -> char {
    match c {
        '[' => ']',
        '(' => ')',
        '{' => '}',
        '<' => '>',
        _ => panic!(),
    }
}

fn parse_line(line: &str) -> Result<(), Error> {
    let mut stack: Vec<char> = Vec::new();

    for c in line.chars() {
        match c {
            '[' | '(' | '{' | '<' => stack.push(c),
            ']' | ')' | '}' | '>' => {
                let matcher = flip(stack.pop().unwrap());
                if matcher != c {
                    //println!("stack: {:?} {:?} {:?}", stack, matcher, c);
                    return Err(Error::WrongMatch(c));
                }
            }
            _ => panic!(),
        }
    }

    if !stack.is_empty() {
        Err(Error::Incomplete(stack))
    } else {
        Ok(())
    }
}

fn score(c: char) -> i64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!(),
    }
}

fn part_a(input: &Input) -> i64 {
    input
        .iter()
        .map(|i| (i, parse_line(i)))
        .filter_map(|(i, r)| {
            //println!("{}, {:?}", i, r);
            match r {
                Err(Error::WrongMatch(c)) => Some(c),
                _ => {
                    //println!("Discard: {}", i);
                    None
                },
            }
        })
        .map(score)
        .sum()
}

fn score_2(stack: &[char]) -> i64 {
    let mut score = 0;
    //println!("stack: {:?}", stack);
    for c in stack.iter().copied() {
        let c_score = match flip(c) {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!(),
        };
        score = score * 5 + c_score;
        //println!("score: {}", score);
    }

    score
}

fn part_b(input: &Input) -> i64 {
    assert!(score_2(&['[','(','{','<']) == 294); 
    assert!(score_2(&['{','{','[','[','(','{','(', '[']) == 288957);
    let mut scores = input
        .iter()
        .map(|i| (i, parse_line(i)))
        .filter_map(|(i, r)| {
            match r {
                Err(Error::Incomplete(s)) => {
                    println!("Keep: {:?}", s);
                    Some(s.iter().copied().rev().collect::<Vec<_>>())
                }
                _ => {
                    //println!("Discard: {}", i);
                    None
                },
            }
        })
        .map(|s| score_2(&s))
        .collect::<Vec<_>>();
    scores.sort();
    println!("{:#?}", scores);
    scores[scores.len() / 2]
}

aoc::aoc!(parser, part_a, part_b, Some(26397), Some(288957));
