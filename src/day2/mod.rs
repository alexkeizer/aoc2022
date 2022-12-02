fn part1() {
    let data = include_str!("./input.txt");

    let res =
        data.lines()
            .map(|line| {
                let mut bytes = line.bytes();
                let op = bytes.next().unwrap() - b'A';
                bytes.next();
                let me = bytes.next().unwrap() - b'X';

                let win_or_lose = if op == me {
                    3 // draw
                } else {
                    match (op, me) {
                        (0, 1) => 6,
                        (1, 2) => 6,
                        (2, 0) => 6,
                        (_, _) => 0,
                    }
                };

                let piece = (me + 1);
                let score = (win_or_lose + piece) as u32;


                eprintln!("{}: {} + {} = {}", line, win_or_lose, piece, score);

                score
            })
            .sum::<u32>();

    println!("{}", res)
}

fn part2() {
    let data = include_str!("./input.txt");

    let res =
        data.lines()
            .map(|line| {
                let mut bytes = line.bytes();
                let op = bytes.next().unwrap() - b'A';
                bytes.next();
                let tgt = bytes.next().unwrap() - b'X';

                let win_or_lose = tgt * 3;

                let piece = (tgt + 2 + op) % 3 + 1;
                let score = (win_or_lose + piece) as u32;

                eprintln!("{}: {} + {} = {}", line, win_or_lose, piece, score);

                score
            })
            .sum::<u32>();

    println!("{}", res)
}

pub fn main() {
    part2()
}