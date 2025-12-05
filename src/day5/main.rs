use std::collections::VecDeque;
use std::fs;

fn main() {
    let filename = "data2.txt";
    println!("Part1: {}", part1(filename));
    println!("Part2: {}", part2(filename));
}

fn part1(filename: &str) -> i32 {
    let input = fs::read_to_string(filename).unwrap();
    let (ranges_input, ingredients_input) = input.split_once("\n\n").unwrap();

    let ranges = ranges_input
        .split("\n")
        .map(|range_string| range_string.split_once("-").unwrap())
        .map(|(low_str, high_str)| {
            low_str.parse::<u64>().unwrap()..=high_str.parse::<u64>().unwrap()
        })
        .collect::<Vec<_>>();

    ingredients_input
        .split("\n")
        .map(|as_str| as_str.parse::<u64>().unwrap())
        .filter(|ingredient| ranges.iter().any(|range| range.contains(ingredient)))
        .count() as i32
}

fn part2(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    let (ranges_input, _) = input.split_once("\n\n").unwrap();

    let mut ranges = ranges_input
        .split("\n")
        .map(|range_string| range_string.split_once("-").unwrap())
        .map(|(low_str, high_str)| {
            (low_str.parse::<u64>().unwrap(), high_str.parse::<u64>().unwrap())
        })
        .collect::<Vec<_>>();
    ranges.sort();

    let mut flattened_ranges = VecDeque::<(u64, u64)>::from(vec![ranges[0]]);
    for range in ranges.iter().skip(1) {
        let last_element = flattened_ranges.pop_back().unwrap();
        if last_element.1 < range.0 {
            flattened_ranges.push_back(last_element);
            flattened_ranges.push_back(*range);
        } else if last_element.1 < range.1 {
            flattened_ranges.push_back((last_element.0, range.1));
        } else {
            flattened_ranges.push_back(last_element);
        }
    }

    flattened_ranges.iter().map(|(low, high)| high - low + 1).sum()
}
