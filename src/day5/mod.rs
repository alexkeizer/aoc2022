use std::num::NonZeroU8;
use regex::Regex;
use pest::Parser;

#[derive(Parser)]
#[grammar_inline=r#"
    crat_some = { "[" ~ ASCII_ALPHA ~ "]" }
    crat_none = { "   " }
    crat = { crat_some | crat_none }
    crat_line = { crat ~ (" " ~ crat)* ~ NEWLINE? }

    number = @{ ASCII_DIGIT+ }
    move_line = {
        "move " ~ number ~ " from " ~ number ~ " to " ~ number ~ NEWLINE?
    }

    file = {
        crat_line+
        ~ (" " | ASCII_DIGIT)+ ~ "\n"
        ~ "\n"
        ~ move_line+
    }
"#]
struct MyParser;

struct Crates (Vec<Option<NonZeroU8>>);

struct Move {
    crat: u8,
    from: u8,
    to: u8,
}

fn part1() {
    let input = include_str!("./test.txt");

    let parse = MyParser::parse(Rule::file, input)
        .expect("Parse Error")
        .next().unwrap()
        .into_inner(); // get and unwrap the `file` rule; never fails;

    // let mut crates = Vec::new();
    // let mut moves = Vec::new();

    for pair in parse.clone().rev() {
        eprintln!("{:?}", pair);
        // match pair.as_rule() {
        //     Rule::
        // }
    }

    for pair in parse {
        eprintln!("{:?}", pair);
        // match pair.as_rule() {
        //     Rule::
        // }
    }

}

// fn part1() {
//     let re_box = Regex::new("^(\\s\\[?([A-Z]|\\s{3})])+$").unwrap();
//     let re_op = Regex::new("^move\\s+([0-9]+)\\s+from\\s+([0-9]+)\\s+to\\s+([0-9]+)\\s*$").unwrap();
//
//     println!("START");
//     let mut phase1 = true;
//     for line in include_str!("./test.txt").lines() {
//         println!("-- {}", line);
//         if phase1 {
//             if line.is_empty() {
//                 phase1 = false;
//             } else {
//                 re_box.captures_iter(line)
//                     .map(|capt| capt[2].chars().next()
//                         .map(|c| {
//                             if c.is_ascii_whitespace() {
//                                 None
//                             } else {
//                                 Some(c)
//                             }
//                         }))
//                         .flatten()
//                     .enumerate()
//                     .for_each(|(i, item)| {
//                         if let Some(item) = item {
//                             println!("Test: {} - {}", i, item)
//                         }
//                     })
//             }
//         } else if let Some(m) = re_op.captures(&line) {}
//     }
// }

pub fn main() {
    part1()
}