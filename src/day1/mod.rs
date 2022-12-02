use std::io::{BufRead, stdin};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Elf {
    total: u32
}

impl Elf {
    fn new() -> Self {
        Self {
            total: 0
        }
    }

    fn add(&mut self, calories: u32) {
        self.total += calories;
    }
}

pub fn main() {
    let mut cur = Elf::new();
    let mut all = Vec::new();


    for line in stdin().lock().lines() {
        let line = line.unwrap();
        let line = line.trim();

        if line.is_empty() {
            all.push(cur);
            cur = Elf::new();
        } else {
            cur.add(
                line.parse().unwrap()
            )
        }
    }

    if cur.total > 0 {
        all.push(cur)
    }

    all.sort();

    let sum: u32 = (&all[all.len()-3..]).iter().map(|e| e.total).sum();
    println!("{}", sum);
}