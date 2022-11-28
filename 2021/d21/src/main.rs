use aoc::map::{FreeCoord, Map};
use aoc::prelude::*;

struct Input {
    player_1_pos: i64,
    player_2_pos: i64,
}

aoc::hot_parse!(player_line, "^Player ([0-9]) starting position: ([0-9]|10)$", { 1 => u8, 2 => i64, }, |t| t);

fn parser(path: &Path) -> anyhow::Result<Input> {
    let reader = BufReader::new(File::open(path)?);
    let mut lines = reader.lines();

    let line = lines.next().unwrap()?;
    let (player_1, p1_pos) = player_line(line.trim_end()).unwrap();
    let line = lines.next().unwrap()?;
    let (player_2, p2_pos) = player_line(line.trim_end()).unwrap();
    assert!(lines.next().is_none());

    assert!(player_1 == 1);
    assert!(player_2 == 2);

    Ok(Input {
        player_1_pos: p1_pos,
        player_2_pos: p2_pos,
    })
}

struct Die {
    next_roll: i64,
    total_rolls: i64,
}

impl Die {
    fn new() -> Die {
        Die {
            next_roll: 1,
            total_rolls: 0,
        }
    }
}

impl Iterator for Die {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let roll = self.next_roll;
        self.next_roll = (self.next_roll + 1);
        if 100 < self.next_roll {
            self.next_roll = 1;
        }
        self.total_rolls += 1;
        Some(roll)
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Player {
    pos: i64,
    score: i64,
}

impl Player {
    fn short_repr(&self) -> String {
        format!("p: {}, s:{}", self.pos, self.score)
    }
}

struct GameResult {
    winner: Player,
    loser: Player,
    die: Die,
}

fn other_player(cur: usize) -> usize {
    match cur {
        0 => 1,
        1 => 0,
        _ => panic!(),
    }
}

fn simulate(input: &Input) -> GameResult {
    let mut players = [
        Player {
            pos: input.player_1_pos,
            score: 0,
        },
        Player {
            pos: input.player_2_pos,
            score: 0,
        },
    ];
    let mut die = Die::new();
    let mut current_player = 0;

    loop {
        let die_sum = {
            let r1 = die.next().unwrap();
            let r2 = die.next().unwrap();
            let r3 = die.next().unwrap();
            r1 + r2 + r3
        };
        let new_pos = ((players[current_player].pos - 1) + die_sum) % 10 + 1;
        players[current_player].score += new_pos;
        players[current_player].pos = new_pos;

        if 1000 <= players[current_player].score {
            return GameResult {
                winner: players[current_player].clone(),
                loser: players[other_player(current_player)].clone(),
                die: die,
            };
        } else {
            current_player = other_player(current_player);
        }
    }
}

fn part_a(input: &Input) -> i64 {
    let result = simulate(input);
    result.loser.score * result.die.total_rolls
}

fn advance_player(player: &Player, die_sum: i64) -> Player {
    let new_pos = ((player.pos - 1) + die_sum) % 10 + 1;
    Player {
        pos: new_pos,
        score: player.score + new_pos,
    }
}

// Returns (sum, # of universes)
fn make_dirac_dice() -> Vec<(i64, i64)> {
    let mut output = HashMap::<i64, i64>::new();
    for d1 in 1..=3 {
        for d2 in 1..=3 {
            for d3 in 1..=3 {
                let sum = d1 + d2 + d3;
                *output.entry(sum).or_insert(0) += 1;
            }
        }
    }
    output.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>()
}

fn simulate_2(input: &Input) -> i64 {
    let mut universe_map: HashMap<(Player, Player, usize), i64> = HashMap::new();
    let dirac_die = make_dirac_dice();
    //let dirac_die = vec![(1, 1), (2, 1), (3, 1)];
    universe_map.insert(
        (
            Player {
                pos: input.player_1_pos,
                score: 0,
            },
            Player {
                pos: input.player_2_pos,
                score: 0,
            },
            0,
        ),
        1,
    );

    let mut player_1_wins = 0;
    let mut player_2_wins = 0;

    while !universe_map.is_empty() {
        //println!("Turn");
        let mut new_universes = HashMap::new();

        for ((player_1, player_2, whose_turn), count) in universe_map.iter() {
            let current_player = match whose_turn {
                0 => player_1,
                1 => player_2,
                _ => panic!(),
            };
            for (die_sum, new_universes_cnt) in dirac_die.iter() {
                let new_player = advance_player(current_player, *die_sum);
                if 21 <= new_player.score {
                    match whose_turn {
                        0 => player_1_wins += count * new_universes_cnt,
                        1 => player_2_wins += count * new_universes_cnt,
                        _ => panic!(),
                    }
                } else {
                    //println!(" {:?}", (player_1, player_2, whose_turn, count, new_player.clone()));
                    let (p1, p2) = match whose_turn {
                        0 => (new_player, player_2.clone()),
                        1 => (player_1.clone(), new_player),
                        _ => panic!(),
                    };
                    *new_universes.entry((p1, p2, other_player(*whose_turn))).or_insert(0) += count * new_universes_cnt;
                    //println!(" t = {}, rolled = {}", whose_turn + 1, die_sum);
                }
            }
        }
        universe_map = new_universes;
        for ((player_1, player_2, whose_turn), count) in universe_map.iter() {
            //println!("  (p1: {}, p2: {}, t = {}) x {}", player_1.short_repr(), player_2.short_repr(), whose_turn + 1, count);
        }
    }

    println!("wins:: p1: {}, p2: {}", player_1_wins, player_2_wins);
    std::cmp::max(player_1_wins, player_2_wins)
}

fn part_b(input: &Input) -> i64 {
    simulate_2(input)
}

aoc::aoc!(parser, part_a, part_b, Some(739785), Some(444356092776315));

#[cfg(test)]
mod tests {
    use super::*;
}
