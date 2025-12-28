use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

//bottlenecks: 91, 112, 160, 168
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
                .collect::<Vec<(Vec<bool>, Vec<usize>, i32)>>(),
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

fn find_patterns(light: &Vec<bool>, buttons: &Vec<Vec<usize>>) -> BTreeSet<BTreeSet<Vec<usize>>> {
    let mut queue = VecDeque::from(
        buttons
            .clone()
            .iter()
            .map(|btn| (light.clone(), btn.clone(), 0, BTreeSet::new()))
            .collect::<Vec<(Vec<bool>, Vec<usize>, i32, BTreeSet<Vec<usize>>)>>(),
    );
    let mut cache = HashSet::<(Vec<bool>, Vec<usize>, i32, BTreeSet<Vec<usize>>)>::new();

    let mut result = BTreeSet::<BTreeSet<Vec<usize>>>::new();
    while let Some(el) = queue.pop_front() {
        if !cache.insert(el.clone()) {
            continue;
        }
        let (processed_light, button_to_press, press_count, mut history) = el;
        let mut new_light = processed_light.clone();
        if press_count > light.len() as i32 {
            continue;
        }
        if history.contains(&button_to_press) {
            continue;
        }
        for btn_index in button_to_press.iter() {
            new_light[*btn_index] = !processed_light[*btn_index];
        }
        history.insert(button_to_press);
        if new_light.iter().all(|x| *x == false) {
            result.insert(history);
            continue;
        }
        buttons
            .iter()
            .filter(|btn| !history.contains(*btn))
            .map(|btn| {
                (
                    new_light.clone(),
                    btn.clone(),
                    press_count + 1,
                    history.clone(),
                )
            })
            .for_each(|el| queue.push_back(el))
    }
    result
}

