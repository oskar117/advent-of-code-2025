use std::fs;
use std::str::FromStr;

fn main() {
    let filename = "data2.txt";
    println!("Part1: {}", part1(filename));
    println!("Part2: {}", part2(filename));
}

fn part1(filename: &str) -> u64 {
    fs::read_to_string(filename)
        .unwrap()
        .split("\n")
        .map(|bank| {
            let chars = bank
                .trim()
                .split_terminator("")
                .skip(1)
                .map(|x| i32::from_str(x).unwrap())
                .collect::<Vec<i32>>();
            let (first_index, first_battery) = chars[..chars.len() - 1]
                .iter()
                .max()
                .map(|x| (chars.iter().position(|p| p == x).unwrap(), x))
                .unwrap();
            let second_battery = chars[(first_index + 1)..].iter().max().unwrap();
            let i = (first_battery.to_string() + second_battery.to_string().as_ref())
                .parse::<u64>()
                .unwrap();
            i
        })
        .sum()
}

fn part2(filename: &str) -> u64 {
    fs::read_to_string(filename)
        .unwrap()
        .split("\n")
        .map(|bank| {
            let mut result = Vec::<i32>::new();
            let chars = bank
                .trim()
                .split_terminator("")
                .skip(1)
                .map(|x| i32::from_str(x).unwrap())
                .collect::<Vec<i32>>();

            let mut start_index = 0;
            for i in 0..12 {
                let chars_split = chars[start_index..chars.len() - 11 + i]
                    .iter()
                    .collect::<Vec<&i32>>();
                let battery = chars_split.iter().max().unwrap();
                let max_index = chars_split.iter().position(|p| *p == *battery).unwrap();
                start_index = start_index + max_index + 1;
                result.push(**battery);
            }
            result
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("")
                .parse::<u64>()
                .unwrap()
        })
        .sum()
}
