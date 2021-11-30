use std::collections::HashSet;
use aoc::prelude::*;

fn main() {
    let items = aoc::file_item_per_line("input", |l| -> Result<String, &'static str> { Ok(l.to_string()) })
        .unwrap_or_else(|err| {
            panic!("failed to load input");
        });

    let tree_rows = items.iter().map(|r| {
        r.chars().map(|c| match c {
            '#' => true,
            '.' => false,
            _ => panic!("not tree: {}", c),
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let mut total_trees = 1;
    for (shift, row_shift) in &[(1,1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let mut pos = 0;
        let mut row_idx = 0;
        let mut trees: u128 = 0;
        while row_idx < tree_rows.len() {
            let row = &tree_rows[row_idx];
            if row[pos % row.len()] {
                trees += 1;
            }
            pos += shift;
            row_idx += row_shift;
        }
        total_trees *= trees;
    }
    println!("{}", total_trees);
}
