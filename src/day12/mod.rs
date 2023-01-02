use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug)]
struct Coords(u32, u32);

impl From<(usize, usize)> for Coords {
    fn from(value: (usize, usize)) -> Self {
        Self(value.0 as u32, value.1 as u32)
    }
}

impl From<(u32, u32)> for Coords {
    fn from(value: (u32, u32)) -> Self {
        Self(value.0, value.1)
    }
}

impl TryFrom<(Option<u32>, Option<u32>)> for Coords {
    type Error = ();

    fn try_from(value: (Option<u32>, Option<u32>)) -> Result<Self, Self::Error> {
        Ok(Self(
            value.0.ok_or(())?,
            value.1.ok_or(())?,
        ))
    }
}

impl TryFrom<(Option<u32>, u32)> for Coords {
    type Error = ();

    fn try_from(value: (Option<u32>, u32)) -> Result<Self, Self::Error> {
        Ok(Self(
            value.0.ok_or(())?,
            value.1,
        ))
    }
}

impl TryFrom<(u32, Option<u32>)> for Coords {
    type Error = ();

    fn try_from(value: (u32, Option<u32>)) -> Result<Self, Self::Error> {
        Ok(Self(
            value.0,
            value.1.ok_or(())?,
        ))
    }
}

impl Coords {
    fn adjacent(&self) -> impl Iterator<Item=Self> {
        ([
            (self.0.checked_sub(1), self.1).try_into(),
            (self.0.checked_add(1), self.1).try_into(),
            (self.0, self.1.checked_add(1)).try_into(),
            (self.0, self.1.checked_sub(1)).try_into(),
        ]).into_iter().flatten()
    }
}

struct Matrix<T> {
    buf: Vec<T>,
    columns: u32,
}

impl<T, I: Into<Coords>> Index<I> for Matrix<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        let index = index.into();
        let index = index.0 + index.1 * self.columns;
        &self.buf[index as usize]
    }
}

impl<T, I: Into<Coords>> IndexMut<I> for Matrix<T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        let index = index.into();
        let index = index.0 + index.1 * self.columns;
        &mut self.buf[index as usize]
    }
}

impl<T> Matrix<T> {
    fn with_dimensions(columns: u32, rows: u32) -> Self {
        Self {
            buf: Vec::with_capacity((columns * rows) as usize),
            columns,
        }
    }

    fn rows(&self) -> u32 {
        self.buf.len() as u32 / self.columns
    }

    fn dimensions(&self) -> Coords {
        Coords(self.columns, self.rows())
    }

    fn in_bounds(&self, coords: Coords) -> bool {
        coords.0 < self.columns && coords.1 < self.rows()
    }
}

impl<T: Clone> Matrix<T> {
    fn fill<I: Into<Coords>>(value: T, dims: I) -> Self {
        let dims = dims.into();
        let columns = dims.0;
        let rows = dims.1;
        Self {
            buf: vec![value; (columns * rows) as usize],
            columns,
        }
    }
}

impl<T: Debug> Debug for Matrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut col = 0;

        for value in &self.buf {
            write!(f, " {:3?} ", value)?;
            if col >= self.columns - 1 {
                write!(f, "\n")?;
                col = 0;
            } else {
                write!(f, "|")?;
                col += 1;
            }
        }

        Ok(())
    }
}


struct HeightMap {
    inner: Matrix<u8>,
    start: Coords,
    end: Coords,
}

impl<I: Into<Coords>> Index<I> for HeightMap {
    type Output = u8;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        &self.inner[index]
    }
}

impl Debug for HeightMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl HeightMap {
    fn add_line(&mut self, line: &str) {
        assert_eq!(line.len() as u32, self.inner.columns);
        let row = self.inner.rows() as usize;
        self.inner.buf.extend(
            line.bytes().enumerate().map(|(col, c)| {
                let height = if c == b'S' {
                    eprintln!("S = {}, {}", col, row);
                    self.start = (col, row).into();
                    b'a'
                } else if c == b'E' {
                    eprintln!("E = {}, {}", col, row);
                    self.end = (col, row).into();
                    b'z'
                } else {
                    assert!(b'a' <= c && c <= b'z');
                    c
                };
                height - b'a'
            })
        );
    }

    fn from_str(input: &str) -> Self {
        let mut lines = input.trim_end().lines().map(str::trim_end);

        let first_line = lines.next().unwrap();
        let row = lines.size_hint().0 as u32;
        let col = first_line.len() as u32;

        let mut obj = Self {
            inner: Matrix::with_dimensions(col, row),
            start: Coords(0, 0),
            end: Coords(0, 0),
        };
        obj.add_line(first_line);

        for line in lines {
            obj.add_line(line)
        }

        obj
    }
}

fn dijkstra(map: &HeightMap) -> Matrix<u32> {
    let mut distances = Matrix::fill(u32::MAX, map.inner.dimensions());

    let mut stack = Vec::new();
    eprintln!("{:?}", map.end);
    distances[map.end] = 0;
    stack.push(map.end);

    while let Some(pos) = stack.pop() {
        let h = map[pos].saturating_sub(1);
        let d = distances[pos].saturating_add(1);

        for p in pos.adjacent() {
            if map.inner.in_bounds(p)
                && distances[p] > d
                && map[p] >= h
            {
                distances[p] = d;
                stack.push(p);
            }
        }
    }

    distances
}


#[aoc(day12, part1)]
fn part1(input: &str) -> u32 {
    let map = HeightMap::from_str(input);
    eprintln!("{:?}", map.inner);
    let d = dijkstra(&map);
    eprintln!("{:?}", d);

    d[map.start]
}

#[aoc(day12, part2)]
fn part2(input: &str) -> u32  {
    let map = HeightMap::from_str(input);
    eprintln!("{:?}", map.inner);
    let d = dijkstra(&map);
    eprintln!("{:?}", d);

    let (_, d) = map.inner.buf.iter().zip(d.buf.iter())
        .filter(|(h, _)| **h == 0)
        .min_by_key(|(_, d)| **d)
        .unwrap();

    *d
}