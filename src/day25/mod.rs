use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use anyhow::bail;

#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[repr(u8)]
pub enum SnafuDigit {
    DoubleMinus = b'=',
    Minus = b'-',
    Zero = b'0',
    One = b'1',
    Two = b'2',
}

#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct SnafuNum(
    Vec<SnafuDigit>
);


// *************************************************************************************************


impl SnafuDigit {
    const BASE: i64 = 5;

    fn value(self) -> i64 {
        match self {
            SnafuDigit::DoubleMinus => -2,
            SnafuDigit::Minus => -1,
            _ => {
                self as i64 - (b'0' as i64)
            }
        }
    }

    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_char(self) -> char {
        self.as_u8() as char
    }
}

impl Display for SnafuDigit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.as_char(), f)
    }
}

impl Debug for SnafuDigit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Display for SnafuNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: Vec<_> =
            self.0.iter()
                .map(|s| s.as_u8())
                .collect();
        let s = unsafe {
            // SAFETY: SnafuDigit::as_u8 generates only valid ASCII bytes
            String::from_utf8_unchecked(s)
        };

        Display::fmt(&s, f)
    }
}


// *************************************************************************************************

impl SnafuDigit {
    fn from_value(value: i8) -> anyhow::Result<Self> {
        Ok(match value {
            -2 => Self::DoubleMinus,
            -1 => Self::Minus,
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Two,
            _ => bail!("Unexpected value: {value}")
        })
    }
}


impl TryFrom<u8> for SnafuDigit {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let res = match value {
            b'=' => Self::DoubleMinus,
            b'-' => Self::Minus,
            b'0' => Self::Zero,
            b'1' => Self::One,
            b'2' => Self::Two,
            _ => bail!("Unexpected byte: {value} ('{}')", value as char)
        };
        debug_assert_eq!(res as u8, value);
        Ok(res)
    }
}

impl TryFrom<char> for SnafuDigit {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        <Self as TryFrom<u8>>::try_from(value.try_into()?)
    }
}


impl FromStr for SnafuNum {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits: Result<_, _> =
            s.bytes()
                .map(|b| b.try_into())
                .collect();
        Ok(Self(digits?))
    }
}


// *************************************************************************************************


impl SnafuNum {
    fn value(&self) -> i64 {
        self.0.iter()
            .rev()
            .enumerate()
            .map(|(ind, digit)| {
                let exp = SnafuDigit::BASE.pow(ind as u32);
                digit.value() * exp
            })
            .sum()
    }

    fn from_value(value: i64) -> Self {
        let mut value = value;
        let mut digits = Vec::with_capacity(16);


        while value > 0 {
            value += 2;
            let d = (value % SnafuDigit::BASE) - 2;
            value /= SnafuDigit::BASE;

            digits.push(SnafuDigit::from_value(d as i8).unwrap())
        }
        digits.reverse();
        Self(digits)
    }
}

impl From<SnafuNum> for i64 {
    fn from(snafu: SnafuNum) -> Self {
        snafu.value()
    }
}


// *************************************************************************************************

#[aoc(day25, part1)]
pub fn part1(input: &str) -> SnafuNum {
    let sum =
        input.lines()
            .map(|line| {
                let line = line.trim_end();
                let snafu: SnafuNum = line.parse()
                    .expect("Failed to parse line into Snafu");
                let value = snafu.value();
                println!("{snafu:>20}  {value:>16}");
                value
            })
            .sum();
    SnafuNum::from_value(sum)
}


// *************************************************************************************************

pub fn main() {
    part1(include_str!("test.txt"));
}


#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &[(i64, &str)] = &[
        (1, "1"),
        (2, "2"),
        (3, "1="),
        (4, "1-"),
        (5, "10"),
        (6, "11"),
        (7, "12"),
        (8, "2="),
        (9, "2-"),
        (10, "20"),
        (15, "1=0"),
        (20, "1-0"),
        (2022, "1=11-2"),
        (12345, "1-0---0"),
        (314159265, "1121-1110-1=0"),
        (1747, "1=-0-2"),
        (906, "12111"),
        (198, "2=0="),
        (11, "21"),
        (201, "2=01"),
        (31, "111"),
        (1257, "20012"),
        (32, "112"),
        (353, "1=-1="),
        (107, "1-12"),
        (7, "12"),
        (37, "122"),
    ];

    fn parsed_data() -> impl Iterator<Item=(i64, SnafuNum)> {
        DATA
            .iter()
            .map(|(i, snafu)| (*i, SnafuNum::from_str(snafu).unwrap()))
    }

    #[test]
    fn test_snafu_to_value() {
        for (val, snafu) in parsed_data() {
            assert_eq!(snafu.value(), val)
        }
    }

    #[test]
    fn test_snafu_from_value() {
        for (val, snafu) in parsed_data() {
            assert_eq!(SnafuNum::from_value(val), snafu)
        }
    }
}