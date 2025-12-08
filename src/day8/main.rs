use std::fs;
use std::ops::Range;

fn main() {
    let filename = "data2.txt";
    let connection_number = 1000;
    // println!("Part1: {}", part1(filename, connection_number));
    println!("Part2: {}", part2(filename));
}

type Coord = (i64, i64, i64);

#[derive(Debug)]
struct JunctionPair {
    a: Coord,
    b: Coord,
    distance: f64,
}

impl JunctionPair {
    fn new(a: Coord, b: Coord) -> JunctionPair {
        JunctionPair {
            a,
            b,
            distance: (((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)) as f64)
                .sqrt(),
        }
    }
}

impl PartialEq for JunctionPair {
    fn eq(&self, other: &Self) -> bool {
        (self.a == other.a && self.b == other.b) || (self.b == other.a && self.a == other.b)
    }
}

fn part1(filename: &str, connection_number: usize) -> u64 {
    let coords = fs::read_to_string(filename)
        .unwrap()
        .split("\n")
        .map(|x| {
            x.split(",")
                .map(|x| x.parse::<i64>().expect("not a number!"))
                .collect::<Vec<_>>()
        })
        .map(|vec| (vec[0], vec[1], vec[2]))
        .collect::<Vec<Coord>>();

    let mut pairs = Vec::<JunctionPair>::new();
    let mut circuits = coords
        .iter()
        .map(|coord| vec![coord.to_owned()])
        .collect::<Vec<Vec<Coord>>>();

    for first_pair_index in 0..coords.len() {
        for second_pair_index in first_pair_index..coords.len() {
            if first_pair_index == second_pair_index {
                continue;
            }
            pairs.push(JunctionPair::new(
                coords[first_pair_index],
                coords[second_pair_index],
            ));
        }
    }
    pairs.sort_by(|a, b| (&a).distance.partial_cmp(&b.distance).unwrap());

    let mut iterator = 0usize;
    let mut index = 0usize;
    while index < connection_number {
        let pair = pairs.get(index).unwrap();
        let first = circuits
            .iter()
            .position(|circut| circut.contains(&&pair.a))
            .unwrap();
        let second = circuits
            .iter()
            .position(|circut| circut.contains(&&pair.b))
            .unwrap();
        if first != second {
            let mut s = std::mem::take(&mut circuits[second]);
            circuits[first].append(&mut s);
            iterator += 1;
        }
        index += 1;
    }
    circuits.retain(|circuit| !circuit.is_empty());
    circuits.sort_by(|a, b| b.len().partial_cmp(&a.len()).unwrap());

    circuits
        .iter()
        .take(3)
        .map(|x| x.len())
        .fold(1, |a, b| a * b) as u64
}

fn part2(filename: &str) -> i64 {
    let coords = fs::read_to_string(filename)
        .unwrap()
        .split("\n")
        .map(|x| {
            x.split(",")
                .map(|x| x.parse::<i64>().expect("not a number!"))
                .collect::<Vec<_>>()
        })
        .map(|vec| (vec[0], vec[1], vec[2]))
        .collect::<Vec<Coord>>();

    let mut pairs = Vec::<JunctionPair>::new();
    let mut circuits = coords
        .iter()
        .map(|coord| vec![coord.to_owned()])
        .collect::<Vec<Vec<Coord>>>();

    for first_pair_index in 0..coords.len() {
        for second_pair_index in first_pair_index..coords.len() {
            if first_pair_index == second_pair_index {
                continue;
            }
            pairs.push(JunctionPair::new(
                coords[first_pair_index],
                coords[second_pair_index],
            ));
        }
    }
    pairs.sort_by(|a, b| (&a).distance.partial_cmp(&b.distance).unwrap());

    let mut index = 0usize;
    loop {
        let pair = pairs.get(index).unwrap();
        let first = circuits
            .iter()
            .position(|circut| circut.contains(&&pair.a))
            .unwrap();
        let second = circuits
            .iter()
            .position(|circut| circut.contains(&&pair.b))
            .unwrap();
        if first != second {
            let mut s = std::mem::take(&mut circuits[second]);
            circuits[first].append(&mut s);
            circuits.remove(second);
            if circuits.len() == 1 {
                return pair.a.0 * pair.b.0;
            }
        }
        index += 1;
    }
}
