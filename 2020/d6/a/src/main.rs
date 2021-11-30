use std::collections::HashSet;

use once_cell::sync::Lazy;
use regex::Regex;

use aoc::prelude::*;

fn parse_questions() -> anyhow::Result<Vec<HashSet<char>>> {
    let file = BufReader::new(File::open("input")?);

    let mut groups = Vec::new();
    let mut current_group = HashSet::new();
    for line in file.lines() {
        let line = line?;

        if line.is_empty() {
            let mut old_group = HashSet::new();
            std::mem::swap(&mut old_group, &mut current_group);
            groups.push(old_group);
            continue;
        }

        for c in line.chars() {
            current_group.insert(c);
        }
    }
    groups.push(current_group);

    Ok(groups)
}

fn group_to_set(group: &HashMap<char, u32>, group_members: u32) -> HashSet<char> {
    let mut all_yes = HashSet::new();
    for (k, v) in group.iter() {
        if *v == group_members {
            all_yes.insert(*k);
        }
    }
    all_yes
}

fn parse_questions_2() -> anyhow::Result<Vec<HashSet<char>>> {
    let file = BufReader::new(File::open("input")?);

    let mut groups = Vec::new();
    let mut current_group = HashMap::new();
    let mut group_members = 0;
    for line in file.lines() {
        let line = line?;

        if line.is_empty() {
            let mut old_group = HashMap::new();
            std::mem::swap(&mut old_group, &mut current_group);
            let all_yes = group_to_set(&old_group, group_members);
            println!("Old group: {:?}", old_group);
            println!("all yes: {:?}", all_yes);
            println!("members: {}", group_members);
            groups.push(all_yes);
            group_members = 0;
            continue;
        }

        for c in line.chars() {
            *current_group.entry(c).or_insert(0) += 1;
        }
        group_members += 1;
    }
    groups.push(group_to_set(&current_group, group_members));

    Ok(groups)
}

fn main() {
    let groups = parse_questions().unwrap();
    println!("sum: {}", groups.iter().map(|g| g.len()).sum::<usize>());

    let groups = parse_questions_2().unwrap();
    println!("sum: {}", groups.iter().map(|g| g.len()).sum::<usize>());
}
