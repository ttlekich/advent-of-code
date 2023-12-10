use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("1.txt").unwrap();
    part_1(&file);
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

    let mut current_value = "AAA".to_string();
    let search_value = "ZZZ";
    let mut count = 0;
    while current_value != search_value {
        for direction in directions.chars() {
            println!("{:?}", indexed_lookups.get(&current_value).unwrap());
            println!("{:?}", direction);
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
    println!("{:?}", count);
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
