use aoc::prelude::*;

struct Input {
    target_x: (i64, i64),
    target_y: (i64, i64),
}

impl Input {
    fn is_hit(&self, probe: &Pos) -> bool {
        let x_hit = self.target_x.0 <= probe.x && probe.x <= self.target_x.1;
        let y_hit = self.target_y.0 <= probe.y && probe.y <= self.target_y.1;
        x_hit && y_hit
    }
}

aoc::hot_parse!(pline, "^target area: x=([0-9]+)..([0-9]+), y=(-[0-9]+)..(-[0-9]+)$", { 1 => i64, 2 => i64, 3 => i64, 4 => i64, }, |t| t);
fn parser(path: &Path) -> anyhow::Result<Input> {
    let reader = BufReader::new(File::open(path)?);
    let mut lines = reader.lines();

    let line = lines.next().unwrap()?;
    assert!(lines.next().is_none());
    let (target_x, target_y) = {
        let (a, b, c, d) = pline(&line).unwrap();
        ((a, b), (c, d))
    };
    Ok(Input { target_x, target_y })
}

struct Pos {
    x: i64,
    y: i64,
}

fn simulate(input: &Input, mut x_velocity: i64, mut y_velocity: i64) -> (bool, i64) {
    let mut probe_pos = Pos { x: 0, y: 0 };
    let mut max_y = 0;
    loop {
        probe_pos.x += x_velocity;
        probe_pos.y += y_velocity;
        max_y = std::cmp::max(max_y, probe_pos.y);
        x_velocity = std::cmp::max(x_velocity - 1, 0);
        y_velocity -= 1;

        if input.is_hit(&probe_pos) {
            return (true, max_y);
        }
        if probe_pos.y < input.target_y.0 {
            break;
        }
    }
    (false, max_y)
}

fn part_a(input: &Input) -> i64 {
    println!("Simulate: xv = {}, yv = {}", 9, 0);
    let (hit, max_y) = simulate(input, 9, 0);
    println!("  hit: {}, max_y = {}", hit, max_y);
    //assert!(false);
    let mut best_max_y = 0;
    for y_vol in 0..=200 {
        for x_vol in 0..=50 {
            let (hit, max_y) = simulate(input, x_vol, y_vol);
            if hit && best_max_y < max_y {
                //println!("Simulate: xv = {}, yv = {}", x_vol, y_vol);
                //println!("  hit: {}, max_y = {}", hit, max_y);
                best_max_y = std::cmp::max(best_max_y, max_y);
            }
        }
    }
    best_max_y
}

fn part_b(input: &Input) -> i64 {
    let mut initial_vels = HashSet::<(i64, i64)>::new();

    for y_vol in -400..=400 {
        for x_vol in -400..=400 {
            let (hit, _) = simulate(input, x_vol, y_vol);
            if hit {
                println!("Hit: {} {}", x_vol, y_vol);
                initial_vels.insert((x_vol, y_vol));
            }
        }
    }

    i64::try_from(initial_vels.len()).unwrap()
}

aoc::aoc!(parser, part_a, part_b, Some(45), Some(112));
