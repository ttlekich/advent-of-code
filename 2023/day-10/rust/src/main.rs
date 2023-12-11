use std::{fs, ops, vec};

fn main() {
    let file = fs::read_to_string("1.txt").unwrap();
    part_1(&file);
}

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

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[derive(Debug, Clone)]
struct Node {
    valid_moves: Vec<Move>,
    value: String,
    visited: bool,
    location: Coordinate,
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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

fn part_1(file: &str) {
    let rr = file.split("\n").collect::<Vec<&str>>();
    let m = rr
        .iter()
        .enumerate()
        .map(|(x, r)| {
            r.chars()
                .enumerate()
                .map(|(y, s)| parse_node(s, x as i32, y as i32))
                .collect::<Vec<Node>>()
        })
        .collect::<Vec<Vec<Node>>>();
    let start_coordinate: Coordinate = find_start(&m);
    let start_node = &m[start_coordinate.x as usize][start_coordinate.y as usize];
    println!("Start {:?}", start_node);
    let mut stack: Stack<Node> = Stack::new();
    let initial_nodes: Vec<Node> = get_adjacent_nodes(&m, start_node);
    for node in initial_nodes {
        stack.push(node);
    }
    loop {
        let mut current_node = stack.pop().unwrap();
        println!("Current {:?}", current_node);
        if current_node.value == start_node.value {
            break;
        }
        current_node.visited = true;
        let adjacent_nodes: Vec<Node> = get_adjacent_nodes(&m, &current_node);
        for node in adjacent_nodes {
            stack.push(node);
        }
    }
}

fn get_adjacent_nodes(matrix: &Vec<Vec<Node>>, current_node: &Node) -> Vec<Node> {
    current_node
        .valid_moves
        .iter()
        .map(|m| m.value() + current_node.location)
        .map(|c| (&matrix[c.x as usize][c.y as usize]).clone())
        .filter(|n| !n.visited)
        .collect::<Vec<Node>>()
}

fn parse_node(s: char, x: i32, y: i32) -> Node {
    match s {
        '|' => Node {
            value: "|".to_string(),
            valid_moves: vec![Move::North, Move::South],
            visited: false,
            location: Coordinate { x: x, y: y },
        },
        '-' => Node {
            value: "-".to_string(),
            valid_moves: vec![Move::East, Move::West],
            visited: false,
            location: Coordinate { x: x, y: y },
        },
        'L' => Node {
            value: "L".to_string(),
            valid_moves: vec![Move::North, Move::East],
            visited: false,
            location: Coordinate { x: x, y: y },
        },
        'J' => Node {
            value: "J".to_string(),
            valid_moves: vec![Move::North, Move::West],
            visited: false,
            location: Coordinate { x: x, y: y },
        },
        '7' => Node {
            value: "7".to_string(),
            valid_moves: vec![Move::South, Move::West],
            visited: false,
            location: Coordinate { x: x, y: y },
        },
        'F' => Node {
            value: "F".to_string(),
            valid_moves: vec![Move::South, Move::East],
            visited: false,
            location: Coordinate { x: x, y: y },
        },
        '.' => Node {
            value: ".".to_string(),
            valid_moves: vec![],
            visited: false,
            location: Coordinate { x: x, y: y },
        },
        'S' => Node {
            value: "S".to_string(),
            valid_moves: vec![Move::North, Move::East, Move::South, Move::West],
            visited: false,
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
