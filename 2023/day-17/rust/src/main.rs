use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
    fmt::Display,
    fs,
    ops::Add,
};

#[derive(Debug, Clone)]
struct Data {
    node: Node,
    cost: Cost,
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        ((self.cost, &self.node)).cmp(&(other.cost, &other.node))
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        (&self.node, self.cost) == (&other.node, other.cost)
    }
}

impl Eq for Data {}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let one = part_1(&input);
    println!("{:?}", one);
    let two = part_2(&input);
    println!("{:?}", two);
}

fn part_1(input: &str) -> () {
    let board = parse_tiles(&input);
    let source = Node { x: 0, y: 0 };
    let mut visited = HashSet::new();
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, source, Direction::None, 0)));

    while !pq.is_empty() {
        let Reverse((hl, n1, direction, direction_count)) = pq.pop().unwrap();

        if n1.x == (board[0].len() - 1) as i64 && n1.y == (board.len() - 1) as i64 {
            println!("{}", hl);
            break;
        }

        if visited.contains(&(n1, direction, direction_count)) {
            continue;
        }
        visited.insert((n1, direction, direction_count));

        if direction_count < 3 && direction != Direction::None {
            let n2 = n1 + direction.value();
            if n2.x < board[0].len() as i64 && n2.x >= 0 && n2.y < board.len() as i64 && n2.y >= 0 {
                pq.push(Reverse((
                    hl + board[n2.y as usize][n2.x as usize],
                    n2,
                    direction,
                    direction_count + 1,
                )));
            }
        }

        for next_direction in vec![
            Direction::Right,
            Direction::Left,
            Direction::Up,
            Direction::Down,
        ]
        .iter()
        {
            if direction != *next_direction && direction.invert() != *next_direction {
                let n2 = n1 + next_direction.value();
                if n2.x < board[0].len() as i64
                    && n2.x >= 0
                    && n2.y < board.len() as i64
                    && n2.y >= 0
                {
                    pq.push(Reverse((
                        hl + board[n2.y as usize][n2.x as usize],
                        n2,
                        *next_direction,
                        1,
                    )));
                }
            }
        }
    }
}

fn part_2(input: &str) -> () {
    println!("======");
    let board = parse_tiles(&input);
    let source = Node { x: 0, y: 0 };
    let mut visited = HashSet::new();
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, source, Direction::None, 0)));

    while !pq.is_empty() {
        let Reverse((hl, n1, direction, direction_count)) = pq.pop().unwrap();

        if n1.x == (board[0].len() - 1) as i64
            && n1.y == (board.len() - 1) as i64
            && direction_count >= 4
        {
            println!("{}", hl);
            break;
        }

        if visited.contains(&(n1, direction, direction_count)) {
            continue;
        }
        visited.insert((n1, direction, direction_count));

        if direction_count < 10 && direction != Direction::None {
            let n2 = n1 + direction.value();
            if n2.x < board[0].len() as i64 && n2.x >= 0 && n2.y < board.len() as i64 && n2.y >= 0 {
                pq.push(Reverse((
                    hl + board[n2.y as usize][n2.x as usize],
                    n2,
                    direction,
                    direction_count + 1,
                )));
            }
        }

        if direction_count >= 4 || direction == Direction::None {
            for next_direction in vec![
                Direction::Right,
                Direction::Left,
                Direction::Up,
                Direction::Down,
            ]
            .iter()
            {
                if direction != *next_direction && direction.invert() != *next_direction {
                    let n2 = n1 + next_direction.value();
                    if n2.x < board[0].len() as i64
                        && n2.x >= 0
                        && n2.y < board.len() as i64
                        && n2.y >= 0
                    {
                        pq.push(Reverse((
                            hl + board[n2.y as usize][n2.x as usize],
                            n2,
                            *next_direction,
                            1,
                        )));
                    }
                }
            }
        }
    }
}

fn parse_tiles(input: &str) -> Board {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
    None,
}

impl Direction {
    fn value(&self) -> Node {
        match *self {
            Direction::Right => Node { x: 1, y: 0 },
            Direction::Left => Node { x: -1, y: 0 },
            Direction::Up => Node { x: 0, y: -1 },
            Direction::Down => Node { x: 0, y: 1 },
            Direction::None => Node { x: 0, y: 0 },
        }
    }
    fn invert(&self) -> Direction {
        match *self {
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::None => Direction::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Node {
    x: i64,
    y: i64,
}

type Weight = i64;
type Cost = i64;

impl Add<Node> for Node {
    type Output = Node;

    fn add(self, other: Node) -> Node {
        Node {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

type Board = Vec<Vec<i64>>;

fn _pretty_print<T: Display>(a: &Vec<Vec<T>>) {
    for row in a.iter() {
        for column in row.iter() {
            print!("{} ", column);
        }
        println!();
    }
    println!();
}
