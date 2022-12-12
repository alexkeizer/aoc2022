use regex::Regex;

fn part1() {
    let re = Regex::new("\\[(A-Z)]").unwrap();

    for line in include_str!("./test.txt").lines() {
        re.captures_iter(line)
            .map(|capt| capt[1].chars().next().unwrap())
            .enumerate()
            .for_each(|(i, item)| {
                println!("Test")
            })
    }
}

pub fn main() {
    part1()
}