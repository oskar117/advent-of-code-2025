use std::fs;

fn main() {
    let filename = "data2.txt";
    println!("Part1: {}", part1(filename));
    println!("Part2: {}", part2(filename));
}

fn part1(filename: &str) -> i32 {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| (s[..1].to_string(), s[1..].parse::<i32>().unwrap()))
        .map(|(x, y)| if x == "R" { y } else { y * -1 })
        .fold((0, 50), |(reached_zeros, dial), number| {
            let res = (100 + dial + number) % 100;
            if res == 0 {
                (reached_zeros + 1, res)
            } else {
                (reached_zeros, res)
            }
        })
        .0
}

fn part2(filename: &str) -> i32 {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| (s[..1].to_string(), s[1..].parse::<i32>().unwrap()))
        .map(|(x, y)| if x == "R" { y } else { y * -1 })
        .fold((0, 50), |(reached_zeros, dial), number| {
            let multiplier = (number / 100).abs() + 1;
            let res = ((multiplier * 100) + dial + number) % 100;
            let raw_res = dial + number;
            let new_zeros = if raw_res >= 100 {
                raw_res / 100
            } else if raw_res <= 0 {
                let modulator = if dial == 0 { -1 } else { 0 };
                ((raw_res / 100).abs() + 1) + modulator
            } else {
                0
            };
            (reached_zeros + new_zeros, res)
        })
        .0
}
