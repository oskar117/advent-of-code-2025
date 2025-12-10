use std::collections::VecDeque;
use std::fs;

fn main() {
    let filename = "data2.txt";
    // println!("Part1: {}", part1(filename));
    println!("Part2: {}", part2(filename));
}

fn part1(filename: &str) -> i32 {
    let input_as_string = fs::read_to_string(filename).unwrap();
    let input = input_as_string
        .split("\n")
        .map(|x| x.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let lights = input
        .iter()
        .map(|line| line[0])
        .map(|x| {
            x[1..x.len() - 1]
                .chars()
                .map(|y| if y == '#' { true } else { false })
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();

    let buttons = input
        .iter()
        .map(|line| {
            line[1..line.len() - 1]
                .iter()
                .map(|btn| {
                    btn[1..btn.len() - 1]
                        .split(",")
                        .map(|idx| idx.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>()
        })
        .collect::<Vec<Vec<Vec<usize>>>>();

    let mut result = 0;
    for (index, light) in lights.iter().enumerate() {
        let mut queue = VecDeque::from(
            buttons[index]
                .clone()
                .iter()
                .map(|btn| (light.clone(), btn.clone(), 0))
                .collect::<Vec<(Vec<bool>, Vec<usize>, i32)>>()
        );

        loop {
            let (processed_light, button_to_press, press_count) = queue.pop_front().unwrap();
            let mut new_light = processed_light.clone();
            for btn_index in button_to_press.iter() {
               new_light[*btn_index] = !processed_light[*btn_index];
            }
            if new_light.iter().all(|x| *x == false) {
                result += press_count + 1;
                break;
            }
            buttons[index]
                .clone()
                .iter()
                .map(|btn| (new_light.clone(), btn.clone(), press_count + 1))
                .for_each(|el| queue.push_back(el))
        }
    }

    result
}

fn part2(filename: &str) -> i32 {
    let input_as_string = fs::read_to_string(filename).unwrap();
    let input = input_as_string
        .split("\n")
        .map(|x| x.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let target_joltages = input
        .iter()
        .map(|line| line.last().unwrap())
        .map(|x| {
            x[1..x.len() - 1]
                .split(",")
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let buttons = input
        .iter()
        .map(|line| {
            line[1..line.len() - 1]
                .iter()
                .map(|btn| {
                    btn[1..btn.len() - 1]
                        .split(",")
                        .map(|idx| idx.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>()
        })
        .collect::<Vec<Vec<Vec<usize>>>>();

    let mut result = 0;
    for (index, target_joltage) in target_joltages.iter().enumerate() {
        println!("processing {:?}, current res = {}", target_joltage, result);
        let mut queue = VecDeque::from(
            buttons[index]
                .clone()
                .iter()
                .map(|btn| (vec![0; target_joltage.len()], btn.clone(), 0))
                .collect::<Vec<(Vec<i32>, Vec<usize>, i32)>>()
        );

        loop {
            let (processed_joltage, button_to_press, press_count) = queue.pop_front().unwrap();
            let mut new_joltage = processed_joltage.clone();
            for btn_index in button_to_press.iter() {
                new_joltage[*btn_index] += 1;
            }
            if new_joltage == *target_joltage {
                result += press_count + 1;
                break;
            }
            if new_joltage.iter().enumerate().any(|(idx, elemeent)| elemeent > &target_joltage[idx]) {
                continue
            }
            buttons[index]
                .clone()
                .iter()
                .map(|btn| (new_joltage.clone(), btn.clone(), press_count + 1))
                .for_each(|el| queue.push_back(el))
        }
    }

    result
}
