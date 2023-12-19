use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Display,
    fs,
    ops::Add,
};

fn main() {
    let input = fs::read_to_string("test.txt").unwrap();
    let one = part_1(&input);
    println!("{:?}", one);
    let two = part_2(&input);
    println!("{:?}", two);
}

struct Data {
    cost: i64,
    count: i64,
    parent: Move,
}

fn part_1(input: &str) -> () {
    let mut board = parse_tiles(&input);
    let start = Move {
        direction: Direction::Right,
        vector: Node { x: 0, y: 0 },
        weight: board[0][0],
    };
    let mut shortest: HashMap<Move, Data> = HashMap::new();
    let mut visited: HashSet<Move> = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0, 1 as i64));
    loop {
        if queue.is_empty() {
            break;
        }
        let (m, w, c) = queue.pop_front().unwrap();
        if let Some(_) = visited.get(&m) {
            continue;
        }
        println!("n {:?}", c);
        visited.insert(m.clone());
        for next_m in adj_moves(&board, &m, c) {
            let next_w = next_m.weight + w;
            if let Some(existing_data) = shortest.get(&next_m.clone()) {
                if next_w >= existing_data.cost {
                    continue;
                }
            }
            shortest.insert(next_m.clone());
            parents.insert(next_m.clone(), m.clone());
            if next_m.direction == m.direction {
                queue.push_back((next_m.clone(), next_w, c + 1));
            } else {
                queue.push_back((next_m.clone(), next_w, 1));
            }
        }
    }

    let mut p: Move;
    for (k, v) in shortest.iter() {
        if k.vector.x == (board.len() - 1) as i64 && k.vector.y == (board.len() - 1) as i64 {
            let mut new_board = board.clone();
            println!("weight {:?}", v);
            p = k.clone();
            while let Some(new_p) = parents.get(&p) {
                new_board[p.vector.y as usize][p.vector.x as usize] = 0;
                p = new_p.clone();
            }
            _pretty_print(&new_board);
        }
    }

    // println!("{:?}", shortest);
    // _pretty_print(&board);
}

fn adj_moves(board: &Board, m: &Move, c: i64) -> Vec<Move> {
    let mut result = vec![];
    let max = board.len() as i64;
    let directions = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
    for direction in directions {
        let v = m.vector + direction.value();
        if v.x >= 0 && v.x < max && v.y >= 0 && v.y < max {
            let next = Move {
                direction: direction,
                vector: v,
                weight: board[v.y as usize][v.x as usize],
            };
            result.push(next);
        }
    }
    if !(c < 3) {
        result = result
            .into_iter()
            .filter(|r| r.direction != m.direction)
            .collect::<Vec<Move>>()
    }
    result
}

fn part_2(input: &str) -> () {
    todo!()
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn value(&self) -> Node {
        match *self {
            Direction::Right => Node { x: 1, y: 0 },
            Direction::Left => Node { x: -1, y: 0 },
            Direction::Up => Node { x: 0, y: -1 },
            Direction::Down => Node { x: 0, y: 1 },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    x: i64,
    y: i64,
}

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Move {
    vector: Node,
    direction: Direction,
    weight: i64,
}

fn _pretty_print<T: Display>(a: &Vec<Vec<T>>) {
    for row in a.iter() {
        for column in row.iter() {
            print!("{} ", column);
        }
        println!();
    }
    println!();
}
