fn to_priority(x: u8) -> u8 {
    if b'a' <= x && x <= b'z' {
        x - b'a'
    } else if b'A' <= x && x <= b'Z' {
        x - b'A' + 26
    } else {
        panic!("invalid input");
    }
}

fn from_priority(x: u8) -> char {
    (
        if x >= 26 {
            x - 26 + b'A'
        } else {
            x + b'a'
        }
    ) as char
}


fn part1() {
    let input: &'static str = include_str!("./input.txt");

    let total: u32 =
        input
            .lines()
            .map(|line| {
                let half = line.len() / 2;

                println!("{}: {}_{}", line, &line[..half], &line[half..]);
                let line = line.as_bytes();


                let mut map = [false; 52];
                for &c in &line[..half] {
                    map[to_priority(c) as usize] = true;
                }

                let mut map2 = [false; 52];
                let mut total = 0;
                for &c in &line[half..] {
                    let p = to_priority(c);
                    if map[p as usize] && !map2[p as usize] {
                        println!("  {}: {}", c as char, p + 1);
                        map2[p as usize] = true;
                        total += p as u32 + 1;
                    }
                }

                println!("total (line): {}\n", total);
                total
            })
            .sum();

    println!("{}", total)
}

fn part2() {
    let input: &'static str = include_str!("./input.txt");

    let mut index = 0;
    let mut map = [[false; 52]; 3];
    let mut total = 0;

    for line in input.lines() {
        println!("{}", line);
        for &c in line.as_bytes() {
            map[index][to_priority(c) as usize] = true;
        }
        index += 1;
        if index >= 3 {
            for i in 0..52 {
                if map[0][i] && map[1][i] && map[2][i] {
                    let p = i + 1;
                    println!(" {}:{}", from_priority(i as u8), p);
                    total += p;
                }
            }
            index = 0;
            map = [[false; 52]; 3];
        }
    }

    println!("{}", total)
}

pub fn main() {
    part2()
}