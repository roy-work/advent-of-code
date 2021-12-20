use aoc::prelude::*;

type Input = Vec<Scanner>;

struct Scanner {
    num: i64,
    beacons: Vec<Beacon>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Beacon {
    x: i64,
    y: i64,
    z: i64,
}

aoc::hot_parse!(scan_line, "^--- scanner ([0-9]+) ---$", { 1 => i64, }, |t| t);
aoc::hot_parse!(coord_line, "^(-?[0-9]+),(-?[0-9]+),(-?[0-9]+)$", { 1 => i64, 2 => i64, 3 => i64, }, |t| t);

fn parser(path: &Path) -> anyhow::Result<Input> {
    let reader = BufReader::new(File::open(path)?);
    let mut lines = reader.lines();

    let mut scanners = Vec::new();
    'parser: loop {
        let scan_line_text = lines.next().unwrap()?;
        let (n,) = scan_line(scan_line_text.trim_end()).unwrap();
        let mut scanner = Scanner {
            num: n,
            beacons: Vec::new(),
        };
        loop {
            let line = match lines.next() {
                Some(l) => l?,
                None => {
                    scanners.push(scanner);
                    break 'parser;
                }
            };
            let trimmed_line = line.trim_end();
            if trimmed_line.is_empty() {
                break;
            }
            //println!("Line: {}", trimmed_line);
            let (x, y, z) = coord_line(trimmed_line).unwrap();
            scanner.beacons.push(Beacon { x, y, z });
        }
        scanners.push(scanner);
    }
    Ok(scanners)
}

// +y
// | +z
// |/
// *-- +x
//
// Beacon is facing +z (i.e., base position)
//(x, y, z)
// (rotate beacon CW

// (rot_around_x, rot_around_y)
const ALL_AXISES: &[(i64, i64)] = &[
    (0, 0),
    (0, 90),
    (0, 180),
    (0, 270),
    (90, 0),
    (270, 0),
];

struct AllRots(usize, i64);

impl AllRots {
    fn new() -> AllRots {
        AllRots(0, 0)
    }
}

impl Iterator for AllRots {
    type Item = Matrix;

    fn next(&mut self) -> Option<Self::Item> {
        if ALL_AXISES.len() <= self.0 {
            return None;
        }

        let (rot_x, rot_y) = ALL_AXISES[self.0];
        let rot_z = self.1 * 90;

        let matrix = mm(rz(rot_z), mm(rx(rot_x), ry(rot_y)));

        self.1 += 1;
        if 4 <= self.1 {
            self.1 = 0;
            self.0 += 1;
        }
        Some(matrix)
    }
}

struct PointAroundAllAxis(Beacon, AllRots);

impl PointAroundAllAxis {
    fn new(b: Beacon) -> PointAroundAllAxis {
        PointAroundAllAxis(b, AllRots::new())
    }
}

impl Iterator for PointAroundAllAxis {
    type Item = Beacon;

    fn next(&mut self) -> Option<Self::Item> {
        let rot_mat = self.1.next()?;

        let float_beacon = (self.0.x as f32, self.0.y as f32, self.0.z as f32);
        let rot_beacon = mm_point(&rot_mat, float_beacon);
        let rot_beacon = point_to_beacon(rot_beacon);
        Some(rot_beacon)
    }
}

#[derive(Clone, Debug)]
struct Matrix([f32; 9]);

impl Matrix {
    fn at(&self, r: usize, c: usize) -> f32 {
        self.0[r * 3 + c]
    }

    fn at_mut(&mut self, r: usize, c: usize) -> &mut f32 {
        &mut self.0[r * 3 + c]
    }
}

fn to_radians(v: f32) -> f32 {
    v / 180. * std::f32::consts::PI
}

fn rx(th: i64) -> Matrix {
    let th = to_radians(th as f32);
    Matrix([
        1., 0., 0.,
        0., th.cos(), -th.sin(),
        0., th.sin(), th.cos(),
    ])
}

fn ry(th: i64) -> Matrix {
    let th = to_radians(th as f32);
    Matrix([
        th.cos(), 0., th.sin(),
        0., 1., 0.,
        -th.sin(), 0., th.cos(),
    ])
}

