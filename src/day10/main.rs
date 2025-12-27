use std::collections::{HashSet, VecDeque};
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
        result += process_joltage(&target_joltage, &buttons[index]);
    }

    result
}

fn process_joltage(target_joltage: &Vec<i32>, buttons: &Vec<Vec<usize>>) -> i32 {
    let mut cache = HashSet::<(Vec<i32>, &Vec<usize>, i32)>::new();

    let mut queue = VecDeque::from_iter(reduce_possible_presses(&mut cache, &target_joltage, &vec![0; target_joltage.len()], 0, &buttons).into_iter());
    while let Some(queue_message) = queue.pop_front() {
        if cache.contains(&queue_message) {
            continue;
        }
        let (processed_joltage, button_to_press, press_count) = &queue_message;
        if processed_joltage == target_joltage {
            println!("got result from {:?} = {}", target_joltage, press_count);
            return *press_count;
        }
        let mut new_joltage = processed_joltage.clone();
        for btn_index in button_to_press.iter() {
            new_joltage[*btn_index] += 1;
        }
        if new_joltage == *target_joltage {
            println!("got result from {:?} = {}", target_joltage, press_count + 1);
            return press_count + 1;
        }
        if new_joltage.iter().enumerate().any(|(idx, elemeent)| elemeent > &target_joltage[idx]) {
            cache.insert(queue_message);
            continue
        }

        let new_entries = reduce_possible_presses(&mut cache, &target_joltage, &new_joltage, press_count + 1, &buttons);
        queue.extend(new_entries.into_iter().filter(|x| !cache.contains(x)));
        cache.insert(queue_message);
    }
    0
}

fn reduce_possible_presses<'a>(cache: &mut HashSet<(Vec<i32>, &'a Vec<usize>, i32)>, target_joltage: &Vec<i32>, current_joltage: &Vec<i32>, current_press: i32, buttons: &'a Vec<Vec<usize>>) -> HashSet<(Vec<i32>, &'a Vec<usize>, i32)> {
    // println!("reduc {:?} {}", current_joltage, current_press);
    let min_press_joltage = target_joltage.iter()
        .enumerate()
        .map(|(i, val)| (i, val, buttons.iter().filter(|btn| btn.contains(&i)).count()))
        .filter(|(i, val, _)| current_joltage[*i] < **val)
        .min_by_key(|(_,_,count)| *count)
        .unwrap();
    // println!("{:?} {:?}", min_press_joltage, current_joltage);
    let affected_buttons = buttons.iter().filter(|btn| btn.contains(&min_press_joltage.0) && btn.iter().all(|&idx| current_joltage[idx] < target_joltage[idx])).collect::<Vec<&Vec<usize>>>();
    if affected_buttons.is_empty() {
        return HashSet::new();
    }
    // let not_affected_buttons = buttons.iter().filter(|btn| !btn.contains(&min_press_joltage.0)).collect::<Vec<&Vec<usize>>>();
    let mut reduction_queue = VecDeque::from(affected_buttons.iter()
        .map(|&btn| (current_joltage.clone(), btn, current_press))
        .collect::<Vec<(Vec<i32>, &Vec<usize>, i32)>>()
    );
    let mut queue = HashSet::<(Vec<i32>, &Vec<usize>, i32)>::new();
    while let Some(queue_message) = reduction_queue.pop_front() {
        if cache.contains(&queue_message) {
            continue;
        }
        let (processed_joltage, button_to_press, press_count) = &queue_message;
        let mut new_joltage = processed_joltage.clone();
        for btn_index in button_to_press.iter() {
            new_joltage[*btn_index] += 1;
        }
        if new_joltage.iter().enumerate().any(|(idx, elemeent)| elemeent > &target_joltage[idx]) {
            continue
        }
        if new_joltage[min_press_joltage.0] == target_joltage[min_press_joltage.0] {
            buttons
                .iter()
                .filter(|btn| btn.iter().filter(|idx| **idx != min_press_joltage.0).all(|&idx| new_joltage[idx] <= target_joltage[idx]))
                .map(|btn| (new_joltage.clone(), btn, press_count + 1))
                .filter(|element| !cache.contains(element))
                .for_each(|el| { queue.insert(el); });
            cache.insert(queue_message);
            continue;
        }
        affected_buttons
            .iter()
            .filter(|btn| btn.iter().all(|&idx| new_joltage[idx] < target_joltage[idx]))
            .map(|&btn| (new_joltage.clone(), btn, press_count + 1))
            .filter(|element| !cache.contains(element))
            .for_each(|el| reduction_queue.push_back(el));
        cache.insert(queue_message);
    }
    // println!("{:?}", queue);
    queue
}
