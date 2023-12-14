use std::{collections::HashMap, fs, time::Instant};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let solution_1 = part_1(&input);
    println!("[PART 1] :: {:?}", solution_1);
    println!("==============");
    println!();
    let solution_2 = part_2(&input);
    println!("[PART 2] :: {:?}", solution_2);
}

fn part_2(input: &str) -> usize {
    let matrix = to_matrix(input);

    let start = Instant::now();
    let matrix = cycle(&matrix);
    let duration = start.elapsed();
    println!("[DURATION] :: {:?}", duration);

    get_score(&matrix)
}

fn get_score(matrix: &Vec<Vec<char>>) -> usize {
    let scores = matrix
        .iter()
        .rev()
        .enumerate()
        .map(|(i, row)| row.into_iter().filter(|&c| c == &'O').count() * (i + 1))
        .collect::<Vec<usize>>();

    scores.into_iter().sum()
}

fn cycle(init: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut cache: HashMap<Vec<Vec<char>>, i64> = HashMap::new();
    let mut current: i64 = 0;
    let stop: i64 = 1_000_000_000 * 4;
    let mut matrix = init.clone();
    let directions = ['n', 'w', 's', 'e'];
    let mut skipped = false;
    loop {
        let direction = directions[(current as usize) % 4];
        if current >= stop {
            return matrix.clone();
        }
        match direction {
            'n' => {
                matrix = cycle_north(&matrix);
            }
            'w' => {
                matrix = cycle_west(&matrix);
            }
            's' => {
                matrix = cycle_south(&matrix);
            }
            'e' => {
                matrix = cycle_east(&matrix);
            }
            _ => unreachable!(),
        }
        // If found loop.
        if skipped {
            current = current + 1;
            continue;
        }
        if let Some(prev) = cache.get(&matrix) {
            let skip = current - prev;
            let max = (stop - current) / skip;
            current = current + skip * max;
            skipped = true;
        } else {
            cache.insert(matrix.clone(), current);
            current = current + 1;
        }
    }
}

// 1 2 3 4 5 6 7 1 2 3

fn cycle_north(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let matrix_t = transpose(&matrix);
    let pushed = matrix_t
        .iter()
        .map(|row| push_left(row))
        .collect::<Vec<_>>();
    transpose(&pushed)
}

fn cycle_west(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    matrix.iter().map(|row| push_left(row)).collect::<Vec<_>>()
}

fn cycle_south(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let matrix_t = transpose(&matrix);
    let pushed = matrix_t
        .iter()
        .map(|row| push_right(row))
        .collect::<Vec<_>>();
    transpose(&pushed)
}

fn cycle_east(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    matrix.iter().map(|row| push_right(row)).collect::<Vec<_>>()
}

fn push_left(row: &Vec<char>) -> Vec<char> {
    let mut open_i = 0;
    let mut pushed_row = row.clone();
    for (i, c) in row.into_iter().enumerate() {
        match c {
            '.' => continue,
            'O' => {
                pushed_row[i] = '.';
                pushed_row[open_i] = 'O';
                open_i = open_i + 1
            }
            '#' => open_i = i + 1,
            _ => unreachable!(),
        }
    }

    pushed_row
}

fn push_right(row: &Vec<char>) -> Vec<char> {
    let mut open_i = row.len() - 1;
    let mut pushed_row = row.clone();
    for (i, c) in row.into_iter().enumerate().rev() {
        match c {
            '.' => continue,
            'O' => {
                pushed_row[i] = '.';
                pushed_row[open_i] = 'O';
                open_i = if open_i == 0 { 0 } else { open_i - 1 }
            }
            '#' => open_i = if i == 0 { 0 } else { i - 1 },
            _ => unreachable!(),
        }
    }

    pushed_row
}

fn part_1(input: &str) -> usize {
    let matrix = to_matrix(input);
    let matrix = cycle_north(&matrix);
    let scores = transpose(&matrix)
        .iter()
        .map(|row| {
            score_row(
                &row.clone().into_iter().rev().collect::<Vec<_>>(),
                0,
                row.len(),
                row.len(),
            )
        })
        .collect::<Vec<usize>>();
    scores.into_iter().sum()
}

fn score_row(row: &Vec<char>, score: usize, points: usize, index: usize) -> usize {
    if row.is_empty() {
        return score;
    }

    let current_char = row.last().unwrap();

    match current_char {
        '.' => {
            let next_score = score;
            let next_points = points;
            let next_index = {
                if index == 0 {
                    0
                } else {
                    index - 1
                }
            };
            let next_row = &row[..next_index].to_vec();
            return score_row(next_row, next_score, next_points, next_index);
        }
        'O' => {
            let next_score = score + points;
            let next_points = points - 1;
            let next_index = {
                if index == 0 {
                    0
                } else {
                    index - 1
                }
            };
            let next_row = &row[..next_index].to_vec();
            return score_row(next_row, next_score, next_points, next_index);
        }
        '#' => {
            let next_score = score;
            let next_index = {
                if index == 0 {
                    0
                } else {
                    index - 1
                }
            };
            let next_points = next_index;
            let next_row = &row[..next_index].to_vec();
            return score_row(next_row, next_score, next_points, next_index);
        }
        _ => unreachable!(),
    }
}

fn to_matrix(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn transpose<T: Clone>(a: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..a[0].len())
        .map(|r| a.iter().map(|c| c[r].clone()).collect::<Vec<T>>())
        .collect()
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
