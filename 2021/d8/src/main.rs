use aoc::prelude::*;

struct InputLine {
    unique_sigs: Vec<String>,
    output_val: Vec<String>,
}
type Input = Vec<InputLine>;

fn parser(path: &Path) -> anyhow::Result<Input> {
    let mut reader = BufReader::new(File::open(path)?);

    let mut input = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let line = line.trim_end();
        let (unique_sigs, output_val) = line.split_once('|').unwrap();
        let unique_sigs = unique_sigs
            .trim()
            .split(' ')
            .map(|v| v.to_owned())
            .collect::<Vec<_>>();
        assert!(unique_sigs.len() == 10);
        let output_val = output_val
            .trim()
            .split(' ')
            .map(|v| v.to_owned())
            .collect::<Vec<_>>();
        assert!(output_val.len() == 4);
        input.push(InputLine {
            unique_sigs,
            output_val,
        });
    }
    Ok(input)
}

fn part_a(input: &Input) -> i64 {
    let mut count = 0;
    for input_line in input {
        for digit in input_line.output_val.iter() {
            match digit.len() {
                2 | 4 | 3 | 7 => count += 1,
                _ => (),
            }
        }
    }
    count
}

const ALL_SEGS: &[char] = &['a', 'b', 'c', 'd', 'e', 'f', 'g'];

/*
fn decode(input: &Vec<String>) -> HashMap<String, i64> {
    let mut decoded = HashMap::new();
    let input = input.iter().cloned().collect::<HashSet<String>>();
    let mut char_map = HashMap::<char, HashSet<char>>::new();
    let base_set: HashSet<char> = ALL_SEGS.iter().copied().collect();
    for c in ALL_SEGS {
        char_map.insert(*c, base_set.clone());
    }
    fn must_be_one_of(
        char_map: &mut HashMap<char, HashSet<char>>,
        unknown_chs: &str,
        known_chs: &str,
    ) {
        let known_chs = known_chs.chars().collect::<HashSet<char>>();
        let unknown_chs = unknown_chs.chars().collect::<HashSet<char>>();
        let other_chs = {
            let s = ALL_SEGS.iter().copied().collect::<HashSet<char>>();
            s.difference(&unknown_chs).copied().collect::<HashSet<char>>()
        };
        for c in unknown_chs {
            char_map.get_mut(&c).unwrap().retain(|c| known_chs.contains(c))
        }
        for c in other_chs {
            let map = char_map.get_mut(&c).unwrap();
            for c2 in &known_chs {
                map.remove(c2);
            }
        }
    }

    for val in input {
        match val.len() {
            // digit is 1
            2 => must_be_one_of(&mut char_map, &val, "cf"),
            // digit is 4
            4 => must_be_one_of(&mut char_map, &val, "bcdf"),
            // digit is 7
            3 => must_be_one_of(&mut char_map, &val, "acf"),
            // digit is 8
            7 => must_be_one_of(&mut char_map, &val, "abcdefg"),
            _ => (),
        }
    }

    unimplemented!();

    assert!(decoded.len() == 10);
    decoded
}
*/
fn decode(input: &Vec<String>) -> HashMap<String, i64> {
    let input = input.iter().cloned().collect::<HashSet<String>>();
    let mut decoded = HashMap::new();

    let mut five_segs = HashSet::new();
    let mut six_segs = HashSet::new();

    let mut the_one = None;
    let mut the_four = None;

    for val in input {
        match val.len() {
            // digit is 1
            2 => {
                decoded.insert(val.clone(), 1);
                assert!(the_one.replace(val.clone()).is_none());
            }
            // digit is 4
            4 => {
                decoded.insert(val.clone(), 4);
                assert!(the_four.replace(val.clone()).is_none());
            }
            // digit is 7
            3 => { decoded.insert(val.clone(), 7); }
            // digit is 8
            7 => { decoded.insert(val.clone(), 8); }
            5 => {
                five_segs.insert(val.clone());
            },
            6 => {
                six_segs.insert(val.clone());
            },
            _ => panic!()
        }
    }
    let the_one = the_one.unwrap();
    let the_four = the_four.unwrap();

    let mut the_six = None;
    for unk_six in &six_segs {
        if !the_one.chars().all(|c| unk_six.contains(c)) {
            assert!(the_six.replace(unk_six.clone()).is_none());
        }
    }
    let the_six = the_six.unwrap();
    six_segs.remove(&the_six);
    decoded.insert(the_six.clone(), 6);

    let mut the_nine = None;
    for unk_six in &six_segs {
        if the_four.chars().all(|c| unk_six.contains(c)) {
            assert!(the_nine.replace(unk_six.clone()).is_none());
        }
    }
    let the_nine = the_nine.unwrap();
    six_segs.remove(&the_nine);
    decoded.insert(the_nine, 9);

    // What's left is the 0
    assert!(six_segs.len() == 1);
    decoded.insert(six_segs.iter().next().unwrap().clone(), 0);

    // Solve for 3
    let mut the_three = None;
    for unk_five in &five_segs {
        if the_one.chars().all(|c| unk_five.contains(c)) {
            assert!(the_three.replace(unk_five.clone()).is_none());
        }
    }
    let the_three = the_three.unwrap();
    five_segs.remove(&the_three);
    decoded.insert(the_three, 3);

    // Solve for 5
    let mut the_five = None;
    for unk_five in &five_segs {
        let mut count = 0;
        for c in the_six.chars() {
            if unk_five.contains(c) {
                count += 1;
            }
        }
        if count == 5 {
            assert!(the_five.replace(unk_five.clone()).is_none());
        }
    }
    let the_five = the_five.unwrap();
    five_segs.remove(&the_five);
    decoded.insert(the_five, 5);

    // What's left is the 2
    assert!(five_segs.len() == 1);
    decoded.insert(five_segs.iter().next().unwrap().clone(), 2);

    assert!(decoded.len() == 10);
    decoded
}

fn norm_str(s: &str) -> String {
    let mut chars = s.chars().collect::<Vec<_>>();
    chars.sort();
    let mut s = String::new();
    for c in chars {
        s.push(c);
    }
    s
}

fn decode_and_reveal(input: &InputLine) -> i64 {
    let decoded = decode(&input.unique_sigs);
    let decoded = decoded.iter().map(|(k, v)| (norm_str(k), *v)).collect::<HashMap<String, i64>>();
    let mut value = 0;
    for digit in &input.output_val {
        value = value * 10;
        let digit = norm_str(digit);
        let digit = decoded.get(&digit).unwrap();
        value += digit;
    }
    value
}

fn part_b(input: &Input) -> i64 {
    input.iter().map(|i| decode_and_reveal(i)).sum()
}

aoc::aoc!(parser, part_a, part_b, Some(26), Some(61229));
