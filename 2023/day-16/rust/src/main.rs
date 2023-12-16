use core::borrow;
use std::{fs, ops::Add};

fn main() {
    let input = fs::read_to_string("test.txt").unwrap();
    let one = part_1(&input);
    println!("{:?}", one);
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
struct Move {
    vector: Vector,
    direction: Direction,
}

fn part_1(input: &str) -> () {
    let mut board = parse_tiles(&input);
    let mut stack: Vec<Move> = vec![Move {
        vector: Vector { x: 0, y: 0 },
        direction: Direction::Right,
    }];

    // while !stack.is_empty() {
    //     tick(&mut board, &mut stack, &board);
    // }
    for _ in 0..10000 {
        tick(&mut board, &mut stack);
    }

    _pretty_print(&board);
    _pretty_print(&board);
    println!("{:?}", stack);

    todo!()
}

fn tick(board: &mut Board, stack: &mut Vec<Move>) -> () {
    if let Some(current_move) = stack.pop() {
        if current_move.vector.y >= board.len() as i64
            || current_move.vector.x >= board[0].len() as i64
            || current_move.vector.y < 0
            || current_move.vector.x < 0
        {
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
            // '#' => stack.push(Move {
            //     vector: current_move.vector + current_move.direction.value(),
            //     direction: current_move.direction,
            // }),
            '/' => {
                board[current_move.vector.y as usize][current_move.vector.x as usize] = '#';
                match current_move.direction {
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
                }
            }
            '\\' => {
                board[current_move.vector.y as usize][current_move.vector.x as usize] = '#';
                match current_move.direction {
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
                }
            }
            '-' => {
                board[current_move.vector.y as usize][current_move.vector.x as usize] = '#';
                match current_move.direction {
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
                }
            }
            '|' => {
                board[current_move.vector.y as usize][current_move.vector.x as usize] = '#';
                match current_move.direction {
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
                            direction: Direction::Left,
                        });
                    }
                    Direction::Down => {
                        stack.push(Move {
                            vector: current_move.vector + Direction::Down.value(),
                            direction: Direction::Down,
                        });
                    }
                }
            }
            _ => unreachable!(),
        }
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
