use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("1.txt").unwrap();
    part_1(&file);
    part_2(&file);
}

#[derive(Debug)]
struct Node {
    value: String,
    left: String,
    right: String,
}

fn part_1(file: &String) {
    let lines = file.lines().collect::<Vec<&str>>();
    let directions = lines[0];
    let lookups = lines[2..]
        .iter()
        .map(|&line| parse_line(line))
        .collect::<Vec<Node>>();

    let mut indexed_lookups: HashMap<String, &Node> = HashMap::new();
    for lookup in lookups.iter() {
        indexed_lookups.insert(lookup.value.clone(), lookup);
    }
    let count = find_path_length("AAA", "ZZZ", directions, indexed_lookups);

    println!("{:?}", count);
}

fn find_path_length(
    start: &str,
    end: &str,
    directions: &str,
    indexed_lookups: HashMap<String, &Node>,
) -> usize {
    let mut current_value = start.to_string().clone();
    let search_value = end.to_string().clone();
    let mut count: usize = 0;
    while current_value != search_value {
        for direction in directions.chars() {
            match direction {
                'L' => {
                    let node = indexed_lookups.get(&current_value).unwrap();
                    let next_value = node.left.clone();
                    count = count + 1;
                    current_value = next_value;
                }
                'R' => {
                    let node = indexed_lookups.get(&current_value).unwrap();
                    let next_value = node.right.clone();
                    count = count + 1;
                    current_value = next_value;
                }
                _ => panic!(),
            }
        }
    }
    count
}

fn parse_line(line: &str) -> Node {
    let parts = line.split(" = (").collect::<Vec<&str>>();
    let value = parts[0];
    let lr = parts[1].split(", ").collect::<Vec<&str>>();
    let left = lr[0];
    let right = &lr[1][0..lr[1].len() - 1];
    Node {
        value: value.to_string(),
        left: left.to_string(),
        right: right.to_string(),
    }
}

fn part_2(file: &String) {
    let lines = file.lines().collect::<Vec<&str>>();
    let directions = lines[0];
    let lookups = lines[2..]
        .iter()
        .map(|&line| parse_line(line))
        .collect::<Vec<Node>>();

    let mut indexed_lookups: HashMap<String, &Node> = HashMap::new();
    for lookup in lookups.iter() {
        indexed_lookups.insert(lookup.value.clone(), lookup);
    }

    let counts = indexed_lookups
        .keys()
        .filter(|&key| key.chars().last().unwrap() == 'A')
        .map(|key| find_path_length_2(key, directions, &indexed_lookups))
        .collect::<Vec<usize>>();

    let count = lcmn(counts);

    println!("{:?}", count);
}

fn find_path_length_2(
    start: &str,
    directions: &str,
    indexed_lookups: &HashMap<String, &Node>,
) -> usize {
    let mut current_value = start.to_string().clone();
    let mut count: usize = 0;
    while current_value.chars().last().unwrap() != 'Z' {
        for direction in directions.chars() {
            match direction {
                'L' => {
                    let node = indexed_lookups.get(&current_value).unwrap();
                    let next_value = node.left.clone();
                    count = count + 1;
                    current_value = next_value;
                }
                'R' => {
                    let node = indexed_lookups.get(&current_value).unwrap();
                    let next_value = node.right.clone();
                    count = count + 1;
                    current_value = next_value;
                }
                _ => panic!(),
            }
        }
    }
    count
}

fn lcmn(xs: Vec<usize>) -> usize {
    xs.iter().fold(1, |a, &b| lcm(a, b))
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        if b < a {
            std::mem::swap(&mut b, &mut a);
        }
        b %= a;
    }
    a
}
