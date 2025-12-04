use std::fs;

fn main() {
    let filename = "data2.txt";
    println!("Part1: {}", part1(filename));
    println!("Part2: {}", part2(filename));
}

fn part1(filename: &str) -> i32 {
    let map = fs::read_to_string(filename)
        .unwrap()
        .split("\n")
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<char>>>();

    let mut liftable = 0;
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            let c = map[x][y];
            if c == '.' {
                continue;
            }
            let mut adjacent_rolls = Vec::<char>::new();
            if x > 0 && y > 0 {
                adjacent_rolls.push(map[x - 1][y - 1])
            };
            if y > 0 {
                adjacent_rolls.push(map[x][y - 1])
            };
            if x < map[y].len() - 1 && y > 0 {
                adjacent_rolls.push(map[x + 1][y - 1])
            };
            if x > 0 {
                adjacent_rolls.push(map[x - 1][y])
            };
            if x < map[y].len() - 1 {
                adjacent_rolls.push(map[x + 1][y])
            };
            if x > 0 && y < map.len() - 1 {
                adjacent_rolls.push(map[x - 1][y + 1])
            };
            if y < map.len() - 1 {
                adjacent_rolls.push(map[x][y + 1])
            };
            if x < map[y].len() - 1 && y < map.len() - 1 {
                adjacent_rolls.push(map[x + 1][y + 1])
            };
            let rolls_count = adjacent_rolls.iter().filter(|x| **x == '@').count();
            if rolls_count < 4 {
                liftable += 1;
            }
        }
    }
    liftable
}

fn part2(filename: &str) -> i32 {
    let mut map = fs::read_to_string(filename)
        .unwrap()
        .split("\n")
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<char>>>();

    let mut liftable = 0;

    loop {
        let mut removable = Vec::<(usize, usize)>::new();
        for x in 0..map.len() {
            for y in 0..map[x].len() {
                let c = map[x][y];
                if c == '.' {
                    continue;
                }
                let mut adjacent_rolls = Vec::<(usize, usize)>::new();
                if x > 0 && y > 0 {
                    adjacent_rolls.push((x - 1, y - 1))
                };
                if y > 0 {
                    adjacent_rolls.push((x, y - 1))
                };
                if x < map[y].len() - 1 && y > 0 {
                    adjacent_rolls.push((x + 1, y - 1))
                };
                if x > 0 {
                    adjacent_rolls.push((x - 1, y))
                };
                if x < map[y].len() - 1 {
                    adjacent_rolls.push((x + 1, y))
                };
                if x > 0 && y < map.len() - 1 {
                    adjacent_rolls.push((x - 1, y + 1))
                };
                if y < map.len() - 1 {
                    adjacent_rolls.push((x, y + 1))
                };
                if x < map[y].len() - 1 && y < map.len() - 1 {
                    adjacent_rolls.push((x + 1, y + 1))
                };
                let rolls = adjacent_rolls
                    .iter()
                    .filter(|&pos| map[pos.0][pos.1] == '@')
                    .collect::<Vec<&(usize, usize)>>();

                if rolls.len() < 4 {
                    removable.push((x, y));
                    liftable += 1;
                }
                adjacent_rolls.clear()
            }
        }
        if removable.is_empty() {
            return liftable;
        }
        removable.iter().for_each(|(xx, yy)| map[*xx][*yy] = '.');
        removable.clear()
    }
}
