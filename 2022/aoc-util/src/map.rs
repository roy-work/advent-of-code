use std::convert::TryFrom;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// A 2D map.
///
/// Internal storage is row major.
#[derive(Clone)]
pub struct Map<T> {
    tiles: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Copy> Map<T> {
    pub fn new(width: usize, height: usize, init: T) -> Map<T> {
        let tiles = width.checked_mul(height).expect("width * height exceeded usize::MAX");
        let tiles = [init].repeat(tiles);
        Map {
            tiles,
            width,
            height,
        }
    }
}

impl<T> Map<T> {
    pub fn parse_filename_char_is_tile<F: Fn(char) -> T>(
        path: &Path,
        tile_parse: F,
    ) -> io::Result<Map<T>> {
        let file = BufReader::new(File::open(path)?);
        let mut tiles = Vec::<T>::new();
        let mut width = None;
        let mut height = 0usize;

        for (line_idx, line) in file.lines().enumerate() {
            let line = line?;
            let row = line.chars().map(&tile_parse).collect::<Vec<_>>();
            let this_width = row.len();
            match width {
                Some(w) if w == this_width => (),
                Some(w) => {
                    panic!("Line {}'s width was {}, but we expected {}.", line_idx.checked_add(1).unwrap(), this_width, w);
                },
                None => width = Some(this_width),
            }
            tiles.extend(row);
            height = height.checked_add(1).unwrap();
        }
        let width = match width {
            Some(w) => w,
            None => {
                assert!(tiles.len() == 0);
                0
            }
        };
        Ok(Map {
            tiles,
            width,
            height,
        })
    }

    /// Iterate over the tiles of the map.
    pub fn tiles(&self) -> impl Iterator<Item = (BoundCoord, &T)> {
        struct TileIter<'a, T> {
            x: usize,
            y: usize,
            map: &'a Map<T>,
        }

        impl<'a, T> Iterator for TileIter<'a, T> {
            type Item = (BoundCoord, &'a T);

            fn next(&mut self) -> Option<Self::Item> {
                if self.map.height <= self.y {
                    return None;
                }

                let coord = BoundCoord {
                    width: self.map.width,
                    height: self.map.height,
                    x: self.x,
                    y: self.y,
                };
                let tile = self.map.at(&coord).unwrap();

                self.x = self.x.checked_add(1).unwrap();
                if self.map.width <= self.x {
                    self.x = 0;
                    self.y = self.y.checked_add(1).unwrap();
                }
                Some((coord, tile))
            }
        }

        TileIter {
            x: 0,
            y: 0,
            map: &self,
        }
    }

    pub fn at<C: AsCoord>(&self, position: C) -> Option<&T> {
        let (x, y) = position.as_coord();
        let idx = x.checked_mul(y).expect("tile coordinate exceeded usize::MAX");
        self.tiles.get(idx)
    }

    pub fn at_mut<C: AsCoord>(&mut self, position: C) -> Option<&mut T> {
        let (x, y) = position.as_coord();
        let idx = x.checked_mul(y).expect("tile coordinate exceeded usize::MAX");
        self.tiles.get_mut(idx)
    }

    /// Iterate over rows of tiles on the map.
    ///
    /// This method exists to make it easy to write a double for loop over the tiles.
    pub fn rows(&self) -> impl Iterator<Item = Row<T>> {
        (0..self.height).map(move |y| {
            Row {
                y,
                map: &self,
            }
        })
    }

    /// Get the width of the map.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get the height of the map.
    pub fn height(&self) -> usize {
        self.height
    }

    /*
    pub fn render_single_char<F: Fn(&T) -> char>(&self, df: F) {
        for row in self.0.iter() {
            for cell in row.iter() {
                print!("{}", df(cell));
            }
            println!()
        }
    }
    */
}

trait DisplayTile {
    fn to_term(&self) -> (&str, Option<crate::term::TermAttr>);
}

/// A single row in a map, when iterating over rows with [`Map::rows`].
pub struct Row<'a, T> {
    y: usize,
    map: &'a Map<T>,
}

impl<T> Row<'_, T> {
    pub fn y(&self) -> usize {
        self.y
    }

    pub fn tiles(&self) -> impl Iterator<Item = (BoundCoord, &T)> {
        (0..self.map.width).map(move |x| {
            let coord = BoundCoord {
                width: self.map.width,
                height: self.map.height,
                x,
                y: self.y,
            };
            let tile = self.map.at(&coord).unwrap();
            (coord, tile)
        })
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
