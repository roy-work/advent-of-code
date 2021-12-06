use aoc::prelude::*;

#[derive(Clone)]
struct Input {
    moves: Vec<i64>,
    boards: Vec<Board>,
}

#[derive(Clone)]
struct Board {
    // [y][x]
    d: Vec<Vec<(i64, bool)>>,
}

fn parser(path: &Path) -> anyhow::Result<Input> {
    let mut lines = BufReader::new(File::open(path)?).lines();

    let moves = lines
        .next()
        .unwrap()?
        .split(',')
        .map(|i| -> i64 { i.parse().unwrap() })
        .collect::<Vec<_>>();

    let mut boards = Vec::new();

    loop {
        if let None = lines.next() {
            break;
        }

        let mut board = Board {
            d: Vec::new(),
        };
        for _ in 0..5 {
            let row = lines
                .next()
                .unwrap()?
                .split_whitespace()
                .map(|i| -> (i64, bool) { (i.parse().unwrap(), false) })
                .collect::<Vec<_>>();
            board.d.push(row);
        }

        boards.push(board);
    }

    Ok(Input { moves, boards })
}

fn mark_move(board: &mut Board, mv: i64) {
    for row in board.d.iter_mut() {
        for cell in row.iter_mut() {
            if cell.0 == mv {
                cell.1 = true;
            }
        }
    }
}

fn has_won(board: &Board) -> bool {
    for row in board.d.iter() {
        if row.iter().all(|c| c.1) {
            return true;
        }
    }

    for col in 0..5 {
        let mut all_true = true;
        for row in 0..5 {
            if !board.d[row][col].1 {
                all_true = false;
            }
        }
        if all_true {
            return true;
        }
    }
    false
}

fn score_board(board: &Board) -> i64 {
    let mut sum = 0;
    for row in board.d.iter() {
        for cell in row.iter() {
            if !cell.1 {
                sum += cell.0;
            }
        }
    }
    sum
}

fn part_a(input: &Input) -> i64 {
    let mut boards = input.boards.clone();
    for mv in input.moves.iter() {
        for board in boards.iter_mut() {
            mark_move(board, *mv);
            if has_won(board) {
                return score_board(board) * mv;
            }
        }
    }
    panic!()
}

fn part_b(input: &Input) -> i64 {
    let mut boards = input.boards.clone();
    let mut boards_left = boards.len();
    for mv in input.moves.iter() {
        let mut this_boards = Vec::new();
        std::mem::swap(&mut this_boards, &mut boards);
        for mut board in this_boards {
            mark_move(&mut board, *mv);
            if has_won(&board) {
                if boards_left == 1 {
                    return score_board(&board) * mv;
                } else {
                    boards_left -= 1;
                }
            } else {
                boards.push(board);
            }
        }
    }
    panic!()
}

aoc::aoc!(parser, part_a, part_b, Some(4512), Some(1924));
