use std::fmt::{Debug, Display, Formatter};
use std::mem;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
    crat_some = { "[" ~ ASCII_ALPHA ~ "]" }
    crat_none = { "   " }
    crat = { crat_some | crat_none }
    crat_line = { crat ~ (" " ~ crat)* ~ NEWLINE? }

    number = @{ ASCII_DIGIT+ }
    move_line = {
        "move " ~ number ~ " from " ~ number ~ " to " ~ number ~ NEWLINE?
    }

    crates = {crat_line+}
    moves = {move_line+}

    file = {
        crates
        ~ (" " | ASCII_DIGIT)+ ~ NEWLINE
        ~ NEWLINE
        ~ moves
    }
"#]
struct MyParser;


// *************************************************************************************************


#[derive(Debug, Copy, Clone)]
struct Crate(u8);

struct Move {
    num: u8,
    from: u8,
    to: u8,
}

struct Cargo {
    /// Each stack is a *column*
    stacks: Vec<Vec<Crate>>,
}


// *************************************************************************************************

impl Crate {
    fn as_char(self) -> char {
        self.0 as char
    }
}

impl Display for Crate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.as_char())
    }
}

impl Display for Cargo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let highest = self.stacks.iter().map(|c| c.len()).max().unwrap_or(0);

        for i in (0..highest).rev() {
            for c in &self.stacks {
                match c.get(i) {
                    Some(crat) => {
                        write!(f, "{crat} ")?
                    }
                    None => {
                        write!(f, "    ")?
                    }
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "move {} from {} to {}", self.num, self.from, self.to)
    }
}

// *************************************************************************************************

impl Crate {
    fn from_ascii(ascii: u8) -> Self {
        Self(ascii)
    }

    /// Call it with the parse result of `crat` rule
    fn parse(crat: Pair<Rule>) -> Option<Self> {
        debug_assert_eq!(crat.as_rule(), Rule::crat);
        let crat = crat.into_inner().next().unwrap();
        match crat.as_rule() {
            Rule::crat_some => {
                Some(Self::from_ascii(
                    crat.as_str().as_bytes()[1]
                ))
            }
            _ => {
                None
            }
        }
    }
}

impl Cargo {
    fn push(&mut self, column: usize, crat: Crate) {
        while self.stacks.len() <= column {
            self.stacks.push(Vec::new())
        }
        self.stacks[column].push(crat)
    }

    fn parse(crates: Pair<Rule>) -> Self {
        debug_assert!(crates.as_rule() == Rule::crates);

        let mut cargo = Self {
            stacks: Vec::with_capacity(16)
        };

        for crate_line in crates.into_inner().rev() {
            crate_line.into_inner()
                .enumerate()
                .flat_map(|(i, crat)| -> Option<_> {
                    Some((i, Crate::parse(crat)?))
                })
                .for_each(|(i, crat)| {
                    cargo.push(i, crat)
                });
        }

        cargo
    }
}

impl Move {
    fn parse(move_line: Pair<Rule>) -> Self {
        debug_assert_eq!(move_line.as_rule(), Rule::move_line);
        let mut inner = move_line.into_inner();
        let mut next_num = || -> u8 {
            inner.next().unwrap().as_str().parse().unwrap()
        };

        Self {
            num: next_num(),
            from: next_num(),
            to: next_num(),
        }
    }

    fn parse_moves(moves: Pair<Rule>) -> impl Iterator<Item=Self> + '_ {
        debug_assert_eq!(moves.as_rule(), Rule::moves);
        moves.into_inner().map(Move::parse)
    }
}


// *************************************************************************************************

impl Cargo {
    fn do_move(&mut self, mov: Move) {
        let num = mov.num as usize;
        let from = mov.from as usize - 1;
        let to = mov.to as usize - 1;

        self.stacks[to].reserve(num);
        for _ in 0..num {
            if let Some(c) = self.stacks[from].pop() {
                self.stacks[to].push(c)
            }
        }
    }

    fn do_move_preserve_order(&mut self, mov: Move) {
        let num = mov.num as usize;
        let from = mov.from as usize - 1;
        let to = mov.to as usize - 1;

        let f = &mut self.stacks[from];
        let len = f.len();
        let crates = if let Some(at) = len.checked_sub(num) {
            f.split_off(at)
        } else {
            mem::take(f)
        };
        self.stacks[to].extend_from_slice(&crates);
    }

    fn top_crate(self) -> String {
        self.stacks
            .iter()
            .flat_map(|c| -> Option<_> {
                Some(c.last()?.as_char())
            })
            .collect()
    }
}

// *************************************************************************************************

fn parse(input: &str) -> (Cargo, impl Iterator<Item=Move> + '_) {
    let mut parse = MyParser::parse(Rule::file, input)
        .expect("Parse Error")
        .next().unwrap()
        .into_inner(); // get and unwrap the `file` rule; never fails;

    (
        Cargo::parse(parse.next().unwrap()),
        Move::parse_moves(parse.next().unwrap())
    )
}

#[aoc(day5, part1)]
fn part1(input: &str) -> String {
    let (mut cargo, moves) = parse(input);
    println!("{cargo}");

    for mov in moves {
        println!("{mov}");
        cargo.do_move(mov);
        println!("{cargo}")
    }

    cargo.top_crate()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> String {
    let (mut cargo, moves) = parse(input);
    println!("{cargo}");

    for mov in moves {
        println!("{mov}");
        cargo.do_move_preserve_order(mov);
        println!("{cargo}")
    }

    cargo.top_crate()
}

pub fn main() {
    println!("{}",
             part2(include_str!("./test.txt"))
    );
}