use std::fs;

fn main() {
    let filename = "data2.txt";
    println!("Part1: {}", part1(filename));
    println!("Part2: {}", part2(filename));
}

fn part1(filename: &str) -> u64 {
    let input_string = fs::read_to_string(filename).unwrap();
    let split_input = input_string.split("\n").collect::<Vec<&str>>();
    let numbers = split_input[..split_input.len() - 1]
        .iter()
        .map(|x| {
            x.split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();
    let operators = split_input[split_input.len() - 1]
        .split_whitespace()
        .collect::<Vec<_>>();

    let mut results = Vec::<u64>::new();
    for i in 0..operators.len() {
        let operator = operators[i];
        let current_numbers = numbers.iter().map(|x| x[i]).collect::<Vec<u64>>();
        let result = match operator {
            "*" => current_numbers.iter().fold(1, |x, y| x * y),
            "+" => current_numbers.iter().fold(0, |x, y| x + y),
            _ => panic!("Unknown operator: {}", operator),
        };
        results.push(result);
    }

    results.iter().sum()
}

fn part2(filename: &str) -> u64 {
    let input_string = fs::read_to_string(filename).unwrap();
    let split_input = input_string.split("\n").collect::<Vec<&str>>();
    let operators = split_input[split_input.len() - 1]
        .split_whitespace()
        .collect::<Vec<_>>();

    let numbers = split_input[..split_input.len() - 1]
        .iter()
        .map(|x| x.split_terminator("").skip(1).collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let longest_row_length = numbers.iter().map(|x| x.len()).max().unwrap();

    let mut rotated_numbers = Vec::<Vec<&str>>::with_capacity(longest_row_length);

    for ry in 0..longest_row_length {
        let mut new_row = Vec::<&str>::with_capacity(numbers.len());
        for rx in 0..numbers.len() {
            if ry < numbers[rx].len() {
                new_row.push(numbers[rx][ry]);
            }
        }
        rotated_numbers.push(new_row)
    }

    let joined_numbers = rotated_numbers
        .iter()
        .map(|x| x.join("").trim().to_string())
        .chain(std::iter::once("".to_string()))
        .collect::<Vec<String>>();

    let mut results = Vec::<u64>::with_capacity(numbers.len());
    let mut operator_index = 0;
    let mut problem_numbers = Vec::<u64>::new();
    for number_index in 0..joined_numbers.len() {
        if joined_numbers[number_index].is_empty() {
            let operator = operators[operator_index];
            let result = match operator {
                "*" => problem_numbers.iter().fold(1, |x, y| x * y),
                "+" => problem_numbers.iter().fold(0, |x, y| x + y),
                _ => panic!("Unknown operator: {}", operator),
            };
            operator_index += 1;
            problem_numbers.clear();
            results.push(result);
        } else {
            let number = joined_numbers[number_index].parse::<u64>().unwrap();
            problem_numbers.push(number);
        }
    }

    results.iter().sum()
}