fn rz(th: i64) -> Matrix {
    let th = to_radians(th as f32);
    Matrix([
        th.cos(), -th.sin(), 0.,
        th.sin(), th.cos(), 0.,
        0., 0., 1.,
    ])
}

fn mm(a: Matrix, b: Matrix) -> Matrix {
    let mut r = Matrix([0.; 9]);

    for r_row in 0..3 {
        for r_col in 0..3 {
            for shift in 0..3 {
                *r.at_mut(r_row, r_col) += a.at(r_row, shift) * b.at(shift, r_col);
            }
        }
    }

    r
}

fn mm_point(a: &Matrix, b: (f32, f32, f32)) -> (f32, f32, f32) {
    let in_point = [b.0, b.1, b.2];
    let mut out_point = [0.; 3];
    for idx in 0 .. 3 {
        for shift in 0 .. 3 {
            out_point[idx] += a.at(idx, shift) * in_point[shift];
        }
    }
    (out_point[0], out_point[1], out_point[2])
}

fn point_to_beacon(p: (f32, f32, f32)) -> Beacon {
    fn to_int(v: f32) -> i64 {
        let vr = v.round() as i64;
        assert!((v - (vr as f32)).abs() < 0.01);
        vr
    }
    Beacon {
        x: to_int(p.0),
        y: to_int(p.1),
        z: to_int(p.2),
    }
}

fn reorient_beacons(beacons: &[Beacon], new_orient: &Matrix) -> Vec::<Beacon> {
    beacons
        .iter()
        .map(|b| (b.x as f32, b.y as f32, b.z as f32))
        .map(|v| mm_point(new_orient, v))
        .map(point_to_beacon)
        .collect::<Vec<_>>()
}

fn reorient_beacon(b: &Beacon, new_orient: &Matrix) -> Beacon {
    let b = (b.x as f32, b.y as f32, b.z as f32);
    let v = mm_point(new_orient, b);
    point_to_beacon(v)
}

fn manhatten_dist(a: &Beacon, b: &Beacon) -> i64 {
    (a.x - b.x).abs()
    + (a.y - b.y).abs()
    + (a.z - b.z).abs()
}

type Shift = (i64, i64, i64);

fn do_scanners_overlap_in_orientation(beacons_a: &[Beacon], beacons_b: &[Beacon], orient: &Matrix) -> Option<Shift> {
    let beacons_b = reorient_beacons(beacons_b, orient);
    for choose_a in beacons_a.iter() {
        for choose_b in beacons_b.iter() {
            let shift = (
                choose_a.x - choose_b.x,
                choose_a.y - choose_b.y,
                choose_a.z - choose_b.z,
            );
            let beacons_a_hash: HashSet<_> = beacons_a.iter().cloned().collect();
            let beacons_b_hash: HashSet<_> = beacons_b.iter().map(|b| {
                Beacon {
                    x: b.x + shift.0,
                    y: b.y + shift.1,
                    z: b.z + shift.2,
                }
            }).collect();
            let matching = beacons_a_hash.intersection(&beacons_b_hash).collect::<Vec<_>>();
            if 12 <= matching.len() {
                return Some(shift);
            }
        }
    }
    None
}

fn find_all_overlaps(input: &Input) -> Vec<(usize, usize, Matrix, Shift)> {
    let mut overlaps: Vec<(usize, usize, Matrix, Shift)> = Vec::new();

    let all_orientations = AllRots::new().collect::<Vec<_>>();

    for (idx_a, scanner_a) in input.iter().enumerate() {
        {
            use std::io::Write;
            let mut stdout = std::io::stdout();
            write!(stdout, "\r\x1bK{}/{}", idx_a, input.len()).unwrap();
            stdout.flush().unwrap();
        }

        'b_loop: for (idx_b, scanner_b) in input.iter().enumerate() {
            if idx_a == idx_b {
                continue;
            }
            for orientation in all_orientations.iter() {
                let shift = do_scanners_overlap_in_orientation(
                    &scanner_a.beacons,
                    &scanner_b.beacons,
                    orientation,
                );
                if let Some(shift) = shift {
                    overlaps.push((idx_a, idx_b, orientation.to_owned(), shift));
                    continue 'b_loop;
                }
            }
        }
    }
    println!();

    overlaps
}

