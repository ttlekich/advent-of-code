use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("test.txt").unwrap();
    part_1(&input);
}

fn part_1(input: &str) -> () {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let map = parse_map(&parts[0]);
    // let map = parse_map(&parts[0]);
}

fn parse_map(input: &str) -> () {
    let mut result = HashMap::new();

    for line in input.lines() {
        let parts = line.split("{").collect::<Vec<_>>();
        let name = *parts.first().unwrap();
        let rules = parse_rules(*parts.last().unwrap());
        result.insert(name, rules.clone());
    }

    // println!("{:?}", result);

    todo!()
}

fn parse_rules(input: &str) -> () {
    let chars = input.chars();

    let mut current = String::new();

    for c in chars {}
}

enum Op {
    Less,
    Greater,
}
