use std::{cmp, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let part_1_solution = part_1(&input);
    println!("[Part 1] :: {}", part_1_solution);
    let part_2_solution = part_2(&input);
    println!("[Part 2] :: {}", part_2_solution);
}

#[derive(Debug, Copy, Clone)]
struct Coordinate {
    x: i64,
    y: i64,
}

#[derive(Debug, Copy, Clone)]
struct Tile {
    _kind: TileKind,
    location: Coordinate,
}

#[derive(Debug, Copy, Clone)]
enum TileKind {
    Space,
    Galaxy,
}

impl TileKind {
    fn from(c: char) -> Self {
        match c {
            '.' => TileKind::Space,
            '#' => TileKind::Galaxy,
            _ => panic!(),
        }
    }
    fn _value(&self) -> char {
        match self {
            TileKind::Space => '.',
            TileKind::Galaxy => '#',
        }
    }
}

fn part_1(input: &str) -> i64 {
    let map = parse_input(input);
    let map_t = transpose(&map);
    let expansion_factor = 2;
    let galaxies = get_galaxies(&map);
    let pairs = get_pairs(&galaxies);

    let empty_rows = get_empty_rows(&map);
    let empty_cols = get_empty_cols(&map_t);

    let distances = pairs.iter().map(|(g1, g2)| {
        let min_x = cmp::min(g1.location.x, g2.location.x);
        let max_x = cmp::max(g1.location.x, g2.location.x);
        let min_y = cmp::min(g1.location.y, g2.location.y);
        let max_y = cmp::max(g1.location.y, g2.location.y);

        let n_empty_rows = empty_rows
            .iter()
            .filter(|&row_index| row_index > &(min_y as usize) && row_index < &(max_y as usize))
            .count();

        let n_empty_cols = empty_cols
            .iter()
            .filter(|&col_index| col_index > &(min_x as usize) && col_index < &(max_x as usize))
            .count();

        distance(
            &g1.location,
            &g2.location,
            ((n_empty_cols) * (expansion_factor - 1))
                .try_into()
                .unwrap(),
            ((n_empty_rows) * (expansion_factor - 1))
                .try_into()
                .unwrap(),
        )
    });

    distances.sum()
}

fn part_2(input: &str) -> i64 {
    let map = parse_input(input);
    let map_t = transpose(&map);

    let expansion_factor = 1_000_000;

    let galaxies = get_galaxies(&map);
    let pairs = get_pairs(&galaxies);

    let empty_rows = get_empty_rows(&map);
    let empty_cols = get_empty_cols(&map_t);

    let distances = pairs.iter().map(|(g1, g2)| {
        let min_x = cmp::min(g1.location.x, g2.location.x);
        let max_x = cmp::max(g1.location.x, g2.location.x);
        let min_y = cmp::min(g1.location.y, g2.location.y);
        let max_y = cmp::max(g1.location.y, g2.location.y);

        let n_empty_rows = empty_rows
            .iter()
            .filter(|&row_index| row_index > &(min_y as usize) && row_index < &(max_y as usize))
            .count();

        let n_empty_cols = empty_cols
            .iter()
            .filter(|&col_index| col_index > &(min_x as usize) && col_index < &(max_x as usize))
            .count();

        distance(
            &g1.location,
            &g2.location,
            ((n_empty_cols) * (expansion_factor - 1))
                .try_into()
                .unwrap(),
            ((n_empty_rows) * (expansion_factor - 1))
                .try_into()
                .unwrap(),
        )
    });

    distances.sum()
}

fn parse_input(input: &str) -> Vec<Vec<TileKind>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| TileKind::from(c))
                .collect::<Vec<TileKind>>()
        })
        .collect::<Vec<Vec<TileKind>>>()
}

fn transpose(map: &Vec<Vec<TileKind>>) -> Vec<Vec<TileKind>> {
    (0..map[0].len())
        .map(|r| map.iter().map(|c| c[r].clone()).collect::<Vec<TileKind>>())
        .collect()
}

fn is_empty(row: &Vec<TileKind>) -> bool {
    row.iter().all(|i| match i {
        TileKind::Space => true,
        _ => false,
    })
}

fn distance(c1: &Coordinate, c2: &Coordinate, x_scale: i64, y_scale: i64) -> i64 {
    (c2.x - c1.x).abs() + (c2.y - c1.y).abs() + x_scale + y_scale
}

fn get_galaxies(map: &Vec<Vec<TileKind>>) -> Vec<Tile> {
    let mut galaxies = vec![];
    for (y, row) in map.iter().enumerate() {
        for (x, column) in row.iter().enumerate() {
            match column {
                TileKind::Galaxy => {
                    galaxies.push(Tile {
                        _kind: TileKind::Galaxy,
                        location: Coordinate {
                            x: x as i64,
                            y: y as i64,
                        },
                    });
                }
                _ => continue,
            }
        }
    }
    galaxies
}

fn get_pairs(galaxies: &Vec<Tile>) -> Vec<(Tile, Tile)> {
    let mut pairs = vec![];
    for (i, galaxy) in galaxies.iter().enumerate() {
        for j in (i + 1)..galaxies.len() {
            pairs.push((*galaxy, galaxies[j]))
        }
    }
    pairs
}

fn get_empty_cols(map_t: &Vec<Vec<TileKind>>) -> Vec<usize> {
    map_t
        .iter()
        .enumerate()
        .filter_map(|(col_index, col)| {
            if is_empty(col) {
                return Some(col_index);
            }
            None
        })
        .collect()
}

fn get_empty_rows(map: &Vec<Vec<TileKind>>) -> Vec<usize> {
    map.iter()
        .enumerate()
        .filter_map(|(row_index, row)| {
            if is_empty(row) {
                return Some(row_index);
            }
            None
        })
        .collect()
}

fn _pretty_print(map: &Vec<Vec<TileKind>>) {
    for row in map.iter() {
        for tile in row.iter() {
            print!("{}", tile._value());
        }
        println!();
    }
    println!();
}
