use std::fmt::{self, Display};
use std::str::FromStr;

use aoc::prelude::*;

type Input = Vec<Instruction>;

#[derive(Clone, Copy, Debug)]
enum Var {
    W,
    X,
    Y,
    Z,
}

impl Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Var::*;
        let ch = match self {
            W => 'w',
            X => 'x',
            Y => 'y',
            Z => 'z',
        };
        write!(f, "{}", ch)
    }
}

impl FromStr for Var {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "w" => Var::W,
            "x" => Var::X,
            "y" => Var::Y,
            "z" => Var::Z,
            _ => return Err(()),
        })
    }
}

#[derive(Clone)]
enum Arg {
    Var(Var),
    Int(i64),
}

impl Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Arg::Var(v) => write!(f, "{}", v),
            Arg::Int(n) => write!(f, "{}", n),
        }
    }
}

impl FromStr for Arg {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(v) = s.parse::<Var>() {
            Ok(Arg::Var(v))
        } else {
            Ok(Arg::Int(s.parse()?))
        }
    }
}

#[derive(Clone)]
enum Instruction {
    Inp(Var),
    Add(Var, Arg),
    Mul(Var, Arg),
    Div(Var, Arg),
    Mod(Var, Arg),
    Eql(Var, Arg),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Instruction::*;

        match self {
            Inp(v) => write!(f, "inp {}", v),
            Add(v, a) => write!(f, "add {} {}", v, a),
            Mul(v, a) => write!(f, "mul {} {}", v, a),
            Div(v, a) => write!(f, "div {} {}", v, a),
            Mod(v, a) => write!(f, "mod {} {}", v, a),
            Eql(v, a) => write!(f, "eql {} {}", v, a),
        }
    }
}


fn parser(path: &Path) -> anyhow::Result<Input> {
    let reader = BufReader::new(File::open(path)?);
    let lines = reader.lines();

    let mut instructions = Vec::new();
    for line in lines {
        let line = line?;
        let (ins, rem) = line.trim_end().split_once(' ').unwrap();
        let instr = {
            if ins == "inp" {
                Instruction::Inp(rem.parse().unwrap())
            } else {
                let (a, b) = rem.split_once(' ').unwrap();
                let a = a.parse::<Var>().unwrap();
                let b = b.parse::<Arg>().unwrap();
                match ins {
                    "add" => Instruction::Add(a, b),
                    "mul" => Instruction::Mul(a, b),
                    "div" => Instruction::Div(a, b),
                    "mod" => Instruction::Mod(a, b),
                    "eql" => Instruction::Eql(a, b),
                    _ => panic!(),
                }
            }
        };
        instructions.push(instr);
    }

    Ok(instructions)
}

#[derive(Debug)]
struct Regs {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl Regs {
    fn new() -> Regs {
        Regs {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    fn read(&self, v: Var) -> i64 {
        match v {
            Var::W => self.w,
            Var::X => self.x,
            Var::Y => self.y,
            Var::Z => self.z,
        }
    }

    fn read_arg(&self, a: &Arg) -> i64 {
        match a {
            Arg::Var(v) => self.read(*v),
            Arg::Int(n) => *n,
        }
    }

    fn write(&mut self, v: Var, n: i64) -> (Var, i64) {
        match v {
            Var::W => self.w = n,
            Var::X => self.x = n,
            Var::Y => self.y = n,
            Var::Z => self.z = n,
        }
        (v, n)
    }
}

fn run(prog: &[Instruction], input: &str) -> Regs {
    let mut regs = Regs::new();
    run_w_regs(prog, input, &mut regs, false);
    regs
}

fn run_w_regs(prog: &[Instruction], input: &str, regs: &mut Regs, debug: bool) {
    let mut input = input.chars();

    for instruction in prog {
        use Instruction::*;

        let (v, w) = match instruction {
            Inp(v) => {
                let digit = (input.next().unwrap()).to_digit(10).unwrap();
                regs.write(*v, digit.into())
            }
            Add(v, a) => {
                regs.write(*v, regs.read(*v) + regs.read_arg(a))
            }
            Mul(v, a) => {
                regs.write(*v, regs.read(*v) * regs.read_arg(a))
            }
            Div(v, a) => {
                regs.write(*v, regs.read(*v) / regs.read_arg(a))
            }
            Mod(v, a) => {
                regs.write(*v, regs.read(*v) % regs.read_arg(a))
            }
            Eql(v, a) => {
                if regs.read(*v) == regs.read_arg(a) {
                    regs.write(*v, 1)
                } else {
                    regs.write(*v, 0)
                }
            }
        };
        if debug {
            let ins = instruction.to_string();
            let pad = &"           "[ins.len()..];
            println!("{} {}# {} = {}", ins, pad, v, w);
        }
    }
}

fn chunk_prog(input: &Input) -> Vec<Vec<Instruction>> {
    let mut result = Vec::new();
    for instruction in input {
        if matches!(instruction, Instruction::Inp(_)) {
            result.push(Vec::new());
        }
        result.last_mut().unwrap().push(instruction.clone());
    }

    result
}

fn part_a(input: &Input) -> i64 {
    println!("{:?}", run(&input, "12345678912345"));
    2
}

fn part_b(input: &Input) -> i64 {
    2
}

aoc::aoc!(parser, part_a, part_b, Some(12521), Some(444356092776315));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_futility() {
        let input = parser(&Path::new("input")).unwrap();
        // Got to: 99995149336576
        let mut n: i64 = 99999999999999;

        while n > 0 {
            let n_str = n.to_string();
            if n_str.chars().any(|c| c == '0') {
                n -= 1;
                continue;
            } else {
                let regs = run(&input, &n_str);
                let accept = regs.z == 0;
                let what = if accept {
                    "accept"
                } else {
                    "reject"
                };
                if n & 0xffff == 0 {
                    println!("{} -> {}", n, what);
                }
                if accept {
                    return;
                }
                n -= 1;
            }
        }
    }

    struct MIter(i64);

    impl Iterator for MIter {
        type Item = std::ops::Range<i64>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.0 < 0 {
                return None;
            }
            self.0 -= 100_000;
            let range = std::ops::Range {
                start: self.0,
                end: self.0 + 100_000,
            };
            Some(range)
        }
    }

