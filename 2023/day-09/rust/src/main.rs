use std::fs;

fn main() {
    let file = fs::read_to_string("1.txt").unwrap();
    part_1(&file);
    part_2(&file);
}

fn part_1(file: &str) {
    println!("*** PART 1 ***");
    let lines = file.lines().collect::<Vec<&str>>();
    let next_values = lines
        .iter()
        .map(|line| parse_line(line))
        .map(|report| get_next_value(report))
        .collect::<Vec<i64>>();

    let sum: i64 = next_values.iter().sum();

    println!("{:?}", sum);
}

fn part_2(file: &str) {
    println!("*** PART 2 ***");
    let lines = file.lines().collect::<Vec<&str>>();
    let next_values = lines
        .iter()
        .map(|line| parse_line(line))
        .map(|report| get_prev_value(report))
        .collect::<Vec<i64>>();

    let sum: i64 = next_values.iter().sum();

    println!("{:?}", sum);
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|raw| raw.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn get_next_value(report: Vec<i64>) -> i64 {
    let mut diffs: Vec<Vec<i64>> = vec![report];
    loop {
        let diff = get_diff(diffs.last().unwrap().clone());
        let is_last = diff.iter().all(|&d| d == 0);
        diffs.push(diff);
        if is_last {
            break;
        }
    }
    diffs
        .iter()
        .fold(0, |acc, value| acc + value.last().unwrap())
}

fn get_prev_value(report: Vec<i64>) -> i64 {
    let mut diffs: Vec<Vec<i64>> = vec![report];
    loop {
        let diff = get_diff(diffs.last().unwrap().clone());
        let is_last = diff.iter().all(|&d| d == 0);
        diffs.push(diff);
        if is_last {
            break;
        }
    }
    diffs
        .iter()
        .rev()
        .fold(0, |acc, value| value.first().unwrap() - acc)
}

fn get_diff(report: Vec<i64>) -> Vec<i64> {
    report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect::<Vec<i64>>()
}
