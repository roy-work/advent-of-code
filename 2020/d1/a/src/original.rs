use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = BufReader::new(File::open("input").unwrap());

    let numbers = {
        let mut data = Vec::new();
        for line in file.lines() {
            let line = line.unwrap();
            data.push(line.parse::<u32>().unwrap());
        }
        data
    };

    //let numbers =/I 
    let numbers = numbers.iter().map(|n| *n).collect::<HashSet<_>>();

    //let mut the_match = None;
    'outer: for a in numbers.iter() {
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
}
