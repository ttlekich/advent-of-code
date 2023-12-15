use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let output_1 = part_1(&input);
    println!("{:?}", output_1);
}

fn part_1(input: &str) -> i64 {
    let steps = input.split(",");
    steps.map(|step| hash(step)).sum()
}

fn hash(step: &str) -> i64 {
    let chars = step.chars();
    let constant = 17;
    let mut current = 0;

    for c in chars {
        let a = c as i64;
        current = current + a;
        current = current * constant;
        current = current % 256;
    }

    current
}

#[derive(Debug)]
enum Op {
    Equal,
    Minus,
}

#[derive(Debug)]
struct Step {
    label: String,
    hash: i64,
    op: Op,
    focal_length: i64,
}

fn part_2(input: &str) -> i64 {
    let steps = parse_steps(&input);
    let mut boxes: HashMap<i64, Vec<Step>> = HashMap::new();
    for step in steps.iter() {
        match step.op {
            Op::Equal => {
                if let Some(values) = boxes.get(&step.hash) {
                    // get new values;
                    boxes.insert(step.hash, v);
                }
            }
            Op::Minus => {}
        }
    }

    todo!()
}

fn parse_steps(input: &str) -> Vec<Step> {
    input
        .split(",")
        .map(|s| match s {
            x if s.contains('=') => {
                let parts = x.split("=").collect::<Vec<&str>>();
                Step {
                    label: parts[0].to_string(),
                    op: Op::Equal,
                    focal_length: parts[2].parse::<i64>().unwrap(),
                    hash: hash(parts[0]),
                }
            }
            x if s.contains('-') => {
                let parts = x.split("=").collect::<Vec<&str>>();
                Step {
                    label: parts[0].to_string(),
                    op: Op::Equal,
                    focal_length: parts[2].parse::<i64>().unwrap(),
                    hash: hash(parts[0]),
                }
            }
            _ => unreachable!(),
        })
        .collect::<Vec<Step>>()
}