fn part2(filename: &str) -> i32 {
    let input_as_string = fs::read_to_string(filename).unwrap();
    let input = input_as_string
        .split("\n")
        .map(|x| x.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let target_joltages = input[0..]
        .iter()
        .map(|line| line.last().unwrap())
        .map(|x| {
            x[1..x.len() - 1]
                .split(",")
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let buttons = input[0..]
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
        println!(
            "processing {:?}, current res = {}, button_len = {}",
            target_joltage,
            result,
            buttons[index].len()
        );

        let mut cache = HashMap::new();
        let loc_res = explore_path(&mut cache, target_joltage.clone(), &buttons[index]);
        result += loc_res;
        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        println!(
            "processed {:?} in {:?}, final result = {} \n",
            target_joltage,
            end - start,
            loc_res
        );
    }

    result
}

fn explore_path(cache: &mut HashMap<Vec<i32>, i32>, target_joltage: Vec<i32>, buttons: &Vec<Vec<usize>>) -> i32 {
    if cache.contains_key(&target_joltage) {
        return cache[&target_joltage];
    }
    if target_joltage.iter().all(|&x| x <= 0) {
        return 0;
    }
    let buttons = buttons.clone();
    let light = target_joltage
        .iter()
        .map(|x| x % 2 == 1)
        .collect::<Vec<bool>>();
    let patterns = find_patterns(&light, buttons.as_ref());

    // println!("got result from {:?} = {:?}, {:?}", target_joltage, patterns, light);
    let mut local_results = Vec::new();
    if patterns.is_empty() {
        // return 1000;
        // let loc_joltage = target_joltage.clone();
        local_results
            .push(1000);
    } else {
        for pattern in patterns.iter() {
            let mut local_target = target_joltage.clone();
            for btn in pattern.iter() {
       /*         if btn.iter().any(|&x| x < 1) {
                    continue;
                }*/
                for i in btn {
                    local_target[*i] -= 1
                }
            }
            if local_target.iter().any(|&x| x < 0) {
                // println!("skipping {:?} {:?} {:?}", target_joltage, local_target, pattern);
                continue;
            }
            // println!("local: {:?} -> {:?} {:?}", target_joltage, local_target, pattern);
            let patter_len = pattern.len();
            let btns = buttons.clone();
            // let handle = thread::spawn(move || {
            //     (2 * process_joltage(
            //         &local_target.iter().map(|x| x / 2).collect(),
            //         &btns
            //     )) + patter_len as i32
            // });
            // local_results.push(handle);
            local_results.push(
                2 * explore_path(cache, local_target.iter().map(|x| x / 2).collect(), &btns)
                    + patter_len as i32,
            )
        }
    }
    // let loc_res = local_results.into_iter().map(|thread| thread.join().unwrap()).min().unwrap();
    let loc_res = local_results.iter().min().unwrap_or(&1000);
    cache.insert(target_joltage.clone(), *loc_res);
    // println!("target_joltage: {:?} {} {:?} {:?}", target_joltage, loc_res, local_results, patterns);
    *loc_res
}

fn process_joltage(target_joltage: &Vec<i32>, buttons: &Vec<Vec<usize>>) -> i32 {
    let mut cache = HashSet::<(u128, &Vec<usize>, i32)>::new();
    let encoded_target_joltage = encode_joltages(target_joltage);

    let mut queue = VecDeque::from_iter(
        reduce_possible_presses(
            &mut cache,
            encoded_target_joltage,
            0,
            0,
            &buttons,
            target_joltage.len(),
        )
        .into_iter(),
    );

    let mut min = 1000;
    while let Some(queue_message) = queue.pop_front() {
        if cache.contains(&queue_message) {
            continue;
        }
        // if cache.len() > 1000000 {
        //     cache.clear()
        // }
        cache.insert(queue_message.clone());
        // let peek: Vec<_> = (0..target_joltage.len()).map(|x| get_joltage(queue_message.0, x)).collect();
        // println!("{:?} {:?} {}", peek, queue_message.1, queue_message.2);
        let (processed_joltage, button_to_press, press_count) = &queue_message;
        // if *button_to_press < *previous_button {
        //     continue;
        // }
        if *press_count > 300 {
            println!("LIMITER HIT");
            continue;
        }
        if *press_count > min && min != 1000 {
            // println!("larger than min");
            continue;
        }
        if joltages_equal(
            *processed_joltage,
            encoded_target_joltage,
            target_joltage.len(),
        ) {
            // println!("got result from {:?} = {}", target_joltage, press_count);
            // return *press_count;
            if *press_count < min {
                min = *press_count;
            }
            continue;
        }
        let mut new_joltage = processed_joltage.clone();
        for btn_index in button_to_press.iter() {
            new_joltage = increment_joltage(new_joltage, *btn_index)
        }
        if joltages_equal(new_joltage, encoded_target_joltage, target_joltage.len()) {
            // println!("got result from {:?} = {}", target_joltage, press_count + 1);
            if *press_count + 1 < min {
                min = *press_count + 1;
            }
            continue;
        }
        if (0..target_joltage.len())
            .any(|idx| get_joltage(new_joltage, idx) > get_joltage(encoded_target_joltage, idx))
        {
            continue;
        }
        if *press_count + 1 > min && min != 1000 {
            // println!("larger than min");
            continue;
        }

        let new_entries = reduce_possible_presses(
            &mut cache,
            encoded_target_joltage,
            new_joltage,
            press_count + 1,
            &buttons,
            target_joltage.len(),
        );
        queue.extend(new_entries.into_iter().filter(|x| !cache.contains(x)));
    }
    min
}

fn reduce_possible_presses<'a>(
    cache: &mut HashSet<(u128, &'a Vec<usize>, i32)>,
    target_joltage: u128,
    current_joltage: u128,
    current_press: i32,
    buttons: &'a Vec<Vec<usize>>,
    joltage_length: usize,
) -> Vec<(u128, &'a Vec<usize>, i32)> {
    // println!("reduc {:?} {}", current_joltage, current_press);
    let min_press_joltage = (0..joltage_length)
        .map(|i| {
            let current_val = get_joltage(current_joltage, i);
            let target_val = get_joltage(target_joltage, i);
            (
                i,
                target_val,
                buttons.iter().filter(|btn| btn.contains(&i)).count(),
                current_val,
            )
        })
        .filter(|(_, val, _, current_val)| current_val < val)
        .min_by_key(|(_, _, count, _)| *count)
        .unwrap();
    // println!("{:?} {:?}", min_press_joltage, current_joltage);
    let mut affected_buttons = buttons
        .iter()
        .filter(|btn| {
            btn.contains(&min_press_joltage.0)
                && btn.iter().all(|&idx| {
                    get_joltage(current_joltage, idx) < get_joltage(target_joltage, idx)
                })
        })
        .collect::<Vec<&Vec<usize>>>();
    if affected_buttons.is_empty() {
        return Vec::new();
    }
    affected_buttons.sort();
    // let not_affected_buttons = buttons.iter().filter(|btn| !btn.contains(&min_press_joltage.0)).collect::<Vec<&Vec<usize>>>();
    let mut reduction_queue = VecDeque::from(
        affected_buttons
            .iter()
            .map(|&btn| (current_joltage.clone(), btn, current_press))
            .collect::<Vec<(u128, &Vec<usize>, i32)>>(),
    );
    let mut queue = Vec::<(u128, &Vec<usize>, i32)>::new();
    while let Some(queue_message) = reduction_queue.pop_front() {
        if cache.contains(&queue_message) {
            continue;
        }
        cache.insert(queue_message.clone());
        let (processed_joltage, button_to_press, press_count) = &queue_message;
        // if *button_to_press < *previous_button {
        //     continue;
        // }
        let mut new_joltage = processed_joltage.clone();
        for btn_index in button_to_press.iter() {
            new_joltage = increment_joltage(new_joltage, *btn_index)
        }
        if (0..joltage_length)
            .any(|idx| get_joltage(new_joltage, idx) > get_joltage(target_joltage, idx))
        {
            continue;
        }
        if get_joltage(new_joltage, min_press_joltage.0)
            == get_joltage(target_joltage, min_press_joltage.0)
        {
            buttons
                .iter()
                .filter(|btn| {
                    btn.iter()
                        .filter(|idx| **idx != min_press_joltage.0)
                        .all(|&idx| {
                            get_joltage(new_joltage, idx) <= get_joltage(target_joltage, idx)
                        })
                })
                .map(|btn| (new_joltage.clone(), btn, press_count + 1))
                .filter(|element| !cache.contains(element))
                .for_each(|el| {
                    queue.push(el);
                });
            continue;
        }
        affected_buttons
            .iter()
            .filter(|btn| {
                btn.iter()
                    .all(|&idx| get_joltage(new_joltage, idx) < get_joltage(target_joltage, idx))
            })
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
