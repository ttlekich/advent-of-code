use std::{fmt::Display, fs, ops::Add, thread::current};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    part_1(&input);
    part_2(&input);
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    length: i64,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Direction {
    fn value(&self) -> Point {
        match self {
            Direction::Down => Point { x: 0, y: 1 },
            Direction::Up => Point { x: 0, y: -1 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1, y: 0 },
        }
    }
}

// impl Add for Direction {}

fn part_1(input: &str) -> () {
    let instructions = parse_instructions(&input);

    let mut current = Point { x: 0, y: 0 };
    let mut points = vec![current.clone()];
    for instruction in instructions {
        let current_points = get_points(instruction, current.clone());
        points.append(&mut current_points.clone());
        current = current_points.last().unwrap().clone();
    }

    let area = find_area(&points);

    let interior = (area + 1 as f64 - (0.5) * points.len() as f64).floor() as i64;

    println!("{}", points.len() as i64 + interior);
}

fn part_2(input: &str) -> () {
    let instructions = parse_instructions_2(&input);
    for instruction in &instructions {
        println!("{:?}", instruction);
    }

    let mut current = Point { x: 0, y: 0 };
    let mut points = vec![current.clone()];
    for instruction in instructions {
        let current_points = get_points(instruction, current.clone());
        points.append(&mut current_points.clone());
        current = current_points.last().unwrap().clone();
    }

    let area = find_area(&points);

    let interior = (area + 1 as f64 - (0.5) * points.len() as f64).floor() as i64;

    println!("{}", points.len() as i64 + interior);
}

fn find_area(points: &Vec<Point>) -> f64 {
    let mut xs = points.iter().map(|p| p.x).collect::<Vec<i64>>();
    let mut ys = points.iter().map(|p| p.y).collect::<Vec<i64>>();
    ys.rotate_left(1);

    let a = xs.iter().zip(ys.iter()).fold(0, |acc, (x, y)| acc + x * y);

    ys.rotate_right(1);
    xs.rotate_left(1);

    let b = xs.iter().zip(ys.iter()).fold(0, |acc, (x, y)| acc + x * y);

    0.5 * (a as f64 - b as f64)
}

fn get_points(instruction: Instruction, current: Point) -> Vec<Point> {
    match instruction.direction {
        Direction::Right => (1..=instruction.length)
            .map(|i| Point {
                x: current.x + i,
                y: current.y,
            })
            .collect::<Vec<Point>>(),
        Direction::Left => (1..=instruction.length)
            .map(|i| Point {
                x: current.x - i,
                y: current.y,
            })
            .collect::<Vec<Point>>(),
        Direction::Down => (1..=instruction.length)
            .map(|i| Point {
                x: current.x,
                y: current.y + i,
            })
            .collect::<Vec<Point>>(),
        Direction::Up => (1..=instruction.length)
            .map(|i| Point {
                x: current.x,
                y: current.y - i,
            })
            .collect::<Vec<Point>>(),
    }
}

fn parse_instructions_2(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|r| r[2])
        .map(|r| r.replace("(", ""))
        .map(|r| r.replace(")", ""))
        .map(|hex| hex.chars().collect::<Vec<char>>())
        .map(|cs| (cs[1..6].to_vec(), cs[6]))
        .map(|(length, c)| {
            let l = i64::from_str_radix(length.iter().collect::<String>().as_str(), 16).unwrap();
            match c {
                '0' => Instruction {
                    direction: Direction::Right,
                    length: l,
                },
                '2' => Instruction {
                    direction: Direction::Left,
                    length: l,
                },
                '3' => Instruction {
                    direction: Direction::Up,
                    length: l,
                },
                '1' => Instruction {
                    direction: Direction::Down,
                    length: l,
                },
                _ => unreachable!(),
            }
        })
        .collect()
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|r| match r[0] {
            "R" => Instruction {
                direction: Direction::Right,
                length: r[1].parse::<i64>().unwrap(),
            },
            "L" => Instruction {
                direction: Direction::Left,
                length: r[1].parse::<i64>().unwrap(),
            },
            "U" => Instruction {
                direction: Direction::Up,
                length: r[1].parse::<i64>().unwrap(),
            },
            "D" => Instruction {
                direction: Direction::Down,
                length: r[1].parse::<i64>().unwrap(),
            },
            _ => unreachable!(),
        })
        .collect::<Vec<Instruction>>()
}

/******* */

fn _pretty_print<T: Display>(a: &Vec<Vec<T>>) {
    for row in a.iter() {
        for column in row.iter() {
            print!("{} ", column);
        }
        println!();
    }
    println!();
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
