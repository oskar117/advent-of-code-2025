use std::fs;

fn main() {
    let filename = "data2.txt";
    println!("Part1: {}", part1(filename));
    println!("Part2: {}", part2(filename));
}

fn part1(filename: &str) -> u64 {
    fs::read_to_string(filename)
        .unwrap()
        .split(",")
        .map(|x| x.split_once("-").unwrap())
        .map(|(x, y)| (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap()))
        .flat_map(|(low, high)| {
            (low..=high)
                .filter(|x| x.to_string().len() % 2 == 0)
                .filter(|x| {
                    let as_string = x.to_string();
                    let split = as_string.split_at(as_string.len() / 2);
                    split.0 == split.1
                })
        })
        .sum()
}

fn part2(filename: &str) -> u64 {
    fs::read_to_string(filename)
        .unwrap()
        .split(",")
        .map(|x| x.split_once("-").unwrap())
        .map(|(x, y)| (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap()))
        .flat_map(|(low, high)| {
            (low..=high).filter(|x| {
                let as_string = x.to_string();
                for num in 1..x.ilog10() + 1 {
                    if (as_string.len() as u32) % num != 0 {
                        continue;
                    }
                    let strings = as_string.as_bytes()
                        .chunks(num as usize)
                        .map(str::from_utf8)
                        .map(|x| x.unwrap())
                        .collect::<Vec<_>>();
                    let first = strings.first().unwrap();
                    if strings.iter().all(|x| x == first) {
                        return true;
                    }
                }
                false
            })
        })
        .sum()
}
