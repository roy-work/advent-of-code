use std::convert::TryFrom;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// A 2D map.
///
/// Row-major, i.e., `.0[y][x]`.
#[derive(Clone)]
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

    pub fn at<C: AsCoord>(&self, position: C) -> Option<&T> {
        let (x, y) = position.as_coord();
        self.0.get(y).and_then(|row| row.get(x))
    }

    pub fn at_mut<C: AsCoord>(&mut self, position: C) -> Option<&mut T> {
        let (x, y) = position.as_coord();
        self.0.get_mut(y).and_then(|row| row.get_mut(x))
    }

    pub fn rows(&self) -> impl Iterator<Item = (usize, impl Iterator<Item = (BoundCoord, &T)>)> {
        RowIter(self, self.0.iter().enumerate())
    }

    pub fn width(&self) -> usize {
        self.0.iter().next().map(|r| r.len()).unwrap_or(0)
    }

    pub fn height(&self) -> usize {
        self.0.len()
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

pub trait AsCoord {
    fn as_coord(&self) -> (usize, usize);
}

impl AsCoord for BoundCoord {
    fn as_coord(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

impl AsCoord for &BoundCoord {
    fn as_coord(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

impl AsCoord for (usize, usize) {
    fn as_coord(&self) -> (usize, usize) {
        (self.0, self.1)
    }
}

/// A coordinate that is not bound by space or time.
///
/// (It can be negative, off the map, etc.)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct FreeCoord {
    pub x: i64,
    pub y: i64,
}

impl FreeCoord {
    pub fn offset(self, x_off: i64, y_off: i64) -> FreeCoord {
        let x = self.x.checked_add(x_off).unwrap();
        let y = self.y.checked_add(y_off).unwrap();
        FreeCoord { x, y }
    }

    pub fn bind<T>(self, map: &Map<T>) -> Option<BoundCoord> {
        if self.x < 0 || self.y < 0 {
            return None;
        }
        let x = usize::try_from(self.x).unwrap();
        let y = usize::try_from(self.y).unwrap();
        let width = map.width();
        let height = map.height();
        if width <= x || height <= y {
            None
        } else {
            Some(BoundCoord {
                width,
                height,
                x,
                y,
            })
        }
    }
}

/// A coordinate that is constrained to within a particular `Map`'s dimensions.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct BoundCoord {
    pub width: usize,
    pub height: usize,
    pub x: usize,
    pub y: usize,
}

impl BoundCoord {
    /// Enumerate adjacent tiles in the cardinal directions, but only if they're within the bounds
    /// of the map bound to this coordinate.
    pub fn adj_cardinal(&self) -> impl Iterator<Item = BoundCoord> + '_ {
        BoundCoordCardinals(self, 0)
    }

    pub fn adj_diags(&self) -> impl Iterator<Item = BoundCoord> + '_ {
        BoundCoordDiags(self, 0)
    }

    pub fn unbind(&self) -> FreeCoord {
        FreeCoord {
            x: i64::try_from(self.x).unwrap(),
            y: i64::try_from(self.y).unwrap(),
        }
    }
}

const OFFSETS: &[(i8, i8)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

struct BoundCoordCardinals<'a>(&'a BoundCoord, u8);

impl Iterator for BoundCoordCardinals<'_> {
    type Item = BoundCoord;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // N.b.: loop exit when the iterator is done is here, in the `?`:
            let (off_x, off_y) = OFFSETS.get(usize::from(self.1))?;
            self.1 = self.1.checked_add(1).unwrap();

            let new_x = i64::try_from(self.0.x).unwrap() + i64::from(*off_x);
            let new_y = i64::try_from(self.0.y).unwrap() + i64::from(*off_y);
            if new_x < 0 || new_y < 0 {
                continue;
            }
            let new_x = usize::try_from(new_x).unwrap();
            let new_y = usize::try_from(new_y).unwrap();
            if self.0.width <= new_x || self.0.height <= new_y {
                continue;
            }
            return Some(BoundCoord {
                width: self.0.width,
                height: self.0.height,
                x: new_x,
                y: new_y,
            });
        }
    }
}

const DIAG_OFFSETS: &[(i8, i8)] = &[
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

struct BoundCoordDiags<'a>(&'a BoundCoord, u8);

impl Iterator for BoundCoordDiags<'_> {
    type Item = BoundCoord;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // N.b.: loop exit when the iterator is done is here, in the `?`:
            let (off_x, off_y) = DIAG_OFFSETS.get(usize::from(self.1))?;
            self.1 = self.1.checked_add(1).unwrap();

            let new_x = i64::try_from(self.0.x).unwrap() + i64::from(*off_x);
            let new_y = i64::try_from(self.0.y).unwrap() + i64::from(*off_y);
            if new_x < 0 || new_y < 0 {
                continue;
            }
            let new_x = usize::try_from(new_x).unwrap();
            let new_y = usize::try_from(new_y).unwrap();
            if self.0.width <= new_x || self.0.height <= new_y {
                continue;
            }
            return Some(BoundCoord {
                width: self.0.width,
                height: self.0.height,
                x: new_x,
                y: new_y,
            });
        }
    }
}

/*
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
*/

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

struct RowIter<'a, T>(
    &'a Map<T>,
    std::iter::Enumerate<std::slice::Iter<'a, Vec<T>>>,
);

impl<'a, T> Iterator for RowIter<'a, T> {
    type Item = (usize, CellIter<'a, T>);

    fn next(&mut self) -> Option<Self::Item> {
        self.1
            .next()
            .map(|(i, v)| (i, CellIter(self.0, i, v.iter().enumerate())))
    }
}

struct CellIter<'a, T>(
    &'a Map<T>,
    usize,
    std::iter::Enumerate<std::slice::Iter<'a, T>>,
);

impl<'a, T> Iterator for CellIter<'a, T> {
    type Item = (BoundCoord, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.2.next().map(|(i, v)| {
            (
                BoundCoord {
                    width: self.0.width(),
                    height: self.0.height(),
                    x: i,
                    y: self.1,
                },
                v,
            )
        })
    }
}
