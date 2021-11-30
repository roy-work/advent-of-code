use std::collections::HashSet;

fn main() {
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
}