fn compute_overlap_solution_paths(overlaps: &[(usize, usize)], n: usize) -> Vec<Vec<usize>> {
    let mut paths = Vec::<Option<Vec<usize>>>::new();
    for _ in 0..n {
        paths.push(None);
    }
    paths[0] = Some(vec![]);
    loop {
        let mut changed = false;
        for (idx, (map_to, scan)) in overlaps.iter().copied().enumerate() {
            if paths[scan].is_some() {
                continue;
            }
            if let Some(v) = paths[map_to].as_ref() {
                let mut this_path = v.clone();
                this_path.push(idx);
                paths[scan] = Some(this_path);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    let mut solved_paths = Vec::new();
    for (pidx, p) in paths.into_iter().enumerate() {
        match p {
            Some(p) => solved_paths.push(p),
            None => {
                panic!("failed to find path to solution for scanner {}", pidx);
            }
        };
    }
    solved_paths
}

/*
fn overlap_path_graph(overlaps: &[(usize, usize)]) -> petgraph::UnGraph<usize, ()> {
    let mut g = petgraph::UnGraph::new();
    for n in overlaps.iter().map(|(a, _)| *a).chain(overlaps.iter().map(|(_, b)| *b)) {
        g.
    }
    for (e1, e2) in overlaps {
    }
}
*/

fn part_a(input: &Input) -> i64 {
    return 2;
    let all_overlaps = find_all_overlaps(&input);
    for overlap in all_overlaps.iter() {
        println!("overlap: {:?}", overlap);
    }
    let soln_edges = all_overlaps.iter().map(|(a, b, _, _)| (*a, *b)).collect::<Vec<_>>();
    let soln_paths = compute_overlap_solution_paths(&soln_edges, input.len());
    for soln_path in soln_paths.iter() {
        println!(" path: {:?}", soln_path);
    }

    let mut merged_beacons = HashSet::new();
    for (scanner_idx, scanner) in input.iter().enumerate() {
        let path = &soln_paths[scanner_idx];
        let mut adjusted_beacons = scanner.beacons.clone();
        for step in path.iter().rev().copied() {
            let (_, _, orientation, shift) = &all_overlaps[step];
            let mut new_beacons = reorient_beacons(&adjusted_beacons, orientation);
            for beacon in new_beacons.iter_mut() {
                *beacon = Beacon {
                    x: beacon.x + shift.0,
                    y: beacon.y + shift.1,
                    z: beacon.z + shift.2,
                };
            }
            adjusted_beacons = new_beacons;
        }
        for beacon in adjusted_beacons {
            merged_beacons.insert(beacon);
        }
    }
    i64::try_from(merged_beacons.len()).unwrap()
}

fn part_b(input: &Input) -> i64 {
    let all_overlaps = find_all_overlaps(&input);
    for overlap in all_overlaps.iter() {
        println!("overlap: {:?}", overlap);
    }
    let soln_edges = all_overlaps.iter().map(|(a, b, _, _)| (*a, *b)).collect::<Vec<_>>();
    let soln_paths = compute_overlap_solution_paths(&soln_edges, input.len());
    for soln_path in soln_paths.iter() {
        println!(" path: {:?}", soln_path);
    }

    let mut scanner_positions = Vec::new();
    for (scanner_idx, scanner) in input.iter().enumerate() {
        let path = &soln_paths[scanner_idx];
        let mut adjusted_scanner = Beacon {
            x: 0,
            y: 0,
            z: 0,
        };
        for step in path.iter().rev().copied() {
            let (_, _, orientation, shift) = &all_overlaps[step];
            let new_scanner_pos = reorient_beacon(&adjusted_scanner, orientation);
            let new_scanner_pos = Beacon {
                x: new_scanner_pos.x + shift.0,
                y: new_scanner_pos.y + shift.1,
                z: new_scanner_pos.z + shift.2,
            };
            adjusted_scanner = new_scanner_pos;
        }
        scanner_positions.push(adjusted_scanner);
    }
    for (idx, scanner) in scanner_positions.iter().enumerate() {
        println!("Scanner {}: {:?}", idx, scanner);
    }
    let mut max_dist = 0;
    for scanner_a in scanner_positions.iter() {
        for scanner_b in scanner_positions.iter() {
            let dist = manhatten_dist(scanner_a, scanner_b);
            max_dist = std::cmp::max(max_dist, dist);
        }
    }
    max_dist
}

aoc::aoc!(parser, part_a, part_b, Some(79), Some(3621));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation() {
        let b = Beacon {
            x: 1,
            y: 2,
            z: 3,
        };
        for nb in PointAroundAllAxis::new(b) {
            println!("{:?}", nb);
        }
    }

    #[test]
    fn test_example_scan_0_and_1() {
        let input = parser(Path::new("test-a")).unwrap();
        let scanner_0 = &input[0];
        let scanner_1 = &input[1];
        for orient in AllRots::new() {
            let did_overlap = do_scanners_overlap_in_orientation(&scanner_0.beacons, &scanner_1.beacons, &orient);
            println!("Overlap? {:?}", did_overlap);
        }
    }

    #[test]
    fn test_example() {
        let input = parser(Path::new("test-a")).unwrap();
        let all_overlaps = find_all_overlaps(&input);
        for overlap in all_overlaps.iter() {
            println!("overlap: {:?}", overlap);
        }
        let soln_edges = all_overlaps.iter().map(|(a, b, _, _)| (*a, *b)).collect::<Vec<_>>();
        let soln_paths = compute_overlap_solution_paths(&soln_edges, input.len());
        for soln_path in soln_paths.iter() {
            println!(" path: {:?}", soln_path);
        }

        let mut merged_beacons = HashSet::new();
        for (scanner_idx, scanner) in input.iter().enumerate() {
            let path = &soln_paths[scanner_idx];
            let mut adjusted_beacons = scanner.beacons.clone();
            for step in path.iter().rev().copied() {
                let (_, _, orientation, shift) = &all_overlaps[step];
                let mut new_beacons = reorient_beacons(&adjusted_beacons, orientation);
                for beacon in new_beacons.iter_mut() {
                    *beacon = Beacon {
                        x: beacon.x + shift.0,
                        y: beacon.y + shift.1,
                        z: beacon.z + shift.2,
                    };
                }
                adjusted_beacons = new_beacons;
            }
            for beacon in adjusted_beacons {
                merged_beacons.insert(beacon);
            }
        }
        println!("{} merged beacons.", merged_beacons.len());
    }

    #[test]
    fn test_part_b() {
        let input = parser(Path::new("test-a")).unwrap();
        let all_overlaps = find_all_overlaps(&input);
        for overlap in all_overlaps.iter() {
            println!("overlap: {:?}", overlap);
        }
        let soln_edges = all_overlaps.iter().map(|(a, b, _, _)| (*a, *b)).collect::<Vec<_>>();
        let soln_paths = compute_overlap_solution_paths(&soln_edges, input.len());
        for soln_path in soln_paths.iter() {
            println!(" path: {:?}", soln_path);
        }

        let mut scanner_positions = Vec::new();
        for (scanner_idx, scanner) in input.iter().enumerate() {
            let path = &soln_paths[scanner_idx];
            let mut adjusted_scanner = Beacon {
                x: 0,
                y: 0,
                z: 0,
            };
            for step in path.iter().rev().copied() {
                let (_, _, orientation, shift) = &all_overlaps[step];
                let new_scanner_pos = reorient_beacon(&adjusted_scanner, orientation);
                let new_scanner_pos = Beacon {
                    x: new_scanner_pos.x + shift.0,
                    y: new_scanner_pos.y + shift.1,
                    z: new_scanner_pos.z + shift.2,
                };
                adjusted_scanner = new_scanner_pos;
            }
            scanner_positions.push(adjusted_scanner);
        }
        for (idx, scanner) in scanner_positions.iter().enumerate() {
            println!("Scanner {}: {:?}", idx, scanner);
        }
        let mut max_dist = 0;
        for scanner_a in scanner_positions.iter() {
            for scanner_b in scanner_positions.iter() {
                let dist = manhatten_dist(scanner_a, scanner_b);
                max_dist = std::cmp::max(max_dist, dist);
            }
        }
        println!("Max dist = {}", max_dist);
    }
}
