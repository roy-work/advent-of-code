#[allow(unused_imports)]
use aoc::prelude::*;

struct Input {
    moves: Vec<(Move, Move)>,
}

#[derive(Clone, Copy, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn score(&self) -> i64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}


fn parser(input_file: &aoc::InputFile<'_>) -> anyhow::Result<Input> {
    let mut moves = Vec::new();

    for line in input_file.lines()? {
        let line = line?;
        let (a, b) = line.split_once(' ').unwrap();
        let them = match a {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!(),
        };
        let us = match b {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => panic!(),
        };
        moves.push((them, us));
    }

    Ok(Input {
        moves,
    })
}

#[derive(Clone, Copy, Debug)]
enum WinLose {
    Win,
    Lose,
    Draw,
}

fn we_win(them: Move, us: Move) -> WinLose {
    use Move::*;
    use WinLose::*;
    match (them, us) {
        (Rock, Rock) => Draw,
        (Rock, Paper) => Win,
        (Rock, Scissors) => Lose,
        (Paper, Rock) => Lose,
        (Paper, Paper) => Draw,
        (Paper, Scissors) => Win,
        (Scissors, Rock) => Win,
        (Scissors, Paper) => Lose,
        (Scissors, Scissors) => Draw,
    }
}

fn part_a(input: &Input) -> i64 {
    let mut score_them = 0;
    let mut score_us = 0;

    for (them, us) in input.moves.iter() {
        let win = we_win(*them, *us);
        //println!("{:?} v {:?}, {:?}", them, us, win);
        match win {
            WinLose::Draw => {
                score_them += 3 + them.score();
                score_us += 3 + us.score();
            }
            WinLose::Lose => {
                score_them += 6 + them.score();
                score_us += 0 + us.score();
            }
            WinLose::Win => {
                score_them += 0 + them.score();
                score_us += 6 + us.score();
            }
        }
    }
    score_us
}

fn move_to_outcome(mv: Move) -> WinLose {
    match mv {
        Move::Rock => WinLose::Lose,
        Move::Paper => WinLose::Draw,
        Move::Scissors => WinLose::Win,
    }
}

fn determine_move(them: Move, outcome: WinLose) -> Move {
    use Move::*;
    use WinLose::*;
    match (them, outcome) {
        (Rock, Lose) => Scissors,
        (Rock, Draw) => Rock,
        (Rock, Win) => Paper,
        (Paper, Lose) => Rock,
        (Paper, Draw) => Paper,
        (Paper, Win) => Scissors,
        (Scissors, Lose) => Paper,
        (Scissors, Draw) => Scissors,
        (Scissors, Win) => Rock,
    }
}

fn part_b(input: &Input) -> i64 {
    let mut score_them = 0;
    let mut score_us = 0;

    for (them, win) in input.moves.iter().map(|(t, m)| (t, move_to_outcome(*m))) {
        let us = determine_move(*them, win);
        //println!("{:?} v {:?}, {:?}", them, us, win);
        match win {
            WinLose::Draw => {
                score_them += 3 + them.score();
                score_us += 3 + us.score();
            }
            WinLose::Lose => {
                score_them += 6 + them.score();
                score_us += 0 + us.score();
            }
            WinLose::Win => {
                score_them += 0 + them.score();
                score_us += 6 + us.score();
            }
        }
    }
    score_us
}

aoc::aoc!(parser, part_a, part_b, Some(15), Some(12));

#[cfg(test)]
mod tests {
    use super::*;
}
