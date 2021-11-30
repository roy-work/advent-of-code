use std::collections::{hash_map, VecDeque};
use std::convert::TryFrom;
use std::iter::FromIterator;

use anyhow::Context;
use once_cell::sync::Lazy;
use regex::Regex;

use aoc::prelude::*;

aoc::lazy_regex!(RULE_RE, "^(\\d+): (.+)$");
aoc::lazy_regex!(RULE_CONST, "^\"(a|b)\"$");
aoc::lazy_regex!(RULE_ONE, "^(\\d+)$");
aoc::lazy_regex!(RULE_TWO, "^(\\d+) (\\d+)$");
aoc::lazy_regex!(RULE_OR, "^([ 0-9]+) \\| ([ 0-9]+)$");

type Grammar = HashMap<i64, Rule>;

enum Rule {
    Const(char),
    Or { rule1: Vec<i64>, rule2: Vec<i64> },
    One(i64),
    Two(i64, i64),
}

struct Input {
    grammar: Grammar,
    inputs: Vec<String>,
}

fn parser(path: &Path) -> anyhow::Result<Input> {
    let file = BufReader::new(File::open(path)?);
    let mut lines = file.lines();

    let mut rules = HashMap::<i64, Rule>::new();
    loop {
        let line = lines.next().unwrap()?;
        println!("parsing: {}", line);
        if line.is_empty() {
            break;
        }

        let captures = RULE_RE.captures(&line).unwrap();
        let id = captures
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i64>()
            .context("parse id")?;
        let remainder = captures.get(2).unwrap().as_str();
        println!(" -> rem: {}", remainder);
        let rule = {
            if let Some(cap_rule) = RULE_CONST.captures(remainder) {
                let ch = cap_rule.get(1).unwrap().as_str().chars().next().unwrap();
                Rule::Const(ch)
            } else if let Some(cap_rule) = RULE_ONE.captures(remainder) {
                let n1 = cap_rule.get(1).unwrap().as_str().parse::<i64>().unwrap();
                Rule::One(n1)
            } else if let Some(cap_rule) = RULE_TWO.captures(remainder) {
                let n1 = cap_rule.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let n2 = cap_rule.get(2).unwrap().as_str().parse::<i64>().unwrap();
                Rule::Two(n1, n2)
            } else if let Some(cap_rule) = RULE_OR.captures(remainder) {
                let rule1 = cap_rule
                    .get(1)
                    .unwrap()
                    .as_str()
                    .split(' ')
                    .map(|p| p.parse::<i64>().unwrap())
                    .collect::<Vec<_>>();
                let rule2 = cap_rule
                    .get(2)
                    .unwrap()
                    .as_str()
                    .split(' ')
                    .map(|p| p.parse::<i64>().unwrap())
                    .collect::<Vec<_>>();
                Rule::Or { rule1, rule2 }
            } else {
                panic!()
            }
        };

        rules.insert(id, rule);
    }

    let inputs = lines.collect::<Result<Vec<_>, _>>()?;

    Ok(Input {
        grammar: rules,
        inputs,
    })
}

fn part_a(input: &Input) -> i64 {
    input.inputs.iter().filter(|s| can_rules_produce(&input.grammar, &s)).count() as i64
}

fn can_rules_produce(grammar: &Grammar, s: &str) -> bool {
    let s = s.chars().collect::<Vec<_>>();

    let root_rule = grammar.get(&0).unwrap();
    for rem in consume_rule(grammar, root_rule, &s) {
        if rem.is_empty() {
            return true;
        }
    }
    false
}

fn consume_rule<'a>(grammar: &Grammar, rule: &Rule, s: &'a [char]) -> Vec<&'a [char]> {
    match rule {
        Rule::Const(ch) => {
            if s.is_empty() {
                vec![]
            } else if *ch == s[0] {
                vec![&s[1..]]
            } else {
                vec![]
            }
        }
        Rule::One(id) => {
            consume_rule(grammar, grammar.get(id).unwrap(), s)
        }
        Rule::Two(i1, i2) => {
            let rems = consume_rule(grammar, grammar.get(i1).unwrap(), s);
            let rule2 = grammar.get(i2).unwrap();
            process_one_concat(grammar, rule2, &rems)
        }
        Rule::Or { rule1, rule2 } => {
            let rule1_rems = process_many_concat(grammar, rule1, s);
            let rule2_rems = process_many_concat(grammar, rule2, s);
            let mut rem = vec![];
            rem.extend(rule1_rems);
            rem.extend(rule2_rems);
            rem
        }
    }
}

fn process_one_concat<'a>(grammar: &Grammar, rule: &Rule, rems: &[&'a [char]]) -> Vec<&'a [char]> {
    let mut our_rems = vec![];
    for rem in rems {
        let this_rems = consume_rule(grammar, rule, rem);
        our_rems.extend(this_rems);
    }
    our_rems
}

fn process_many_concat<'a>(grammar: &Grammar, rules: &[i64], s: &'a [char]) -> Vec<&'a [char]> {
    let mut rules = rules.iter();
    let rule = grammar.get(rules.next().unwrap()).unwrap();
    let mut rems = consume_rule(grammar, rule, s);
    for rule in rules {
        let rule = grammar.get(rule).unwrap();
        let new_rems = process_one_concat(grammar, rule, &rems);
        rems = new_rems;
    }
    rems
}

fn part_b(input: &Input) -> i64 {
    2
}

aoc::aoc!(parser, part_a, part_b, None, None);
