use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let output_1 = part_1(&input);
    println!("[PART 1] :: {:?}", output_1);
    let output_2 = part_2(&input);
    println!("[PART 2] :: {:?}", output_2);
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

#[derive(Debug, Clone)]
enum Op {
    Equal,
    Minus,
}

#[derive(Debug, Clone)]
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
            Op::Minus => {
                if let Some(values) = boxes.get(&step.hash) {
                    let mut new_values = values.to_vec();
                    new_values.retain(|x| x.label != step.label);
                    boxes.insert(step.hash, new_values);
                }
            }
            Op::Equal => {
                if let Some(values) = boxes.get(&step.hash) {
                    let mut new_values = values.to_vec();
                    let index_result = new_values.iter().position(|v| v.label == step.label);
                    if let Some(index) = index_result {
                        new_values[index] = step.clone();
                        boxes.insert(step.hash, new_values);
                    } else {
                        let mut new_values = values.to_vec();
                        new_values.push(step.clone());
                        boxes.insert(step.hash, new_values);
                    }
                } else {
                    boxes.insert(step.hash, vec![step.clone()]);
                }
            }
        }
    }
    let sum = boxes
        .keys()
        .map(|k| {
            let vs = boxes.get(k).unwrap();
            vs.iter()
                .enumerate()
                .map(|(slot_n, v)| calculate_focusing_power(v, k.clone(), (slot_n + 1) as i64))
                .sum::<i64>()
        })
        .sum::<i64>();

    sum
}

fn calculate_focusing_power(v: &Step, k: i64, slot_n: i64) -> i64 {
    (k + 1) * slot_n * v.focal_length
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
                    focal_length: parts[1].parse::<i64>().unwrap(),
                    hash: hash(parts[0]),
                }
            }
            x if s.contains('-') => {
                let parts = x.split("-").collect::<Vec<&str>>();
                Step {
                    label: parts[0].to_string(),
                    op: Op::Minus,
                    focal_length: if parts[1] != "" {
                        parts[1].parse::<i64>().unwrap()
                    } else {
                        0
                    },
                    hash: hash(parts[0]),
                }
            }
            _ => unreachable!(),
        })
        .collect::<Vec<Step>>()
}