    #[test]
    fn test_par() {
        let input = parser(&Path::new("input")).unwrap();
        let miter = MIter(10000_00000_00000);

        /*
        miter.par_iter().for_each(|r| {
            for n in r.rev() {
                let n_str = n.to_string();
                if n_str.chars().any(|c| c == '0') {
                    continue;
                } else {
                    let regs = run(&input, &n_str);
                    let accept = regs.z == 0;
                    let what = if accept {
                        "accept"
                    } else {
                        "reject"
                    };
                    if n & 0xffff == 0 {
                        println!("{} -> {}", n, what);
                    }
                    if accept {
                        return;
                    }
                }
            }
        }
        */
    }

    fn merge_chunks(chunks: &[Vec<Instruction>]) -> Vec<Instruction> {
        let mut p = Vec::new();
        for chunk in chunks {
            p.extend(chunk.iter().cloned());
        }
        p
    }

    #[test]
    fn test_solve_by_chunks_1() {
        let input = parser(&Path::new("input")).unwrap();
        let chunks = chunk_prog(&input);

        //let depth: u8 = 9;
        let depth: u8 = 1;
        let full_prog = merge_chunks(&chunks[0..usize::from(depth)]);
        let max = i64::pow(10, u32::from(depth));
        let min = i64::pow(10, u32::from(depth - 1));
        let mut n = max;
        loop {
            n -= 1;
            if n < min {
                break;
            }
            let n_str = n.to_string();
            if n_str.chars().any(|ch| ch == '0') {
                continue;
            }
            let regs = run(&full_prog, &n_str);
            println!("r: {}, {:?}", n, regs);
            if regs.z == 0 {
                println!("FIND: {}, {:?}", n, regs);
                break;
            }
        }
    }

    fn find_inputs_to(chunk: &[Instruction], target_z: i64) -> Vec<(i64, char)> {
        let mut output = Vec::new();
        for z in -2000 ..= 2000 {
            for n_str in &["1", "2", "3", "4", "5", "6", "7", "8", "9"] {
                let mut regs = Regs::new();
                regs.z = z;
                run_w_regs(chunk, &n_str, &mut regs, false);
                if regs.z == target_z {
                    output.push((z, n_str.chars().next().unwrap()));
                }
            }
        }
        output
    }

    fn extend_set(next_chunk: &[Instruction], targets: &[(i64, String)]) -> Vec<(i64, String)> {
        let mut output = Vec::new();

        for (target_z, trailer) in targets {
            let chunk_output = find_inputs_to(next_chunk, *target_z);
            for (tz, ch) in chunk_output {
                output.push((tz, format!("{}{}", ch, trailer)));
            }
        }

        output
    }

    #[test]
    fn test_solve_by_chunks_2() {
        let input = parser(&Path::new("input")).unwrap();
        let chunks = chunk_prog(&input);

        let chunk = &chunks[chunks.len() - 1];
        let mut regs = Regs::new();
        run_w_regs(chunk, "9", &mut regs, true);
        println!("Test: {}, z = {}, {:?}", '9', 0, regs);

        let inp_13 = find_inputs_to(&chunks[13], 0);
        for (z, ch) in inp_13.iter() {
            println!("({}, {}) to chunk 13 -> z = 0", z, ch);
        }
        let inp_13 = inp_13.into_iter().map(|(z, ch)| (z, ch.to_string())).collect::<Vec<_>>();
        let inp_12 = extend_set(&chunks[12], &inp_13);

        /*
        let inp_12 = {
            let mut v = Vec::new();
            for z in inp_13.iter().map(|(z, _)| *z) {
                let mut inp_12 = find_inputs_to(&chunks[12], z);
                v.extend(inp_12);
            }
            v
        };
        */
        for (z, ch) in inp_12.iter() {
            println!("({}, {}) to chunk 12 -> z = ?", z, ch);
        }
        println!("{} for that list.", inp_12.len());
        let inp_11 = extend_set(&chunks[11], &inp_12);
        println!("{} for that [11] list.", inp_11.len());
        let inp_10 = extend_set(&chunks[10], &inp_11);
        println!("{} for that [10] list.", inp_10.len());
    }
}
