#[aoc(day2, part1)]
fn part1(data: &str) -> u32 {
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

            let piece = me + 1;
            let score = (win_or_lose + piece) as u32;


            eprintln!("{}: {} + {} = {}", line, win_or_lose, piece, score);

            score
        })
        .sum::<u32>()
}

#[aoc(day2, part2)]
fn part2(data: &str) -> u32 {
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
        .sum::<u32>()
}