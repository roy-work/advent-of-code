use std::collections::HashSet;

use once_cell::sync::Lazy;
use regex::Regex;

use aoc::prelude::*;

#[derive(Eq, Hash, PartialEq)]
struct Rule {
    color: String,
    contains: Vec<(u64, String)>,
}

static BAG_RE: Lazy<Regex> = Lazy::new(|| Regex::new("^(.+) bags contain (.+).$").unwrap());
static INNER_BAG_RE: Lazy<Regex> = Lazy::new(|| Regex::new("^(\\d+) (.+) bags?$").unwrap());

fn parse_rule(line: &str) -> anyhow::Result<Rule> {
    let captures = BAG_RE.captures(line).expect("BAG_RE no match");
    let color = captures.get(1).unwrap().as_str().to_string();
    let remain = captures.get(2).unwrap().as_str();

    let mut contains = Vec::new();

    if remain == "no other bags" {
        return Ok(Rule {
            color,
            contains,
        });
    }

    for part in remain.split(", ") {
        let captures = INNER_BAG_RE.captures(part).ok_or_else(|| anyhow::anyhow!("INNER_BAG_RE"))?;
        let qty = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
        let color = captures.get(2).unwrap().as_str().to_string();
        contains.push((qty, color));
    }
    Ok(Rule {
        color,
        contains,
    })
}

fn compute_contains(rules: &[Rule]) -> u32 {
    let mut new_colors = vec!["shiny gold"];
    let mut can_contain = HashSet::new();
    //can_contain.insert("shiny_gold");
    while !new_colors.is_empty() {
        let mut old_colors = Vec::new();
        std::mem::swap(&mut old_colors, &mut new_colors);
        for color in old_colors {
            for rule in rules.iter() {
                if rule.contains.iter().any(|(_, c)| c == color) {
                    println!("{} can contain {} (and thus, indirectly, shiny gold)", rule.color, color);
                    can_contain.insert(&rule.color);
                    new_colors.push(&rule.color);
                }
            }
        }
    }
    can_contain.len() as u32
}

fn compute_inner_bags(rules: &[Rule]) -> u64 {
    let mut inner_qty = HashMap::<String, u64>::new();
    use std::collections::hash_map::Entry;

    let mut unsolved = rules.iter().collect::<HashSet::<_>>();

    loop {
        let mut solved = HashSet::<&Rule>::new();

        'rules: for rule in unsolved.iter() {
            let mut our_qty = 0;
            println!("considering: {}", rule.color);
            for (qty, color) in rule.contains.iter() {
                match inner_qty.get(color) {
                    Some(rc_qty) => our_qty += qty * (rc_qty + 1),
                    None => continue 'rules,
                }
            }
            inner_qty.insert(rule.color.clone(), our_qty);
            solved.insert(rule);
            println!("Solved {}", rule.color);
            if rule.color == "shiny gold" {
                return our_qty;
            }
        }

        for rule in solved.iter() {
            unsolved.remove(rule);
        }
        if solved.len() == 0 {
            panic!("Can't solve.");
        }
    }
}

fn main() {
    let rules = aoc::file_item_per_line("input", parse_rule).unwrap();
    println!("Number containing gold: {}", compute_contains(&rules));
    println!("PPPAAARRT TWO");
    println!("Gold contents: {}", compute_inner_bags(&rules));
}
