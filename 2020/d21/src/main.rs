use std::collections::{hash_map, VecDeque};
use std::convert::TryFrom;
use std::iter::FromIterator;

use anyhow::Context;
use once_cell::sync::Lazy;
use regex::Regex;

use aoc::prelude::*;

aoc::lazy_regex!(FOOD_RE, "^([a-z]+(:? [a-z]+)*) \\(contains ([^)]+)\\)$");

struct Food {
    ingredients: Vec<String>,
    allergens: HashSet<String>,
}
type Input = Vec<Food>;

fn parser(path: &Path) -> anyhow::Result<Input> {
    let file = BufReader::new(File::open(path)?);
    let mut lines = file.lines();

    let mut foods = Vec::new();

    for line in lines {
        let line = line?;
        let captures = FOOD_RE.captures(&line).unwrap();
        /*
        println!("get(1) => {:?}", captures.get(1).unwrap().as_str());
        println!("get(2) => {:?}", captures.get(3).unwrap().as_str());
        */
        let ingredients = captures
            .get(1)
            .unwrap()
            .as_str()
            .split(' ')
            .map(|f| f.to_owned())
            .collect::<Vec<_>>();

        let allergens = captures
            .get(3) // ???
            .unwrap()
            .as_str()
            .split(", ")
            .map(|a| a.to_owned())
            .collect::<HashSet<_>>();

        foods.push(Food {
            ingredients,
            allergens,
        });
    }
    Ok(foods)
}

fn common(input: &Input) -> (HashMap<&str, &str>, HashSet<&str>) {
    let all_ingredients = input
        .iter()
        .map(|f| f.ingredients.iter().map(|i| i.as_str()))
        .flatten()
        .collect::<HashSet<_>>();
    let all_allergens = input
        .iter()
        .map(|f| f.allergens.iter().map(|a| a.as_str()))
        .flatten()
        .collect::<HashSet<_>>();

    let mut ingredient_to_allergen = HashMap::<&str, &str>::new();
    let mut allergens_remaining = all_allergens.iter().copied().collect::<VecDeque<&str>>();
    let mut loops = 0;
    while let Some(allergen) = allergens_remaining.pop_front() {
        println!("Thinking about {}", allergen);
        let mut candidate_ingredients = all_ingredients.iter().copied().collect::<HashSet<&str>>();

        for ingredient in ingredient_to_allergen.keys() {
            candidate_ingredients.remove(ingredient);
        }

        for food in input {
            if !food.allergens.contains(allergen) {
                continue;
            }
            let ingredients = food
                .ingredients
                .iter()
                .map(|i| i.as_str())
                .collect::<HashSet<&str>>();
            for ingredient in all_ingredients.difference(&ingredients) {
                candidate_ingredients.remove(ingredient);
            }
        }
        println!("Remaining: {:?}", candidate_ingredients);

        if candidate_ingredients.len() == 1 {
            let allergen_ingredient = candidate_ingredients.iter().next().unwrap();
            ingredient_to_allergen.insert(allergen_ingredient, allergen);
            println!("{} => {}", allergen_ingredient, allergen);
            loops = 0;
        } else {
            allergens_remaining.push_back(allergen);
            loops += 1;
        }

        if loops == 20 {
            panic!();
        }
    }

    let allergen_free_ingredients = {
        let mut afi = all_ingredients.clone();
        for ingredient in ingredient_to_allergen.keys() {
            afi.remove(ingredient);
        }
        afi
    };
    println!("Allergen free: {:?}", allergen_free_ingredients);

    (ingredient_to_allergen, allergen_free_ingredients)
}

fn part_a(input: &Input) -> i64 {
    let (_, allergen_free_ingredients) = common(input);
    let mut sum = 0;
    for food in input {
        for ingredient in food.ingredients.iter() {
            if allergen_free_ingredients.contains(ingredient.as_str()) {
                sum += 1;
            }
        }
    }
    sum
}

fn part_b(input: &Input) -> i64 {
    let (ingredient_to_allergen, allergen_free_ingredients) = common(input);

    let mut output = ingredient_to_allergen
        .iter()
        .map(|(k, v)| (*k, *v))
        .collect::<Vec<(&str, &str)>>();
    output.sort_by_key(|i| i.1);
    let output = output.iter().map(|(i, _)| *i).collect::<Vec<&str>>();
    println!(
        "ANS: {}",
        output.join(","),
    );
    2
}

aoc::aoc!(parser, part_a, part_b, Some(5), None);
