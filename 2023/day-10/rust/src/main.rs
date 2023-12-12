use std::{collections::HashSet, fs, ops, vec};

fn main() {
    let file = fs::read_to_string("1.txt").unwrap();
    // part_1(&file);
    part_2(&file);
}

#[derive(Debug)]
struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    fn is_empty(&mut self) -> bool {
        self.stack.len() == 0
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Node {
    valid_moves: Vec<Move>,
    value: String,
    location: Coordinate,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl ops::Add<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Move {
    North,
    South,
    East,
    West,
}

impl Move {
    fn value(&self) -> Coordinate {
        match *self {
            Move::North => Coordinate { x: 0, y: -1 },
            Move::East => Coordinate { x: 1, y: 0 },
            Move::South => Coordinate { x: 0, y: 1 },
            Move::West => Coordinate { x: -1, y: 0 },
        }
    }
}

fn part_1(file: &str) -> Vec<Node> {
    let rr = file.split("\n").collect::<Vec<&str>>();
    let m = rr
        .iter()
        .enumerate()
        .map(|(x, r)| {
            r.chars()
                .enumerate()
                .map(|(y, s)| parse_node(s, y as i32, x as i32))
                .collect::<Vec<Node>>()
        })
        .collect::<Vec<Vec<Node>>>();
    let start_coordinate: Coordinate = find_start(&m);
    let start_node = &m[start_coordinate.x as usize][start_coordinate.y as usize];

    println!("Start {:?}", start_node);
    let mut stack: Stack<Node> = Stack::new();
    let mut path: Vec<Node> = vec![];
    let mut visited: HashSet<Node> = HashSet::new();
    loop {
        let current_node = {
            if stack.is_empty() {
                start_node.clone()
            } else {
                (stack.pop().unwrap()).clone()
            }
        };
        if current_node == *start_node && path.len() > 1 {
            break;
        }
        if !visited.contains(&current_node) {
            path.push(current_node.clone());
        } else {
            continue;
        }
        visited.insert(current_node.clone());
        let children: Vec<Node> = get_adjacent_nodes(&m, &current_node);
        for child in children {
            if visited.contains(&child) {
                continue;
            }
            stack.push(child.clone());
        }
    }

    for p in path.iter() {
        println!("{:?}", p);
    }
    println!("{:?}", path.len() / 2);
    path
}

fn get_adjacent_nodes(matrix: &Vec<Vec<Node>>, current_node: &Node) -> Vec<Node> {
    let mut moves: Vec<Node> = vec![];

    for m in &current_node.valid_moves {
        let c = m.value() + current_node.location;
        if !is_valid_coordinate(c, matrix) {
            continue;
        }
        let new_node = &matrix[c.y as usize][c.x as usize];
        match m {
            Move::East => {
                if new_node.valid_moves.contains(&Move::West) {
                    moves.push(new_node.clone())
                }
                continue;
            }
            Move::North => {
                if new_node.valid_moves.contains(&Move::South) {
                    moves.push(new_node.clone())
                }
                continue;
            }
            Move::West => {
                if new_node.valid_moves.contains(&Move::East) {
                    moves.push(new_node.clone())
                }
                continue;
            }
            Move::South => {
                if new_node.valid_moves.contains(&Move::North) {
                    moves.push(new_node.clone())
                }
                continue;
            }
        }
    }
    moves
}

fn is_valid_coordinate(c: Coordinate, m: &Vec<Vec<Node>>) -> bool {
    c.x >= 0 && c.y >= 0 && c.x < m.len() as i32 && c.y < m.len() as i32
}

fn parse_node(s: char, x: i32, y: i32) -> Node {
    match s {
        '|' => Node {
            value: "|".to_string(),
            valid_moves: vec![Move::North, Move::South],
            location: Coordinate { x: x, y: y },
        },
        '-' => Node {
            value: "-".to_string(),
            valid_moves: vec![Move::East, Move::West],
            location: Coordinate { x: x, y: y },
        },
        'L' => Node {
            value: "L".to_string(),
            valid_moves: vec![Move::North, Move::East],
            location: Coordinate { x: x, y: y },
        },
        'J' => Node {
            value: "J".to_string(),
            valid_moves: vec![Move::North, Move::West],
            location: Coordinate { x: x, y: y },
        },
        '7' => Node {
            value: "7".to_string(),
            valid_moves: vec![Move::South, Move::West],
            location: Coordinate { x: x, y: y },
        },
        'F' => Node {
            value: "F".to_string(),
            valid_moves: vec![Move::South, Move::East],
            location: Coordinate { x: x, y: y },
        },
        '.' => Node {
            value: ".".to_string(),
            valid_moves: vec![],
            location: Coordinate { x: x, y: y },
        },
        'S' => Node {
            value: "S".to_string(),
            valid_moves: vec![Move::North, Move::East, Move::South, Move::West],
            location: Coordinate { x: x, y: y },
        },
        _ => panic!(),
    }
}

fn find_start(m: &Vec<Vec<Node>>) -> Coordinate {
    let mut start = Coordinate { x: 0, y: 0 };
    for x in 0..m.len() {
        for y in 0..m[x].len() {
            if m[x][y].value == "S".to_string() {
                start = Coordinate {
                    x: x as i32,
                    y: y as i32,
                }
            }
        }
    }
    start
}

fn part_2(file: &str) {
    let rr = file.split("\n").collect::<Vec<&str>>();
    let m = rr
        .iter()
        .enumerate()
        .map(|(x, r)| {
            r.chars()
                .enumerate()
                .map(|(y, s)| parse_node(s, y as i32, x as i32))
                .collect::<Vec<Node>>()
        })
        .collect::<Vec<Vec<Node>>>();
    let start_coordinate: Coordinate = find_start(&m);
    let start_node = &m[start_coordinate.x as usize][start_coordinate.y as usize];

    println!("Start {:?}", start_node);
    let mut stack: Stack<Node> = Stack::new();
    let mut path: Vec<Node> = vec![];
    let mut visited: HashSet<Node> = HashSet::new();
    loop {
        let current_node = {
            if stack.is_empty() {
                start_node.clone()
            } else {
                (stack.pop().unwrap()).clone()
            }
        };
        if current_node == *start_node && path.len() > 1 {
            break;
        }
        if !visited.contains(&current_node) {
            path.push(current_node.clone());
        } else {
            continue;
        }
        visited.insert(current_node.clone());
        let children: Vec<Node> = get_adjacent_nodes(&m, &current_node);
        for child in children {
            if visited.contains(&child) {
                continue;
            }
            stack.push(child.clone());
        }
    }

    for p in path.iter() {
        println!("{:?}", p);
    }
    println!("{:?}", path.len() / 2);

    let map = m.into_iter().map(|xs| {
        xs.into_iter().map(|x| match x {
            x if path.contains(&x) => x,
            x if !path.contains(&x) => Node {
                valid_moves: vec![],
                value: ".".to_string(),
                location: x.location,
            },
            _ => panic!(),
        })
    });

    let mut inside = false;
    let total = map
        .into_iter()
        .flatten()
        .filter(|i| match i.value.as_str() {
            "." => inside,
            "|" => {
                inside = !inside;
                false
            }
            "F" => {
                inside = !inside;
                false
            }
            "J" => {
                inside = !inside;
                false
            }
            _ => false,
        })
        .count();
    println!("{:?}", total)
}
