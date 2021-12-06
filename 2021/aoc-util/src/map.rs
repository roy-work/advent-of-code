use std::convert::TryFrom;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// A 2D map.
///
/// Row-major, i.e., .0[y][x].
pub struct Map<T>(pub Vec<Vec<T>>);

impl<T> Map<T> {
    pub fn iter_tiles(&self) -> impl Iterator<Item = &T> {
        let mut row_iter = self.0.iter();
        let col_iter = row_iter.next().unwrap().iter();
        TileIter { row_iter, col_iter }
    }

    pub fn parse_filename_char_is_tile<F: Fn(char) -> T>(
        path: &Path,
        tile_parse: F,
    ) -> io::Result<Map<T>> {
        let file = BufReader::new(File::open(path)?);
        let mut map = Vec::new();
        for line in file.lines() {
            let line = line?;
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(tile_parse(c));
            }
            map.push(row);
        }
        Ok(Map(map))
    }

    pub fn at(&self, x: usize, y: usize) -> Option<&T> {
        self.0.get(y).and_then(|row| row.get(x))
    }

    pub fn at_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.0.get_mut(y).and_then(|row| row.get_mut(x))
    }

    pub fn render_single_char<F: Fn(&T) -> char>(&self, df: F) {
        for row in self.0.iter() {
            for cell in row.iter() {
                print!("{}", df(cell));
            }
            println!()
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn offset(self, x_off: i64, y_off: i64) -> Result<Coord, (i64, i64)> {
        let new_x = i64::try_from(self.x).unwrap() + i64::from(x_off);
        let new_y = i64::try_from(self.y).unwrap() + i64::from(y_off);

        if new_x < 0 || new_y < 0 {
            Err((new_x, new_y))
        } else {
            Ok(Coord {
                x: usize::try_from(new_x).unwrap(),
                y: usize::try_from(new_y).unwrap(),
            })
        }
    }
}

struct TileIter<'a, T> {
    row_iter: std::slice::Iter<'a, Vec<T>>,
    col_iter: std::slice::Iter<'a, T>,
}

impl<'a, T> Iterator for TileIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(i) = self.col_iter.next() {
            return Some(i);
        }
        self.col_iter = match self.row_iter.next() {
            Some(row) => row.iter(),
            None => return None,
        };
        self.col_iter.next()
    }
}
