use aoc::map::{FreeCoord, Map};
use aoc::prelude::*;

struct Input {
    algorithm: String,
    image: Map<Pixel>,
}

#[derive(Debug, Copy, Clone)]
enum Pixel {
    // light
    Pound,
    // dark
    Dot,
}

impl TryFrom<char> for Pixel {
    type Error = ();

    fn try_from(v: char) -> Result<Pixel, ()> {
        match v {
            '#' => Ok(Pixel::Pound),
            '.' => Ok(Pixel::Dot),
            _ => Err(()),
        }
    }
}

impl Pixel {
    fn to_bit(&self) -> u8 {
        match self {
            Pixel::Pound => 1,
            Pixel::Dot => 0,
        }
    }
}

fn parser(path: &Path) -> anyhow::Result<Input> {
    let reader = BufReader::new(File::open(path)?);
    let mut lines = reader.lines();

    let algorithm = lines.next().unwrap()?.trim_end().to_owned();
    let blank = lines.next().unwrap()?;
    assert!(blank.trim_end() == "");

    let mut image = Vec::new();
    for line in lines {
        let line = line?;
        let row = line
            .trim_end()
            .chars()
            .map(|c| Pixel::try_from(c).unwrap())
            .collect::<Vec<_>>();
        image.push(row);
    }

    Ok(Input {
        algorithm,
        image: Map(image),
    })
}

fn pixel_batch_to_index(batch: &[Pixel]) -> usize {
    let mut result = 0;
    for pixel in batch {
        result = (result << 1) | usize::from(pixel.to_bit());
    }
    result
}

fn new_image(width: usize, height: usize) -> Map<Pixel> {
    let mut rows = Vec::new();
    for _ in 0..height {
        let mut row = Vec::new();
        for _ in 0..width {
            row.push(Pixel::Dot);
        }
        rows.push(row);
    }
    Map(rows)
}

fn border(image: &Map<Pixel>) -> Map<Pixel> {
    let mut new_rows = Vec::new();
    let width = image.width();
    let mut first_row = Vec::new();
    for _ in 0 .. width+2 {
        first_row.push(Pixel::Dot);
    }
    new_rows.push(first_row.clone());
    for row in image.0.iter() {
        let new_row = std::iter::once(Pixel::Dot)
            .chain(row.iter().copied())
            .chain(std::iter::once(Pixel::Dot))
            .collect::<Vec<_>>();
        new_rows.push(new_row);
    }
    new_rows.push(first_row);
    Map(new_rows)
}

const OFFSETS: &[(i64, i64)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn get_pixel_batch(image: &Map<Pixel>, fill: Pixel, center: &FreeCoord) -> [Pixel; 9] {
    let mut output = [fill; 9];
    for idx in 0..9 {
        let offset = OFFSETS[idx];
        let coord = center.offset(offset.0, offset.1);
        let bound_coord = match coord.bind(image) {
            Some(c) => c,
            None => continue,
        };
        output[idx] = image.at(bound_coord).map(|p| *p).unwrap_or(fill);
    }
    output
}

fn step_image(image: &Map<Pixel>, fill: Pixel, algorithm: &[Pixel]) -> (Map<Pixel>, Pixel) {
    let mut out_image = new_image(image.width() + 2, image.height() + 2);

    for y in 0 .. out_image.height() {
        for x in 0 .. out_image.width() {
            let pixel_coord = FreeCoord {
                x: i64::try_from(x).unwrap(),
                y: i64::try_from(y).unwrap(),
            };
            let orig_coord = pixel_coord.offset(-1, -1);
            let batch = get_pixel_batch(image, fill, &orig_coord);
            let index = pixel_batch_to_index(&batch);
            let pixel = algorithm[index];
            *out_image.at_mut(pixel_coord.bind(&out_image).unwrap()).unwrap() = pixel;
        }
    }

    let new_fill = {
        let infini_batch = std::iter::repeat(fill).take(9).collect::<Vec<_>>();
        let index = pixel_batch_to_index(&infini_batch);
        algorithm[index]
    };

    (out_image, new_fill)
}

fn run_sim(input: &Input, steps: usize) -> Map<Pixel> {
    let algorithm = input.algorithm
        .chars()
        .map(|p| Pixel::try_from(p).unwrap())
        .collect::<Vec<_>>();

    let mut image = input.image.clone();
    let mut fill = Pixel::Dot;
    for _ in 0 .. steps {
        let (new_image, new_fill) = step_image(&image, fill, &algorithm);
        image = new_image;
        fill = new_fill;
    }
    image
}

fn part_a(input: &Input) -> i64 {
    let image = run_sim(&input, 2);
    let mut lit = 0;
    for row in image.0 {
        lit += row.iter().filter(|p| matches!(p, Pixel::Pound)).count();
    }
    i64::try_from(lit).unwrap()
}

fn part_b(input: &Input) -> i64 {
    let image = run_sim(input, 50);
    let mut lit = 0;
    for row in image.0 {
        lit += row.iter().filter(|p| matches!(p, Pixel::Pound)).count();
    }
    i64::try_from(lit).unwrap()
}

aoc::aoc!(parser, part_a, part_b, Some(35), Some(3351));

#[cfg(test)]
mod tests {
    use super::*;
}
