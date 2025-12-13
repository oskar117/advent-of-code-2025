use std::fs;

fn main() {
    let filename = "data2.txt";
    println!("Part1: {}", part1(filename));
    // println!("Part2: {}", part2(filename));
}

#[derive(Debug)]
struct Present {
    grid: Vec<Vec<bool>>,
    rotated_shapes: Vec<Vec<Vec<bool>>>
}

impl Present {
    fn new(shape: Vec<Vec<bool>>) -> Present {
        let rot0 = Self::rotate(&shape, 0);
        let rot1 = Self::rotate(&shape, 1);
        let rot2 = Self::rotate(&shape, 2);
        let rot3 = Self::rotate(&shape, 3);
        Present {
            grid: shape,
            rotated_shapes: vec![rot0, rot1, rot2, rot3]
        }
    }

    fn area(self: &Present) -> u8 {
        self.grid.iter().flatten().filter(|&&x| x == true).count() as u8
    }

    fn rotate(shape: &Vec<Vec<bool>>, times: usize) -> Vec<Vec<bool>> {
        let mut result= shape.clone();
        for _ in 0..times  {
            let mut temp_result = vec![vec![false; 3]; 3];
            for x in 0..3 {
                for y in 0..3 {
                    temp_result[y][2 - x] = result[x][y];
                }
            }
            result = temp_result;
        }
        result
    }
}

struct Tree {
    x_size: u8,
    y_size: u8,
    presents: Vec<usize>,
}

struct Grid {
    x_size: u8,
    y_size: u8,
    grid: Vec<Vec<bool>>,
}

impl Grid {
    fn new(x_size: u8, y_size: u8) -> Grid {
        Grid {
            x_size,
            y_size,
            grid: vec![vec![false; x_size as usize]; y_size as usize] }
    }

    fn fit_present(mut self, present: Present, start_x: u8, start_y: u8, rotation: usize) -> Result<(), ()> {
        for y in start_y..start_y + 3 {
            for x in start_x..start_x + 3 {
                let grid = self.grid[y as usize][x as usize];
                if grid == true {
                    return Err(())
                }
                self.grid[y as usize][x as usize] = present.rotated_shapes[rotation][(y - start_y) as usize][(x - start_x) as usize];
            }
        }
        Ok(())
    }
}

fn part1(filename: &str) -> u64 {
    let input_as_string = fs::read_to_string(filename).unwrap();
    let split_input_string = input_as_string.split("\n\n").collect::<Vec<&str>>();
    let presents = split_input_string[..split_input_string.len() - 1]
        .iter()
        .map(|x| {
            x.split("\n")
                .skip(1)
                .map(|l| {
                    l.chars()
                        .map(|y| if y == '#' { true } else { false })
                        .collect::<Vec<bool>>()
                })
                .collect::<Vec<Vec<bool>>>()
        })
        .map(|x| Present::new(x))
        .collect::<Vec<Present>>();
    let trees = split_input_string
        .last()
        .unwrap()
        .lines()
        .map(|line| {
            let line_split = line.split(": ").collect::<Vec<&str>>();
            let size = line_split[0]
                .split("x")
                .map(|x| x.parse::<u8>().unwrap())
                .collect::<Vec<u8>>();
            let indices = line_split[1]
                .split(" ")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            Tree {
                x_size: size[0],
                y_size: size[1],
                presents: indices,
            }
        })
        .collect::<Vec<Tree>>();

    // println!("{:#?}", presents);

    let mut result = 0;
    for tree in trees {
        let min_area: u64 = presents.iter().enumerate().map(|(i, _)| 9 * (tree.presents[i] as u64)).sum();
        if min_area <= (tree.x_size as u64) * (tree.y_size as u64) {
            result += 1;
        }
    }
    result
}

// fn part2(filename: &str) -> u64 {}
