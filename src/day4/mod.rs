use regex::Regex;

fn part1() {
    let re = Regex::new("^(\\d+)-(\\d+),(\\d+)-(\\d+)$").unwrap();

    let c =
        include_str!("./input.txt")
            .lines()
            .filter(|line| {
                println!("{}", line);
                let digits: [u32; 4] = {
                    let captures = re.captures(line).unwrap();
                    println!("{:?}", captures);
                    [
                        &captures[1],
                        &captures[2],
                        &captures[3],
                        &captures[4],
                    ].map(|c| c.parse().unwrap())
                };

                let overlap =
                    (digits[0] <= digits[2] && digits[1] >= digits[3])
                        || (digits[0] >= digits[2] && digits[1] <= digits[3]);
                println!("Overlap: {}", overlap);
                overlap
            })
            .count();

    println!("{}", c)
}

fn part2() {
    let re = Regex::new("^(\\d+)-(\\d+),(\\d+)-(\\d+)$").unwrap();

    let c =
        include_str!("./input.txt")
            .lines()
            .filter(|line| {
                println!("{}", line);
                let digits: [u32; 4] = {
                    let captures = re.captures(line).unwrap();
                    println!("{:?}", captures);
                    [
                        &captures[1],
                        &captures[2],
                        &captures[3],
                        &captures[4],
                    ].map(|c| c.parse().unwrap())
                };

                let overlap =
                    (digits[0] <= digits[2] && digits[2] <= digits[1])
                        || (digits[0] <= digits[3] && digits[3] <= digits[1])
                        || (digits[2] <= digits[0] && digits[0] <= digits[3])
                        || (digits[2] <= digits[1] && digits[1] <= digits[3]);
                println!("Overlap: {}", overlap);
                overlap
            })
            .count();

    println!("{}", c)
}

pub fn main() {
    part2()
}