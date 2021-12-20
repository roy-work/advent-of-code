use std::fmt;

use aoc::prelude::*;

type Input = Vec<Pair>;

fn parser(path: &Path) -> anyhow::Result<Input> {
    let reader = BufReader::new(File::open(path)?);
    let lines = reader.lines();

    let mut pairs = Vec::new();
    for line in lines {
        let line = line?;
        let pair = parse_line(line.trim_end());
        pairs.push(pair);
    }
    Ok(pairs)
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Pair {
    a: PairElement,
    b: PairElement,
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.a, self.b)
    }
}

impl Pair {
    fn magnitude(&self) -> i64 {
        3 * self.a.magnitude() + 2 * self.b.magnitude()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum PairElement {
    Num(i64),
    Pair(Box<Pair>),
}

impl fmt::Display for PairElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PairElement::Num(n) => n.fmt(f),
            PairElement::Pair(p) => p.fmt(f),
        }
    }
}

impl PairElement {
    fn magnitude(&self) -> i64 {
        match self {
            PairElement::Num(n) => *n,
            PairElement::Pair(p) => p.magnitude(),
        }
    }
}

fn parse_line(s: &str) -> Pair {
    let mut stack = Vec::<Vec<PairElement>>::new();

    for c in s.chars() {
        match c {
            ',' => (),
            '[' => {
                stack.push(Vec::new());
            },
            ']' => {
                let mut top = stack.pop().unwrap().into_iter();
                let a = top.next().unwrap();
                let b = top.next().unwrap();
                assert!(top.next().is_none());
                let pair = Pair { a, b };
                if let Some(se) = stack.last_mut() {
                    se.push(PairElement::Pair(Box::new(pair)));
                } else {
                    return pair;
                }
            },
            '0' ..= '9' => {
                let p = PairElement::Num(i64::from(c.to_digit(10).unwrap()));
                stack.last_mut().unwrap().push(p);
            }
            _ => panic!(),
        }
    }
    panic!()
}

#[derive(Debug)]
enum ExplodeResult {
    NoBoom,
    Boom(i64, i64),
    Bubbling(Option<i64>, Option<i64>),
}

impl ExplodeResult {
    fn did_boom(&self) -> bool {
        use ExplodeResult::*;

        match self {
            NoBoom => false,
            Boom(_, _) => true,
            Bubbling(_, _) => true,
        }
    }
}

fn explode(pair: &mut Pair, depth: usize) -> ExplodeResult {
    use ExplodeResult::*;

    //println!("explode: {} ; d = {}", pair, depth);

    if depth < 4 {
        if let PairElement::Pair(a) = &mut pair.a {
            match explode(a, depth + 1) {
                NoBoom => (),
                Boom(a, b) => {
                    //println!("boom up");
                    pair.a = PairElement::Num(0);
                    set_left(&mut pair.b, b);
                    return Bubbling(Some(a), None);
                }
                Bubbling(a, mut b) => {
                    //println!("bubble up d = {}", depth);
                    if let Some(b) = b.take() {
                        set_left(&mut pair.b, b)
                    }
                    return Bubbling(a, b);
                }
            }
        }
        if let PairElement::Pair(b) = &mut pair.b {
            match explode(b, depth + 1) {
                NoBoom => (),
                Boom(a, b) => {
                    pair.b = PairElement::Num(0);
                    set_right(&mut pair.a, a);
                    return Bubbling(None, Some(b));
                }
                Bubbling(mut a, b) => {
                    if let Some(a) = a.take() {
                        set_right(&mut pair.a, a)
                    }
                    return Bubbling(a, b);
                }
            }
        }
        NoBoom
    } else {
        //println!("boom");
        let (a, b) = match (&pair.a, &pair.b) {
            (PairElement::Num(a), PairElement::Num(b)) => (*a, *b),
            _ => panic!(),
        };
        Boom(a, b)
    }
}

fn set_left(pair: &mut PairElement, n: i64) {
    match pair {
        PairElement::Num(inner) => *inner += n,
        PairElement::Pair(p) => set_left(&mut p.a, n),
    }
}

/*
fn set_left_r(pair: &mut PairElement, n: i64) {
    match pair {
        PairElement::Num(inner) => *inner += n,
        PairElement::Pair(p) => set_left_r(&mut p.b, n),
    }
}
*/

fn set_right(pair: &mut PairElement, n: i64) {
    match pair {
        PairElement::Num(inner) => *inner += n,
        PairElement::Pair(p) => set_right(&mut p.b, n),
    }
}

/*
fn set_right_r(pair: &mut PairElement, n: i64) {
    match pair {
        PairElement::Num(inner) => *inner += n,
        PairElement::Pair(p) => set_left(&mut p.a, n),
    }
}
*/

fn split_n(n: i64) -> PairElement {
    assert!(10 <= n);
    let div_two = n / 2;
    let (a, b) = {
        if n % 2 == 1 {
            (div_two, div_two + 1)
        } else {
            (div_two, div_two)
        }
    };
    PairElement::Pair(Box::new(Pair {
        a: PairElement::Num(a),
        b: PairElement::Num(b),
    }))
}

fn split(pair: &mut Pair) -> bool {
    match &mut pair.a {
        PairElement::Num(n) if 10 <= *n => {
            pair.a = split_n(*n);
            return true;
        }
        PairElement::Num(_) => (),
        PairElement::Pair(p) => {
            if split(p) {
                return true;
            }
        }
    }
    match &mut pair.b {
        PairElement::Num(n) if 10 <= *n => {
            pair.b = split_n(*n);
            return true;
        }
        PairElement::Num(_) => (),
        PairElement::Pair(p) => {
            if split(p) {
                return true;
            }
        }
    }
    false
}

