use std::collections::{hash_map, VecDeque};
use std::convert::TryFrom;
use std::iter::FromIterator;

use anyhow::Context;
use once_cell::sync::Lazy;
use regex::Regex;

use aoc::prelude::*;

aoc::lazy_regex!(FOOD_RE, "^([a-z]+(:? [a-z]+)*) \\(contains ([^)]+)\\)$");

#[derive(Clone)]
struct Input {
    player_a_deck: VecDeque<i64>,
    player_b_deck: VecDeque<i64>,
}

impl Input {
    fn game_to_hash(&self) -> String {
        let mut output = String::new();

        for n in self.player_a_deck.iter() {
            output.push_str(&format!("{},", n));
        }
        output.push_str("|");
        for n in self.player_b_deck.iter() {
            output.push_str(&format!("{},", n));
        }
        output
    }
}

fn parser(path: &Path) -> anyhow::Result<Input> {
    let file = BufReader::new(File::open(path)?);
    let mut lines = file.lines();

    lines.next().unwrap()?;
    let mut player_a_deck = VecDeque::new();
    loop {
        let line = lines.next().unwrap()?;
        if line.is_empty() {
            break;
        }

        player_a_deck.push_back(line.parse::<i64>().unwrap());
    }

    lines.next().unwrap()?;
    let mut player_b_deck = VecDeque::new();
    while let Some(line) = lines.next() {
        let line = line?;

        player_b_deck.push_back(line.parse::<i64>().unwrap());
    }

    Ok(Input {
        player_a_deck,
        player_b_deck,
    })
}

fn play_game(input: &Input) -> Input {
    let mut input = input.clone();

    let mut round = 1;

    while !input.player_a_deck.is_empty() && !input.player_b_deck.is_empty() {
        println!("Player A deck: {:?}", input.player_a_deck);
        println!("Player B deck: {:?}", input.player_b_deck);

        let player_a_card = input.player_a_deck.pop_front().unwrap();
        let player_b_card = input.player_b_deck.pop_front().unwrap();

        println!("Player A plays: {}", player_a_card);
        println!("Player B plays: {}", player_b_card);

        if player_b_card < player_a_card {
            println!("Player A wins!");
            input.player_a_deck.push_back(player_a_card);
            input.player_a_deck.push_back(player_b_card);
        } else if player_a_card < player_b_card {
            println!("Player B wins!");
            input.player_b_deck.push_back(player_b_card);
            input.player_b_deck.push_back(player_a_card);
        } else {
            panic!()
        }
    }

    input
}

fn will_recurse(input: &Input, player_a_card: i64, player_b_card: i64) -> bool {
    input.player_a_deck.len() as i64 >= player_a_card
        && input.player_b_deck.len() as i64 >= player_b_card
}

fn play_game_recurse(input: &Input, game: &mut usize, configs_seen: &mut HashSet::<String>) -> (Input, bool) {
    let mut input = input.clone();

    let mut round = 1;

    //println!("=== Game {} ===", game);
    //println!();
    let this_game = *game;

    while !input.player_a_deck.is_empty() && !input.player_b_deck.is_empty() {
        /*
        println!("Player 1's deck: {:?}", input.player_a_deck);
        println!("Player 2's deck: {:?}", input.player_b_deck);
        */

        let game_state = input.game_to_hash();
        if configs_seen.contains(&game_state) {
            return (input, true);
        }
        configs_seen.insert(game_state);

        let player_a_card = input.player_a_deck.pop_front().unwrap();
        let player_b_card = input.player_b_deck.pop_front().unwrap();

        /*
        println!("Player 1 plays: {}", player_a_card);
        println!("Player 2 plays: {}", player_b_card);
        */

        let a_wins = if will_recurse(&input, player_a_card, player_b_card) {
            // recurse.
            let new_input = Input {
                player_a_deck: input
                    .player_a_deck
                    .iter()
                    .take(player_a_card as usize)
                    .copied()
                    .collect::<VecDeque<_>>(),
                player_b_deck: input
                    .player_b_deck
                    .iter()
                    .take(player_b_card as usize)
                    .copied()
                    .collect::<VecDeque<_>>(),
            };
            *game += 1;
            //let (_, winner) = play_game_recurse(&new_input, game, configs_seen);
            let (_, winner) = play_game_recurse(&new_input, game, &mut HashSet::new());
            //println!(" (subgame winner: {})", winner);
            winner
        } else {
            player_b_card < player_a_card
        };

        if a_wins {
            //println!("Player 1 wins round {} of game {}!", round, this_game);
            input.player_a_deck.push_back(player_a_card);
            input.player_a_deck.push_back(player_b_card);
        } else {
            //println!("Player 2 wins round {} of game {}!", round, this_game);
            input.player_b_deck.push_back(player_b_card);
            input.player_b_deck.push_back(player_a_card);
        }

        round += 1;
        //println!();
    }

    let a_wins = if !input.player_a_deck.is_empty() {
        true
    } else if !input.player_b_deck.is_empty() {
        false
    } else {
        panic!()
    };
    (input, a_wins)
}

fn part_a(input: &Input) -> i64 {
    let results = play_game(input);
    let winning_deck = {
        if !results.player_a_deck.is_empty() {
            &results.player_a_deck
        } else if !results.player_b_deck.is_empty() {
            &results.player_b_deck
        } else {
            panic!()
        }
    };

    winning_deck
        .iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i as i64 + 1) * c)
        .sum()
}

fn part_b(input: &Input) -> i64 {
    let (results, a_wins) = play_game_recurse(
        input,
        &mut 1,
        &mut HashSet::new(),
    );
    let winning_deck = {
        if a_wins {
            &results.player_a_deck
        } else {
            &results.player_b_deck
        }
    };

    winning_deck
        .iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i as i64 + 1) * c)
        .sum()
}

aoc::aoc!(parser, part_a, part_b, Some(306), Some(291));
