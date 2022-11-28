use std::cmp::{min, max};
use std::sync::Arc;
use aoc::prelude::*;

type Input = Vec<(Flip, CubeSpec)>;

#[derive(Clone, Copy, Debug)]
enum Flip {
    On,
    Off,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CubeSpec {
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

impl CubeSpec {
    fn valid(&self) {
        assert!(self.x.0 <= self.x.1);
        assert!(self.y.0 <= self.y.1);
        assert!(self.z.0 <= self.z.1);
    }

    fn split(&self) -> [Arc<CubeSpec>; 8] {
        let mid_x = (self.x.0 + self.x.1) / 2;
        let mid_y = (self.y.0 + self.y.1) / 2;
        let mid_z = (self.z.0 + self.z.1) / 2;
        let xs = [(self.x.0, mid_x - 1), (mid_x, self.x.1)];
        let ys = [(self.y.0, mid_y - 1), (mid_y, self.y.1)];
        let zs = [(self.z.0, mid_z - 1), (mid_z, self.z.1)];
        [
            Arc::new(CubeSpec {
                x: xs[0],
                y: ys[0],
                z: zs[0],
            }),
            Arc::new(CubeSpec {
                x: xs[0],
                y: ys[0],
                z: zs[1],
            }),
            Arc::new(CubeSpec {
                x: xs[0],
                y: ys[1],
                z: zs[0],
            }),
            Arc::new(CubeSpec {
                x: xs[0],
                y: ys[1],
                z: zs[1],
            }),
            Arc::new(CubeSpec {
                x: xs[1],
                y: ys[0],
                z: zs[0],
            }),
            Arc::new(CubeSpec {
                x: xs[1],
                y: ys[0],
                z: zs[1],
            }),
            Arc::new(CubeSpec {
                x: xs[1],
                y: ys[1],
                z: zs[0],
            }),
            Arc::new(CubeSpec {
                x: xs[1],
                y: ys[1],
                z: zs[1],
            }),
        ]
    }
}

aoc::hot_parse!(cube_line, "^(on|off) x=(-?[0-9]+)..(-?[0-9]+),y=(-?[0-9]+)..(-?[0-9]+),z=(-?[0-9]+)..(-?[0-9]+)$", { 1 => String, 2 => i64, 3 => i64, 4 => i64, 5 => i64, 6 => i64, 7 => i64, }, |t| t);

fn parser(path: &Path) -> anyhow::Result<Input> {
    let reader = BufReader::new(File::open(path)?);

    let mut ops = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let (fs, x1, x2, y1, y2, z1, z2) = cube_line(line.trim_end()).unwrap();
        let cubespec = CubeSpec {
            x: (x1, x2),
            y: (y1, y2),
            z: (z1, z2),
        };
        let flip = match fs.as_str() {
            "on" => Flip::On,
            "off" => Flip::Off,
            _ => panic!(),
        };
        ops.push((flip, cubespec));
    }
    Ok(ops)
}

fn part_a(input: &Input) -> i64 {
    // def not gonna work on part 2 lol
    let mut on = HashSet::new();
    for (flip, cube) in input {
        let cube = {
            if cube.x.0 > 50 || cube.x.1 < -50
                || cube.y.0 > 50 || cube.y.1 < -50
                || cube.z.0 > 50 || cube.z.1 < -50 {
                continue;
            }
            CubeSpec {
                x: (max(-50, cube.x.0), min(50, cube.x.1)),
                y: (max(-50, cube.y.0), min(50, cube.y.1)),
                z: (max(-50, cube.z.0), min(50, cube.z.1)),
            }
        };
        for x in cube.x.0..=cube.x.1 {
            for y in cube.y.0..=cube.y.1 {
                for z in cube.z.0..=cube.z.1 {
                    match flip {
                        Flip::On => { on.insert((x, y, z)); }
                        Flip::Off => { on.remove(&(x, y, z)); }
                    }
                }

            }
        }
    }
    i64::try_from(on.len()).unwrap()
}

struct TreeCube {
    state: Option<Flip>,
    cube: CubeSpec,
}

enum TCubeState {
    On,
    Off,
    Mixed([Arc<TreeCube>; 8]),
}

fn part_b(input: &Input) -> i64 {
    let extent = {
        use building_blocks_core::point::{Point3i, PointN};
        building_blocks_core::extent::Extent3i {
            minimum: PointN([-262144, -262144, -262144]),
            shape: PointN([524288, 524288, 524288]),
        }
    };
    building_blocks_storage::octree::set::OctreeSet::new_empty(extent);
    2
}

aoc::aoc!(parser, part_a, part_b, Some(590784), Some(444356092776315));

#[cfg(test)]
mod tests {
    use super::*;
}
