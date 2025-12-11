use std::collections::{HashMap, VecDeque};
use std::fs;

fn main() {
    let filename = "data2.txt";
    println!("Part1: {}", part1(filename));
    println!("Part2: {}", part2(filename));
}

fn part1(filename: &str) -> u64 {
    let input_as_string = fs::read_to_string(filename).unwrap();
    let devices = input_as_string
        .lines()
        .map(|x| {
            let split: Vec<&str> = x.split(": ").collect();
            let label = split[0];
            let outputs = split[1].split_whitespace().collect::<Vec<&str>>();
            (label, outputs)
        })
        .collect::<HashMap<&str, Vec<&str>>>();

    let mut result = 0;
    let mut queue = VecDeque::<&&str>::new();
    queue.push_back(&"you");
    while !queue.is_empty() {
        let next = queue.pop_back().unwrap();
        if *next != "out" {
            devices[next].iter().for_each(|x| queue.push_back(x));
            continue;
        }
        result += 1;
    }
    result
}

fn part2(filename: &str) -> u64 {
    let input_as_string = fs::read_to_string(filename).unwrap();
    let devices = input_as_string
        .lines()
        .map(|x| {
            let split: Vec<&str> = x.split(": ").collect();
            let label = split[0];
            let outputs = split[1].split_whitespace().collect::<Vec<&str>>();
            (label, outputs)
        })
        .collect::<HashMap<&str, Vec<&str>>>();

    let svr_to_fft = count_paths(&mut HashMap::new(), &devices, &vec!["out", "dac"], "svr", "fft");
    let svr_to_dac = count_paths(&mut HashMap::new(), &devices, &vec!["out", "fft"], "svr", "dac");
    let dac_to_fft = count_paths(&mut HashMap::new(), &devices, &vec!["out"], "dac", "fft");
    let dac_to_out = count_paths(&mut HashMap::new(), &devices, &vec!["fft"], "dac", "out");
    let fft_to_dac = count_paths(&mut HashMap::new(), &devices, &vec!["out"], "fft", "dac");
    let fft_to_out = count_paths(&mut HashMap::new(), &devices, &vec!["dac"], "fft", "out");

    (svr_to_dac * dac_to_fft * fft_to_out) + (svr_to_fft * fft_to_dac * dac_to_out)
}

fn count_paths<'a>(
    cache: &mut HashMap<&'a str, u64>,
    devices: &HashMap<&str, Vec<&'a str>>,
    forbidden: &Vec<&str>,
    from: &'a str,
    to: &str,
) -> u64 {
    if cache.contains_key(from) {
        return cache[from];
    }
    if from == to {
        return 1;
    }
    if forbidden.contains(&from) {
        return 0;
    }
    let res = devices[from]
        .iter()
        .map(|x| count_paths(cache, devices, forbidden, x, to))
        .sum();
    cache.insert(from, res);
    res
}
