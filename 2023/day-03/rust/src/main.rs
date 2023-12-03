use std::{
    collections::HashSet,
    fs,
    hash::{Hash, Hasher},
};

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let file = fs::read_to_string("input_1.txt").unwrap();
    let lines: Vec<&str> = file.lines().collect();
    let matrix: Vec<Vec<Container>> = lines
        .iter()
        .enumerate()
        .map(|(i, line)| parse_line(i, line))
        .collect();
    let mut sum = 0;
    for (r_index, r) in matrix.iter().enumerate() {
        for (c_index, c) in r.iter().enumerate() {
            if matches!(c.kind, ContainerKind::Symbol) {
                println!("{:?}", "====");
                println!("{:?}", c.value);
                println!("{:?}", r.len());
                let points = find_adjacent_indexes(c_index as i32, r_index as i32);
                let numbers = points
                    .iter()
                    .filter_map(|point| {
                        let row = &matrix.get(point.y as usize);
                        if row.is_none() {
                            return None;
                        }
                        let r = row.unwrap();
                        let column = r.get(point.x as usize);
                        if column.is_none() {
                            return None;
                        }
                        println!("{:?}", column);
                        return column;
                    })
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .filter_map(|n| {
                        println!("{:?}", n);
                        let value = n.value.parse::<i32>().ok();
                        value
                    });

                let total: i32 = numbers.sum();
                sum = sum + total
            }
        }
    }
    println!("{:?}", sum)
}

fn part_2() {
    let file = fs::read_to_string("input_1.txt").unwrap();
    let lines: Vec<&str> = file.lines().collect();
    let matrix: Vec<Vec<Container>> = lines
        .iter()
        .enumerate()
        .map(|(i, line)| parse_line(i, line))
        .collect();
    let mut sum = 0;
    for (r_index, r) in matrix.iter().enumerate() {
        for (c_index, c) in r.iter().enumerate() {
            if matches!(c.kind, ContainerKind::Symbol) && c.value == "*" {
                let points = find_adjacent_indexes(c_index as i32, r_index as i32);
                let numbers: Vec<i32> = points
                    .iter()
                    .filter_map(|point| {
                        let row = &matrix.get(point.y as usize);
                        if row.is_none() {
                            return None;
                        }
                        let r = row.unwrap();
                        let column = r.get(point.x as usize);
                        if column.is_none() {
                            return None;
                        }
                        println!("{:?}", column);
                        return column;
                    })
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .filter_map(|n| {
                        println!("{:?}", n);
                        let value = n.value.parse::<i32>().ok();
                        value
                    })
                    .collect();

                if numbers.len() == 2 {
                    let total = numbers.iter().fold(1, |acc, n| n * acc);
                    sum = sum + total
                }
            }
        }
    }
    println!("{:?}", sum)
}

#[derive(Debug)]
struct Container {
    id: i32,
    kind: ContainerKind,
    value: String,
}

impl PartialEq for Container {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Container {}

impl Hash for Container {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug)]
enum ContainerKind {
    Number,
    Placeholder,
    Symbol,
}

fn parse_line(i: usize, line: &str) -> Vec<Container> {
    let mut result: Vec<Container> = vec![];
    let mut current_number = String::new();
    let mut iter = line.chars();
    let mut id = i as i32 * 1000;
    while let Some(c) = iter.next() {
        if is_int(&c.to_string()) {
            current_number.push(c);
            continue;
        }
        if current_number != String::from("") {
            id = id + 1;
            for _ in 0..current_number.len() {
                let container = Container {
                    id: id,
                    kind: ContainerKind::Number,
                    value: current_number.clone(),
                };
                result.push(container);
            }
            current_number = String::new();
        }
        if c.to_string() == ".".to_string() {
            let container = Container {
                id: 0,
                kind: ContainerKind::Placeholder,
                value: c.to_string(),
            };
            result.push(container);
            continue;
        }
        let container = Container {
            id: 0,
            kind: ContainerKind::Symbol,
            value: c.to_string(),
        };
        result.push(container);
    }

    if current_number != String::from("") {
        id = id + 1;
        for _ in 0..current_number.len() {
            let container = Container {
                id: id,
                kind: ContainerKind::Number,
                value: current_number.clone(),
            };
            result.push(container);
        }
    }

    return result;
}

fn is_int(s: &String) -> bool {
    let test = s.parse::<usize>();
    match test {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// x x x
// x x x
// x x x
fn find_adjacent_indexes(x: i32, y: i32) -> Vec<Point> {
    let top_left = Point { x: x - 1, y: y - 1 };
    let top = Point { x: x, y: y - 1 };
    let top_right = Point { x: x + 1, y: y - 1 };
    let middle_left = Point { x: x - 1, y: y };
    let middle_right = Point { x: x + 1, y: y };
    let bottom_left = Point { x: x - 1, y: y + 1 };
    let bottom = Point { x: x, y: y + 1 };
    let bottom_right = Point { x: x + 1, y: y + 1 };
    let coordinates = vec![
        top_left,
        top,
        top_right,
        middle_left,
        middle_right,
        bottom_left,
        bottom,
        bottom_right,
    ];
    coordinates
}
