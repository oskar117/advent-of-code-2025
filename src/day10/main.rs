use std::collections::{HashSet, VecDeque};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

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
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        println!("processing {:?}, current res = {}, button_len = {}", target_joltage, result, buttons[index].len());
        result += process_joltage(&target_joltage, &buttons[index]);
        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        println!("processed {:?} in {:?}\n", target_joltage, end - start);
    }

    result
}

fn process_joltage(target_joltage: &Vec<i32>, buttons: &Vec<Vec<usize>>) -> i32 {
    let mut cache = HashSet::<(u128, &Vec<usize>, i32)>::new();
    let encoded_target_joltage = encode_joltages(target_joltage);

    let mut queue = VecDeque::from_iter(reduce_possible_presses(&mut cache, encoded_target_joltage, 0, 0, &buttons, target_joltage.len()).into_iter());

    while let Some(queue_message) = queue.pop_front() {
        if cache.contains(&queue_message) {
            continue;
        }
        cache.insert(queue_message.clone());
        let (processed_joltage, button_to_press, press_count) = &queue_message;
        if *press_count > 300 {
            println!("LIMITER HIT");
            continue;
        }
        if joltages_equal(*processed_joltage, encoded_target_joltage, target_joltage.len()) {
            println!("got result from {:?} = {}", target_joltage, press_count);
            return *press_count;
        }
        let mut new_joltage = processed_joltage.clone();
        for btn_index in button_to_press.iter() {
            new_joltage = increment_joltage(new_joltage, *btn_index)
        }
        if joltages_equal(new_joltage, encoded_target_joltage, target_joltage.len()) {
            println!("got result from {:?} = {}", target_joltage, press_count + 1);
            return press_count + 1;
        }
        if (0..target_joltage.len()).any(|idx| get_joltage(new_joltage, idx) > get_joltage(encoded_target_joltage, idx)) {
            continue
        }

        let new_entries = reduce_possible_presses(&mut cache, encoded_target_joltage, new_joltage, press_count + 1, &buttons, target_joltage.len());
        queue.extend(new_entries.into_iter().filter(|x| !cache.contains(x)));
    }
    0
}

fn reduce_possible_presses<'a>(
    cache: &mut HashSet<(u128, &'a Vec<usize>, i32)>,
    target_joltage: u128,
    current_joltage: u128,
    current_press: i32,
    buttons: &'a Vec<Vec<usize>>,
    joltage_length: usize
) -> Vec<(u128, &'a Vec<usize>, i32)> {
    // println!("reduc {:?} {}", current_joltage, current_press);
    let min_press_joltage = (0..joltage_length)
        .map(|i| {
            let current_val = get_joltage(current_joltage, i);
            let target_val = get_joltage(target_joltage, i);
            (i, target_val, buttons.iter().filter(|btn| btn.contains(&i)).count(), current_val)
        }
        )
        .filter(|(_, val, _, current_val)| current_val < val)
        .min_by_key(|(_,_,count, _)| *count)
        .unwrap();
    // println!("{:?} {:?}", min_press_joltage, current_joltage);
    let mut affected_buttons = buttons.iter().filter(|btn| btn.contains(&min_press_joltage.0) && btn.iter().all(|&idx| get_joltage(current_joltage, idx) < get_joltage(target_joltage, idx))).collect::<Vec<&Vec<usize>>>();
    if affected_buttons.is_empty() {
        return Vec::new();
    }
    affected_buttons.sort();
    // let not_affected_buttons = buttons.iter().filter(|btn| !btn.contains(&min_press_joltage.0)).collect::<Vec<&Vec<usize>>>();
    let mut reduction_queue = VecDeque::from(affected_buttons.iter()
        .map(|&btn| (current_joltage.clone(), btn, current_press))
        .collect::<Vec<(u128, &Vec<usize>, i32)>>()
    );
    let mut queue = Vec::<(u128, &Vec<usize>, i32)>::new();
    while let Some(queue_message) = reduction_queue.pop_front() {
        if cache.contains(&queue_message) {
            continue;
        }
        cache.insert(queue_message.clone());
        let (processed_joltage, button_to_press, press_count) = &queue_message;
        let mut new_joltage = processed_joltage.clone();
        for btn_index in button_to_press.iter() {
            new_joltage = increment_joltage(new_joltage, *btn_index)
        }
        if (0..joltage_length).any(|idx| get_joltage(new_joltage, idx) > get_joltage(target_joltage, idx)) {
            continue
        }
        if get_joltage(new_joltage, min_press_joltage.0) == get_joltage(target_joltage, min_press_joltage.0) {
            buttons
                .iter()
                .filter(|btn| btn.iter().filter(|idx| **idx != min_press_joltage.0).all(|&idx| get_joltage(new_joltage, idx) <= get_joltage(target_joltage, idx)))
                .map(|btn| (new_joltage.clone(), btn, press_count + 1))
                .filter(|element| !cache.contains(element))
                .for_each(|el| { queue.push(el); });
            continue;
        }
        affected_buttons
            .iter()
            .filter(|btn| btn.iter().all(|&idx| get_joltage(new_joltage, idx) < get_joltage(target_joltage, idx)))
            .map(|&btn| (new_joltage.clone(), btn, press_count + 1))
            .filter(|element| !cache.contains(element))
            .for_each(|el| reduction_queue.push_back(el));
    }
    // println!("{:?}", queue);
    queue
}

fn encode_joltages(joltages: &Vec<i32>) -> u128 {
    let mut state = 0u128;
    for (i, &val) in joltages.iter().enumerate() {
        state |= ((val as u128) & 0xFF) << (i * 8)
    }
    state
}

fn get_joltage(encoded: u128, idx: usize) -> i32 {
    ((encoded >> (idx * 8)) & 0xFF) as i32
}


fn increment_joltage(encoded: u128, idx: usize) -> u128 {
    let shift = idx * 8;
    let current = (encoded >> shift) & 0xFF;
    let updated = current + 1;

    let clear_mask = !(0xFFu128 << shift);
    let cleared = encoded & clear_mask;

    cleared | (updated << shift)
}

fn joltages_equal(encoded1: u128, encoded2: u128, n: usize) -> bool {
    for i in 0..n {
        if get_joltage(encoded1, i) != get_joltage(encoded2, i) {
            return false;
        }
    }
    true
}