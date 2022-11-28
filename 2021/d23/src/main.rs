use aoc::prelude::*;

struct Input {
    side_rooms: [[char; 2]; 4],
}

aoc::hot_parse!(cube_line, "^(on|off) x=(-?[0-9]+)..(-?[0-9]+),y=(-?[0-9]+)..(-?[0-9]+),z=(-?[0-9]+)..(-?[0-9]+)$", { 1 => String, 2 => i64, 3 => i64, 4 => i64, 5 => i64, 6 => i64, 7 => i64, }, |t| t);

fn parser(path: &Path) -> anyhow::Result<Input> {
    let reader = BufReader::new(File::open(path)?);
    let mut lines = reader.lines();

    lines.next().unwrap()?;
    lines.next().unwrap()?;

    let mut side_rooms = [[' '; 2]; 4];

    for idx in 0..2 {
        let mut side_hall = 0;
        let line = lines.next().unwrap()?;
        for ch in line.chars() {
            match ch {
                'A'..='D' => {
                    side_rooms[side_hall][idx] = ch;
                    side_hall += 1;
                }
                _ => (),
            }
        }
    }

    Ok(Input { side_rooms })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum HallwayPos {
    LeftOuter,
    LeftInner,
    AB,
    BC,
    CD,
    RightInner,
    RightOuter,
}

const ALL_HALLWAY_POS: &[HallwayPos] = &[
    HallwayPos::LeftOuter,
    HallwayPos::LeftInner,
    HallwayPos::AB,
    HallwayPos::BC,
    HallwayPos::CD,
    HallwayPos::RightInner,
    HallwayPos::RightOuter,
];

impl HallwayPos {
    fn x_coord(self) -> u8 {
        use HallwayPos::*;
        match self {
            LeftOuter => 0,
            LeftInner => 1,
            AB => 3,
            BC => 5,
            CD => 7,
            RightInner => 9,
            RightOuter => 10,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PodType {
    A,
    B,
    C,
    D,
}

impl PodType {
    fn hallway_x(self) -> u8 {
        match self {
            PodType::A => 2,
            PodType::B => 4,
            PodType::C => 6,
            PodType::D => 8,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BasicPosition {
    Hallway(HallwayPos),
    Side { which: PodType, lower: bool },
}

#[derive(Clone, Debug)]
enum Position {
    Hallway(HallwayPos),
    Side {
        which: PodType,
        lower: bool,
        locked: bool,
    },
}

impl Position {
    fn is_locked(&self) -> bool {
        match self {
            Position::Side { locked, .. } => *locked,
            Position::Hallway(_) => false,
        }
    }

    fn to_basic(&self) -> BasicPosition {
        match self {
            Position::Hallway(h) => BasicPosition::Hallway(*h),
            Position::Side { which, lower, .. } => BasicPosition::Side {
                which: *which,
                lower: *lower,
            },
        }
    }

    fn is_hallway(&self) -> bool {
        match self {
            Position::Hallway(h) => true,
            Position::Side { .. } => false,
        }
    }
}

fn distance_out_to_hallway(lower: bool) -> i64 {
    match lower {
        true => 2,
        false => 1,
    }
}

fn distance_out_to_hallway_tile(side_hall: PodType, lower: bool, hallway: HallwayPos) -> i64 {
    let initial = distance_out_to_hallway(lower);
    let initial_x = side_hall.hallway_x();
    let final_x = hallway.x_coord();
    (i64::from(final_x) - i64::from(initial_x)).abs() + initial
}

#[derive(Clone, Debug)]
struct Pod {
    ty: PodType,
    position: Position,
}

#[derive(Clone, Debug)]
struct State {
    pods: [Pod; 8],
    energy: i64,
}

impl State {
    fn is_occupied(&self, position: BasicPosition) -> bool {
        self.pods
            .iter()
            .any(|p| p.position.to_basic() == position)
    }

    fn

    fn moves_for(&self, pod_idx: usize, pod: &Pod, output: &mut Vec<State>) {
        if pod.position.is_locked() {
            return;
        }
        if pod.position.is_hallway() {
            if self.can_lock_pods(pod.ty) {
                let mut new_state = self.clone();
                // TODO: need to check that path isn't blocked.
                new_state.pods[pod_idx].position = Position::Side {
                    which: pod.ty,
                    lower: self.lock_would_be_lower(pod.ty),
                    locked: true,
                };
                output.push(new_state);
            }
        } else {
            // in side, not locked, need to move to hall
            let mut blocked_hallway_xs = smallvec::SmallVec::<[u8; 8]>::new();
            for hwp in ALL_HALLWAY_POS.iter() {
                //let pod.ty.hallway_x
            }
        }
    }

    fn can_lock_pods(&self, pod_type: PodType) -> bool {
        self.pods
            .iter()
            .filter_map(|p| match p.position {
                Position::Hallway(_) => None,
                Position::Side { which, locked, .. } => {
                    if which == pod_type {
                        Some(locked)
                    } else {
                        None
                    }
                }
            })
            .all(|l| l)
    }

    fn lock_would_be_lower(&self, pod_type: PodType) -> bool {
        self.pods
            .iter()
            .filter_map(|p| match p.position {
                Position::Hallway(_) => None,
                Position::Side { which, .. } => {
                    if which == pod_type {
                        Some(false)
                    } else {
                        None
                    }
                }
            })
            .next()
            .unwrap_or(true)
    }
}

fn part_a(input: &Input) -> i64 {
    2
}

fn part_b(input: &Input) -> i64 {
    2
}

aoc::aoc!(parser, part_a, part_b, Some(12521), Some(444356092776315));

#[cfg(test)]
mod tests {
    use super::*;
}