fn reduce(pair: &mut Pair, debug: bool) {
    use ExplodeResult::*;
    if debug {
        println!("reduce: {}", pair);
    }
    loop {
        let r = explode(pair, 0);
        //println!("explode, r = {:?}", r);
        match r {
            NoBoom => (),
            _ => {
                if debug {
                    println!("after explode: {}", pair);
                }
                continue
            }
        }
        if !split(pair) {
            break;
        }
        if debug {
            println!("after split: {}", pair);
        }
    }
    if debug {
        println!("done: {}", pair);
    }
}

fn do_sum(input: &[Pair], debug: bool) -> Pair {
    let mut input = input.iter().cloned().collect::<VecDeque<_>>();
    while input.len() > 1 {
        let a = input.pop_front().unwrap();
        let b = input.pop_front().unwrap();
        if debug {
            println!("  {}", a);
            println!("+ {}", b);
        }
        let mut pair = Pair {
            a: PairElement::Pair(Box::new(a)),
            b: PairElement::Pair(Box::new(b)),
        };
        reduce(&mut pair, debug);
        if debug {
            println!("= {}", pair);
            println!();
        }
        input.push_front(pair);
    }
    input.pop_front().unwrap()
}

fn part_a(input: &Input) -> i64 {
    let sum = do_sum(&input, false);
    println!("sum = {}", sum);
    println!("mag = {}", sum.magnitude());
    sum.magnitude()
}

fn part_b(input: &Input) -> i64 {
    let mut largest_mag = 0;
    for (aidx, a) in input.iter().enumerate() {
        for (bidx, b) in input.iter().enumerate() {
            if aidx == bidx {
                continue;
            }
            let mut sum = Pair {
                a: PairElement::Pair(Box::new(a.clone())),
                b: PairElement::Pair(Box::new(b.clone())),
            };
            reduce(&mut sum, false);
            let mag = sum.magnitude();
            largest_mag = std::cmp::max(largest_mag, mag);
        }
    }
    largest_mag
}

aoc::aoc!(parser, part_a, part_b, Some(4140), Some(3993));

#[cfg(test)]
mod tests {
    use super::{explode, parse_line, Pair};

    #[test]
    fn test_print() {
        let pair = parse_line("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
        let pstr = pair.to_string();
        assert!(
            pstr
            == "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]"
        );
    }

    fn do_explode_test(s: &str, expect: &str) {
        let mut pair = parse_line(s);
        let r = explode(&mut pair, 0);
        println!("in: {}", s);
        println!("  r: {:?}", r);
        println!("  after: {}", pair);
        assert!(r.did_boom());
        assert!(pair.to_string() == expect);
    }

    #[test]
    fn test_explode() {
        do_explode_test("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
        do_explode_test("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
        do_explode_test("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
        do_explode_test("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        do_explode_test("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");

        // Roy's
        do_explode_test(
            "[[[[4,0],[5,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
            "[[[[4,0],[5,4]],[[0,[7,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
        );
    }

    #[test]
    fn test_reduce() {
        let mut pair = parse_line("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        super::reduce(&mut pair, true);
        assert!(pair.to_string() == "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }


    #[test]
    fn test_mag() {
        fn do_mag_test(s: &str, expect: i64) {
            let pair = parse_line(s);
            assert!(pair.magnitude() == expect);
        }

        do_mag_test("[[1,2],[[3,4],5]]", 143);
        do_mag_test("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384);
        do_mag_test("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445);
        do_mag_test("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791);
        do_mag_test("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137);
        do_mag_test("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488);
    }

    fn to_list_pair(i: &[&str]) -> Vec<Pair> {
        i.iter().copied().map(parse_line).collect::<Vec<_>>()
    }

    #[test]
    fn test_add_easy_4() {
        const INPUTS: &[&str] = &[
            "[1,1]",
            "[2,2]",
            "[3,3]",
            "[4,4]",
        ];
        let input = to_list_pair(INPUTS);
        let result = super::do_sum(&input, false);
        println!("result: {}", result);
        assert!(result.to_string() == "[[[[1,1],[2,2]],[3,3]],[4,4]]");
    }

    #[test]
    fn test_add_easy_5() {
        const INPUTS: &[&str] = &[
            "[1,1]",
            "[2,2]",
            "[3,3]",
            "[4,4]",
            "[5,5]",
        ];
        let input = to_list_pair(INPUTS);
        let result = super::do_sum(&input, false);
        println!("result: {}", result);
        assert!(result.to_string() == "[[[[3,0],[5,3]],[4,4]],[5,5]]");
    }

    #[test]
    fn test_add_easy_6() {
        const INPUTS: &[&str] = &[
            "[1,1]",
            "[2,2]",
            "[3,3]",
            "[4,4]",
            "[5,5]",
            "[6,6]",
        ];
        let input = to_list_pair(INPUTS);
        let result = super::do_sum(&input, false);
        println!("result: {}", result);
        assert!(result.to_string() == "[[[[5,0],[7,4]],[5,5]],[6,6]]");
    }

    #[test]
    fn test_add_medium() {
        const INPUTS: &[&str] = &[
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ];
        let input = to_list_pair(INPUTS);
        let result = super::do_sum(&input, true);
        println!("result: {}", result);
        assert!(result.to_string() == "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
        assert!(result.magnitude() == 3488);
    }

    #[test]
    fn test_add_example() {
        const INPUTS: &[&str] = &[
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ];
        let input = to_list_pair(INPUTS);
        let result = super::do_sum(&input, true);
        println!("result: {}", result);
        assert!(result.to_string() == "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]");
        assert!(result.magnitude() == 4140);
    }
}
