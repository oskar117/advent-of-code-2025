use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let filename = "data2.txt";
    println!("Part1: {}", part1(filename));
    println!("Part2: {}", part2(filename));
}

fn part1(filename: &str) -> u64 {
    let map = std::fs::read_to_string(&filename)
        .unwrap()
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let start_position = (map[0].iter().position(|&x| x == 'S').unwrap(), 0);
    let mut splits = 0;
    let mut seen = HashSet::<(usize, usize)>::new();
    let mut queue = VecDeque::<(usize, usize)>::new();
    queue.push_back(start_position);

    while !queue.is_empty() {
        let (el_x, el_y) = queue.pop_front().unwrap();
        if el_y >= map.len() - 1 {
            continue;
        }
        if seen.contains(&(el_x, el_y)) {
            continue;
        }
        seen.insert((el_x, el_y));
        match map[el_y + 1][el_x] {
            '.' => queue.push_back((el_x, el_y + 1)),
            '^' => {
                queue.push_back((el_x + 1, el_y + 1));
                queue.push_back((el_x - 1, el_y + 1));
                splits += 1;
            }
            _ => panic!("unknown char!"),
        }
    }
    splits
}

fn part2(filename: &str) -> u64 {
    let map = std::fs::read_to_string(&filename)
        .unwrap()
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let start_position = (map[0].iter().position(|&x| x == 'S').unwrap(), 0);
    let mut seen = HashMap::<(usize, usize), u64>::new();
    visit(&map, &mut seen, start_position.0, start_position.1)
}

fn visit(
    map: &Vec<Vec<char>>,
    visited: &mut HashMap<(usize, usize), u64>,
    x: usize,
    y: usize,
) -> u64 {
    if visited.contains_key(&(x, y)) {
        return *visited.get(&(x, y)).unwrap();
    }
    if y >= map.len() - 1 {
        visited.insert((x, y), 1);
        return 1;
    }
    let res = match map[y + 1][x] {
        '.' => {
            visit(map, visited, x, y + 1)
        }
        '^' => {
            visit(map, visited, x + 1, y + 1) + visit(map, visited, x - 1, y + 1)
        }
        _ => panic!("unknown char!"),
    };
    visited.insert((x, y + 1), res);
    res
}
