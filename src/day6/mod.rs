use std::array;
use std::fmt::{Display, Formatter};
use std::ops::BitOr;


// *************************************************************************************************


struct Window<const N: usize> (
    [u8; N]
);


struct WindowIter<'a, const N: usize> (
    &'a str
);


// *************************************************************************************************


impl<'a, const N: usize> Iterator for WindowIter<'a, N> {
    type Item = Window<N>;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.0.as_bytes().get(0..N)?;
        self.0 = &self.0[1..];
        Some(Window(
            array::from_fn(|i| res[i])
        ))
    }
}


// *************************************************************************************************


impl<const N: usize> Window<N> {
    fn mask(x: &u8) -> u32 {
        1 << (x - b'a')
    }

    fn is_unique(&self) -> bool {
        let ones =
            self.0
                .iter()
                .map(Self::mask)
                .fold(0, u32::bitor)
                .count_ones();
        ones as usize == N
    }
}


// *************************************************************************************************

impl<const N: usize> Display for Window<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in self.0 {
            write!(f, "{}", c as char)?;
        }
        Ok(())
    }
}

// *************************************************************************************************

fn first_marker<const N: usize>(input: &str) -> usize {
    WindowIter::<N>(input)
        .enumerate()
        .inspect(|(_, w)| eprintln!("{w}"))
        .find(|(_, w)| w.is_unique())
        .unwrap()
        .0 + N
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    first_marker::<4>(input)
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    first_marker::<14>(input)
}


pub fn main() {
    include_str!("test.txt").lines()
        .for_each(|line|
            println!("{line}:\t{}", part2(line.trim()))
        )
}