use aoc::prelude::*;
use aoc::hot_parse;

struct Input {
    template: String,
    pair_ins_rules: HashMap<(char, char), char>,
}

fn parser(path: &Path) -> anyhow::Result<Input> {
    let reader = BufReader::new(File::open(path)?);
    let mut lines = reader.lines();

    let template = lines.next().unwrap()?.trim_end().to_owned();
    let blank = lines.next().unwrap()?;
    assert!(blank.trim_end() == "");

    let mut pair_ins_rules = HashMap::new();
    hot_parse!(pparse, "^([A-Z]{2}) -> ([A-Z])$", { 1 => String, 2 => String, }, |t| t);
    for line in lines {
        let line = line?;
        let (pair, ins) = pparse(line.trim_end()).unwrap();
        let pair = pair.chars().collect::<Vec<_>>();
        assert!(pair.len() == 2);
        let pair = (pair[0], pair[1]);
        let ins = ins.chars().collect::<Vec<_>>();
        assert!(ins.len() == 1);
        let ins = ins[0];
        pair_ins_rules.insert(pair, ins);
    }

    Ok(Input { template, pair_ins_rules })
}

fn do_pairs(s: &str, pair_ins_rules: &HashMap<(char, char), char>) -> String {
    let mut new_s = String::new();
    for window in s.as_bytes().windows(2) {
        let pair = (window[0] as char, window[1] as char);
        let insert = *pair_ins_rules.get(&pair).unwrap();
        new_s.push(window[0] as char);
        new_s.push(insert);
    }
    new_s.push(*s.as_bytes().last().unwrap() as char);
    new_s
}

fn char_counts(s: &str) -> HashMap<char, i64> {
    let mut result = HashMap::new();
    for c in s.chars() {
        *result.entry(c).or_insert(0) += 1;
    }
    result
}

fn part_a(input: &Input) -> i64 {
    let mut current_poly = input.template.clone();
    for _ in 0..10 {
        current_poly = do_pairs(&current_poly, &input.pair_ins_rules);
    }
    let counts = char_counts(&current_poly);
    let max_char = counts.iter().max_by_key(|(_, v)| *v).unwrap().1;
    let min_char = counts.iter().min_by_key(|(_, v)| *v).unwrap().1;
    max_char - min_char
}

fn pop_initial_counts(input: &Input) -> HashMap<(i64, char, char), HashMap<char, i64>> {
    let mut memo = HashMap::new();
    for ((a, b), ins) in input.pair_ins_rules.iter() {
        let s = {
            let mut s = String::new();
            s.push(*a);
            s.push(*ins);
            s.push(*b);
            s
        };
        let counts = char_counts(&s);
        memo.insert((1, *a, *b), counts);
    }
    memo
}

fn merge_counts(count_a: &HashMap<char, i64>, count_b: &HashMap<char, i64>) -> HashMap<char, i64> {
    let mut result = HashMap::new();
    for (k, v) in count_a.iter() {
        *result.entry(*k).or_insert(0) += v;
    }
    for (k, v) in count_b.iter() {
        *result.entry(*k).or_insert(0) += v;
    }
    result
}

/*
fn add_step(step: i64, input: &Input, memo: &mut HashMap<(i64, char, char), HashMap<char, i64>>) {
    let one_step_less = step - 1;
    for window in input.template.as_bytes().windows(2) {
        let (a, b) = (window[0] as char, window[1] as char);
        let insert = *input.pair_ins_rules.get(&(a, b)).unwrap();
        println!("Looking up ({}, {}) at depth {}", a, insert, one_step_less);
        let count_a = memo.get(&(one_step_less, a, insert)).unwrap();
        println!("Looking up ({}, {}) at depth {}", insert, b, one_step_less);
        let count_b = memo.get(&(one_step_less, insert, b)).unwrap();
        let mut counts = merge_counts(count_a, count_b);
        *counts.get_mut(&insert).unwrap() -= 1;
        memo.insert((step, a, b), counts);
    }
}
*/

