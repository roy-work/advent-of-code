use aoc::prelude::*;

type Input = Vec<i64>;

fn parser(path: &Path) -> anyhow::Result<Input> {
    let file = BufReader::new(File::open(path)?);
    let line = file.lines().next().unwrap()?;
    Ok(aoc::comma_split(line.trim_end(), |p| p.parse().unwrap()))
}

fn into_buckets(input: &Input) -> VecDeque<i64> {
    let mut buckets = VecDeque::new();
    for _ in 0 ..= 8 {
        buckets.push_back(0);
    }

    for fish in input {
        assert!(0 < *fish);
        buckets[usize::try_from(*fish).unwrap()] += 1;
    }

    buckets
}

fn simulate_day(buckets: &mut VecDeque<i64>) {
    let spawning_fish = buckets.pop_front().unwrap();
    buckets.push_back(0);
    buckets[6] += spawning_fish;
    buckets[8] += spawning_fish;
}

fn part_a(input: &Input) -> i64 {
    let mut buckets = into_buckets(input);
    for _ in 0..80 {
        simulate_day(&mut buckets);
    }
    buckets.iter().sum()
}

fn part_b(input: &Input) -> i64 {
    let mut buckets = into_buckets(input);
    for _ in 0..256 {
        simulate_day(&mut buckets);
    }
    buckets.iter().sum()
}

aoc::aoc!(parser, part_a, part_b, Some(5934), Some(26984457539));
