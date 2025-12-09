use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs;
use std::ops::Add;

fn main() {
    let filename = "data2.txt";
    // println!("Part1: {}", part1(filename));
    println!("Part2: {}", part2(filename));
}

fn part1(filename: &str) -> i64 {
    let coords = fs::read_to_string(filename)
        .unwrap()
        .split("\n")
        .map(|x| x.split_once(",").unwrap())
        .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
        .collect::<Vec<(i64, i64)>>();

    let mut pairs = Vec::new();
    for first_pair_index in 0..coords.len() {
        for second_pair_index in first_pair_index..coords.len() {
            if first_pair_index == second_pair_index {
                continue;
            }
            pairs.push((coords[first_pair_index], coords[second_pair_index]));
        }
    }

    pairs
        .iter()
        .map(|((x1, y1), (x2, y2))| (x2 - x1 + 1).abs() * (y2 - y1 + 1).abs())
        .max()
        .unwrap()
}

fn part2(filename: &str) -> i64 {
    let coords = fs::read_to_string(filename)
        .unwrap()
        .split("\n")
        .map(|x| x.split_once(",").unwrap())
        .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
        .collect::<Vec<(i64, i64)>>();

    let mut pairs = Vec::new();
    for first_pair_index in 0..coords.len() {
        for second_pair_index in first_pair_index..coords.len() {
            if first_pair_index == second_pair_index {
                continue;
            }
            pairs.push((coords[first_pair_index], coords[second_pair_index]));
        }
    }

    pairs
        .iter()
        .filter(|(p1, p3)| {
            let p2 = (p3.0, p1.1);
            let p4 = (p1.0, p3.1);
            let vertices = vec![p1, &p2, p3, &p4];
            let inside = vertices.iter().all(|p| is_inside(*p, &coords));
            inside
        })
        .map(|((x1, y1), (x2, y2))| (x2 - x1).abs().add(1) * (y2 - y1).abs().add(1))
        .max()
        .unwrap()
}

fn is_inside(point: &(i64, i64), verticies: &Vec<(i64, i64)>) -> bool {
    let mut crossing = HashSet::<(i8, i8)>::new();
    for i in 0..verticies.len() {
        let p1 = verticies[i];
        let p2 = verticies[(i + 1) % verticies.len()];
        if (point.0 == p2.0 && point.0 == p1.0 && point.1 <= max(p1.1, p2.1) && point.1 >= min(p1.1, p2.1))
            || (point.1 == p1.1 && point.1 == p2.1 && point.0 <= max(p1.0, p2.0) && point.0 >= min(p1.0, p2.0))
        {
            return true;
        }
        if (point.0 >= min(p1.0, p2.0) && point.0 <= max(p1.0, p2.0) && p1.1 == p2.1 && point.1 <= p1.1) || (point.0 == p1.0 && p1.0 == p2.0 && point.1 > p1.1 && point.1 > p2.1){
            crossing.insert((0, -1));
        } else if point.0 >= min(p1.0, p2.0) && point.0 <= max(p1.0, p2.0) && p1.1 == p2.1 && point.1 >= p1.1 || (point.0 == p1.0 && p1.0 == p2.0 && point.1 > p1.1 && point.1 < p2.1) {
            crossing.insert((0, 1));
        } else if (point.1 >= min(p1.1, p2.1) && point.1 <= max(p1.1, p2.1) && p1.0 == p2.0 && point.0 <= p1.0) || (point.1 == p1.1 && p1.1 == p2.1 && point.0 > p1.0 && point.0 < p2.0) {
            crossing.insert((1, 0));
        } else if (point.1 >= min(p1.1, p2.1) && point.1 <= max(p1.1, p2.1) && p1.0 == p2.0 && point.0 >= p1.0) || (point.1 == p1.1 && p1.1 == p2.1 && point.0 > p1.0 && point.0 > p2.0)  {
            crossing.insert((-1, 0));
        }
    }
    crossing.len() == 4
}
