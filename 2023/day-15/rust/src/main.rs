use std::fs;

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
