use std::{collections::HashSet, fs, ops::Add};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let one = part_1(&input);
    println!("{:?}", one);
    let two = part_2(&input);
    println!("{:?}", two);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn value(&self) -> Vector {
        match *self {
            Direction::Right => Vector { x: 1, y: 0 },
            Direction::Left => Vector { x: -1, y: 0 },
            Direction::Up => Vector { x: 0, y: -1 },
            Direction::Down => Vector { x: 0, y: 1 },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector {
    x: i64,
    y: i64,
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

type Board = Vec<Vec<char>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Move {
    vector: Vector,
    direction: Direction,
}

fn part_2(input: &str) -> usize {
    let board = parse_tiles(&input);
    let max = (board.len() - 1) as i64;
    let initial_moves = (0..board.len()).flat_map(|i| {
        vec![
            Move {
                vector: Vector { x: i as i64, y: 0 },
                direction: Direction::Down,
            },
            Move {
                vector: Vector {
                    x: i as i64,
                    y: max,
                },
                direction: Direction::Up,
            },
            Move {
                vector: Vector {
                    x: 0 as i64,
                    y: i as i64,
                },
                direction: Direction::Right,
            },
            Move {
                vector: Vector {
                    x: max as i64,
                    y: i as i64,
                },
                direction: Direction::Left,
            },
        ]
    });

    let solutions = initial_moves.map(|m| solve(&board, m));

    solutions.max().unwrap()
}

fn part_1(input: &str) -> usize {
    let board = parse_tiles(&input);
    let initial = Move {
        vector: Vector { x: 0, y: 0 },
        direction: Direction::Right,
    };

    solve(&board, initial.clone())
}

fn solve(board: &Board, initial: Move) -> usize {
    let mut current_board = board.clone();
    let mut stack: Vec<Move> = vec![initial];
    let mut visited: HashSet<Move> = HashSet::new();
    let mut energized: HashSet<Vector> = HashSet::new();

    while !stack.is_empty() {
        tick(&mut current_board, &mut stack, &mut visited);
    }

    for v in visited.iter() {
        energized.insert(v.vector);
    }

    energized.len()
}

fn tick(board: &mut Board, stack: &mut Vec<Move>, visited: &mut HashSet<Move>) -> () {
    if let Some(current_move) = stack.pop() {
        if current_move.vector.y >= board.len() as i64
            || current_move.vector.x >= board[0].len() as i64
            || current_move.vector.y < 0
            || current_move.vector.x < 0
        {
            return;
        }
        if visited.contains(&current_move) {
            return;
        }
        let current_tile = board[current_move.vector.y as usize][current_move.vector.x as usize];
        match current_tile {
            '.' => {
                stack.push(Move {
                    vector: current_move.vector + current_move.direction.value(),
                    direction: current_move.direction,
                });
                board[current_move.vector.y as usize][current_move.vector.x as usize] = '#';
            }
            '#' => stack.push(Move {
                vector: current_move.vector + current_move.direction.value(),
                direction: current_move.direction,
            }),
            '/' => match current_move.direction {
                Direction::Right => {
                    stack.push(Move {
                        vector: current_move.vector + Direction::Up.value(),
                        direction: Direction::Up,
                    });
                }
                Direction::Left => {
                    stack.push(Move {
                        vector: current_move.vector + Direction::Down.value(),
                        direction: Direction::Down,
                    });
                }
                Direction::Up => {
                    stack.push(Move {
                        vector: current_move.vector + Direction::Right.value(),
                        direction: Direction::Right,
                    });
                }
                Direction::Down => {
                    stack.push(Move {
                        vector: current_move.vector + Direction::Left.value(),
                        direction: Direction::Left,
                    });
                }
            },
            '\\' => match current_move.direction {
                Direction::Right => {
                    stack.push(Move {
                        vector: current_move.vector + Direction::Down.value(),
                        direction: Direction::Down,
                    });
                }
                Direction::Left => {
                    stack.push(Move {
                        vector: current_move.vector + Direction::Up.value(),
                        direction: Direction::Up,
                    });
                }
                Direction::Up => {
                    stack.push(Move {
                        vector: current_move.vector + Direction::Left.value(),
                        direction: Direction::Left,
                    });
                }
                Direction::Down => {
                    stack.push(Move {
                        vector: current_move.vector + Direction::Right.value(),
                        direction: Direction::Right,
                    });
                }
            },
            '-' => match current_move.direction {
                Direction::Right => {
                    stack.push(Move {
                        vector: current_move.vector + Direction::Right.value(),
                        direction: Direction::Right,
                    });
                }
                Direction::Left => {
                    stack.push(Move {
                        vector: current_move.vector + Direction::Left.value(),
                        direction: Direction::Left,
                    });
                }
                Direction::Up => {
                    stack.push(Move {
                        vector: current_move.vector + Direction::Left.value(),
                        direction: Direction::Left,
                    });
                    stack.push(Move {
                        vector: current_move.vector + Direction::Right.value(),
                        direction: Direction::Right,
                    });
                }
                Direction::Down => {
                    stack.push(Move {
                        vector: current_move.vector + Direction::Left.value(),
                        direction: Direction::Left,
                    });
                    stack.push(Move {
                        vector: current_move.vector + Direction::Right.value(),
                        direction: Direction::Right,
                    });
                }
            },
            '|' => match current_move.direction {
                Direction::Right => {
                    stack.push(Move {
                        vector: current_move.vector + Direction::Up.value(),
                        direction: Direction::Up,
                    });
                    stack.push(Move {
                        vector: current_move.vector + Direction::Down.value(),
                        direction: Direction::Down,
                    });
                }
                Direction::Left => {
                    stack.push(Move {
                        vector: current_move.vector + Direction::Up.value(),
                        direction: Direction::Up,
                    });
                    stack.push(Move {
                        vector: current_move.vector + Direction::Down.value(),
                        direction: Direction::Down,
                    });
                }
                Direction::Up => {
                    stack.push(Move {
                        vector: current_move.vector + Direction::Up.value(),
                        direction: Direction::Up,
                    });
                }
                Direction::Down => {
                    stack.push(Move {
                        vector: current_move.vector + Direction::Down.value(),
                        direction: Direction::Down,
                    });
                }
            },
            _ => unreachable!(),
        }
        visited.insert(current_move.clone());
    }
}

fn parse_tiles(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn _pretty_print(a: &Vec<Vec<char>>) {
    for row in a.iter() {
        for column in row.iter() {
            print!("{} ", column);
        }
        println!();
    }
    println!();
}