fn add_memo(step: i64, input: &Input, ab: (char, char), memo: &mut HashMap<(i64, char, char), HashMap<char, i64>>) {
    let (a, b) = ab;
    let insert = *input.pair_ins_rules.get(&(a, b)).unwrap();
    let one_step_less = step - 1;

    if !memo.contains_key(&(one_step_less, a, insert)) {
        if step == 1 {
            panic!();
        }
        add_memo(one_step_less, input, (a, insert), memo);
    }
    if !memo.contains_key(&(one_step_less, insert, b)) {
        if step == 1 {
            panic!();
        }
        add_memo(one_step_less, input, (insert, b), memo);
    }

    //println!("Looking up ({}, {}) at depth {}", a, insert, one_step_less);
    let count_a = memo.get(&(one_step_less, a, insert)).unwrap();
    //println!("Looking up ({}, {}) at depth {}", insert, b, one_step_less);
    let count_b = memo.get(&(one_step_less, insert, b)).unwrap();
    let mut counts = merge_counts(count_a, count_b);
    *counts.get_mut(&insert).unwrap() -= 1;
    memo.insert((step, a, b), counts);
}

fn part_b(input: &Input) -> i64 {
    let mut memo = pop_initial_counts(input);
    let mut all_counts = HashMap::new();
    const STEP: i64 = 40;
    for window in input.template.as_bytes().windows(2) {
        let (a, b) = (window[0] as char, window[1] as char);
        add_memo(STEP, input, (a, b), &mut memo);
        let counts = memo.get(&(STEP, a, b)).unwrap();
        all_counts = merge_counts(&all_counts, &counts);
        *all_counts.get_mut(&b).unwrap() -= 1;
    }
    let last_char = *input.template.as_bytes().last().unwrap() as char;
    *all_counts.get_mut(&last_char).unwrap() += 1;
    println!("{:#?}", all_counts);

    let max_char = all_counts.iter().max_by_key(|(_, v)| *v).unwrap().1;
    let min_char = all_counts.iter().min_by_key(|(_, v)| *v).unwrap().1;
    max_char - min_char
    /*
    for sn in 2..=10 {
        println!("Adding step {}", sn);
        add_step(sn, input, &mut memo);
        println!("Step additions:");
        for ((_, a, b), counts) in memo.iter().filter(|((tsn, _, _), _)| *tsn == sn) {
            println!("counts for ({}, {}): {:?}", a, b, counts);
        }
    }

    let mut all_counts = HashMap::new();
    let one_step_less = 9;
    for window in input.template.as_bytes().windows(2) {
        let (a, b) = (window[0] as char, window[1] as char);
        let insert = *input.pair_ins_rules.get(&(a, b)).unwrap();
        let count_a = memo.get(&(one_step_less, a, insert)).unwrap();
        let count_b = memo.get(&(one_step_less, insert, b)).unwrap();
        let mut counts = merge_counts(count_a, count_b);
        *counts.get_mut(&insert).unwrap() -= 1;
        *counts.get_mut(&b).unwrap() -= 1;
        all_counts = merge_counts(&all_counts, &counts);
    }
    let last_char = *input.template.as_bytes().last().unwrap() as char;
    *all_counts.get_mut(&last_char).unwrap() += 1;
    println!("{:#?}", all_counts);
    2
    */
    /*
    let mut current_poly = input.template.clone();
    for s in 0..10 {
        println!("Step {}", s);
        current_poly = do_pairs(&current_poly, &input.pair_ins_rules);
        if current_poly.len() > 1000_000_000 {
            assert!(false);
        }
        let counts = char_counts(&current_poly);
        let max_char = counts.iter().max_by_key(|(_, v)| *v).unwrap().1;
        let min_char = counts.iter().min_by_key(|(_, v)| *v).unwrap().1;
        let diff = max_char - min_char;
        println!("Diff at step {} is {}", s, diff);
    }
    let counts = char_counts(&current_poly);
    let max_char = counts.iter().max_by_key(|(_, v)| *v).unwrap().1;
    let min_char = counts.iter().min_by_key(|(_, v)| *v).unwrap().1;
    max_char - min_char
    */
}

aoc::aoc!(parser, part_a, part_b, Some(1588), Some(2188189693529));
