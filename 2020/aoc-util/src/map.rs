use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

trait Tile {
    fn parse(c: char) -> Self;
}

/// A map.
///
/// Row-major.
struct Map<T>(pub Vec<Vec<T>>);

impl<T> Map<T> {
    pub fn iter_tiles(&self) -> impl Iterator<Item = &T> {
        let mut row_iter = self.0.iter();
        let mut col_iter = row_iter.next().unwrap().iter();
        TileIter {
            row_iter,
            col_iter,
        }
    }
}

impl<T: Tile> Map<T> {
    pub fn parse_filename(path: &Path) -> io::Result<Map<T>> {
        let file = BufReader::new(File::open(path)?);
        let mut map = Vec::new();
        for line in file.lines() {
            let line = line?;
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(T::parse(c));
            }
            map.push(row);
        }
        Ok(Map(map))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn offset(self, x_off: i64, y_off: i64) {

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
