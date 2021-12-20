use std::cmp::Reverse;

use keyed_priority_queue::KeyedPriorityQueue;

use aoc::map::{FreeCoord, Map};
use aoc::prelude::*;

type Input = Map<u8>;

fn parser(path: &Path) -> anyhow::Result<Input> {
    let reader = BufReader::new(File::open(path)?);

    let mut map = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let row = line
            .trim_end()
            .chars()
            .map(|c| u8::try_from(c.to_digit(10).unwrap()).unwrap())
            .collect::<Vec<_>>();
        map.push(row);
    }

    Ok(Map(map))
}

/*
struct AStarCoord {
    coord: FreeCoord,
    cost_estimate: i64,
}

impl PartialEq for AStarCoord {
    fn eq(&self, other: &Self) -> bool {
        self.cost_estimate.eq(&other.cost_estimate)
    }
}

impl PartialOrd for AStarCoord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cost_estimate.partial_cmp(&other.cost_estimate)
    }
}

impl Ord for AStarCoord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost_estimate.cmp(&other.cost_estimate)
    }
}

impl Eq for AStarCoord {}
*/

/// The A* heuristic
fn h(cell: &FreeCoord, goal: &FreeCoord) -> i64 {
    (goal.x - cell.x).abs() + (goal.y - cell.y).abs()
}

fn a_star(input: &Input, goal: &FreeCoord) -> Option<Vec<FreeCoord>> {
    let start = FreeCoord { x: 0, y: 0 };

    let mut open_set = KeyedPriorityQueue::<FreeCoord, Reverse<i64>>::new();
    open_set.push(start.clone(), Reverse(h(&start, goal)));

    let mut came_from = HashMap::<FreeCoord, FreeCoord>::new();

    let mut g_score = HashMap::new();
    g_score.insert(start.clone(), 0);

    while let Some(coord) = open_set.pop() {
        let coord = coord.0;

        // See if we're at the goal:
        if coord == *goal {
            let mut total_path = Vec::new();
            total_path.push(coord);
            let mut current = coord;
            while let Some(n) = came_from.get(&current) {
                current = n.clone();
                total_path.push(n.clone());
            }
            total_path.reverse();
            return Some(total_path);
        }

        for neighbor in coord.bind(input).unwrap().adj_cardinal() {
            let neighbor_cost = i64::from(*input.at(&neighbor).unwrap());
            let tentative_score = g_score.get(&coord).unwrap() + neighbor_cost;
            let is_better = g_score
                .get(&neighbor.unbind())
                .map(|n| tentative_score < *n)
                .unwrap_or(true);
            if is_better {
                came_from.insert(neighbor.unbind(), coord);
                g_score.insert(neighbor.unbind(), tentative_score);
                let priority = Reverse(tentative_score + h(&neighbor.unbind(), goal));
                match open_set.entry(neighbor.unbind()) {
                    keyed_priority_queue::Entry::Occupied(occ) => {
                        occ.set_priority(priority);
                    }
                    keyed_priority_queue::Entry::Vacant(vacancy) => {
                        vacancy.set_priority(priority);
                    }
                }
            }
        }
    }
    None
}

fn part_a(input: &Input) -> i64 {
    let goal = FreeCoord {
        x: i64::try_from(input.width()).unwrap() - 1,
        y: i64::try_from(input.height()).unwrap() - 1,
    };
    let path = a_star(input, &goal).unwrap();
    let mut score = 0;
    for coord in &path[1..] {
        let score_here = i64::from(*input.at(coord.bind(input).unwrap()).unwrap());
        score += score_here;
    }
    //println!("{:#?}", path);
    score
}

fn part_b(input: &Input) -> i64 {
    let input = {
        let mut new_rows = Vec::new();
        for row_copy in 0..5 {
            for row in input.0.iter() {
                let mut new_row = Vec::new();
                for cell_copy in 0..5 {
                    for cell in row.iter() {
                        new_row.push((cell - 1 + cell_copy + row_copy) % 9 + 1);
                    }
                }
                new_rows.push(new_row);
            }
        }
        Map(new_rows)
    };
    let goal = FreeCoord {
        x: i64::try_from(input.width()).unwrap() - 1,
        y: i64::try_from(input.height()).unwrap() - 1,
    };
    let path = a_star(&input, &goal).unwrap();
    let mut score = 0;
    for coord in &path[1..] {
        let score_here = i64::from(*input.at(coord.bind(&input).unwrap()).unwrap());
        score += score_here;
    }
    score
}

aoc::aoc!(parser, part_a, part_b, Some(40), Some(315));
