use std::{collections::HashMap, fs};
fn main() {
    let calibration_sum_1 = part_1();
    println!("Calibration Sum 1: {:?}", calibration_sum_1);
    let calibration_sum_2 = part_2();
    println!("Calibration Sum 2: {:?}", calibration_sum_2);
}

fn part_1() -> i32 {
    let lookups = HashMap::from([]);
    fs::read_to_string("input_part_1.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let numbers = parse(line.to_string(), &lookups);
            let max = numbers.len() - 1;
            let first_number = numbers.clone().into_iter().nth(0).unwrap();
            let last_number = numbers.into_iter().nth(max).unwrap();
            let calibration_value = first_number.to_string() + &last_number.to_string();
            calibration_value.parse::<i32>().unwrap()
        })
        .sum()
}

fn part_2() -> i32 {
    let lookups = HashMap::from([
        ("zero".to_string(), "0"),
        ("one".to_string(), "1"),
        ("two".to_string(), "2"),
        ("three".to_string(), "3"),
        ("four".to_string(), "4"),
        ("five".to_string(), "5"),
        ("six".to_string(), "6"),
        ("seven".to_string(), "7"),
        ("eight".to_string(), "8"),
        ("nine".to_string(), "9"),
    ]);
    fs::read_to_string("input_part_2.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let numbers = parse(line.to_string(), &lookups);
            let max = numbers.len() - 1;
            let first_number = numbers.clone().into_iter().nth(0).unwrap();
            let last_number = numbers.into_iter().nth(max).unwrap();
            let calibration_value = first_number.to_string() + &last_number.to_string();
            calibration_value.parse::<i32>().unwrap()
        })
        .sum()
}

fn parse(line: String, lookups: &HashMap<String, &str>) -> Vec<usize> {
    let chars: Vec<char> = line.chars().collect();
    let mut state: Vec<usize> = vec![];
    inner_parse(&chars, 0, 1, &mut state, lookups);
    return state;
}

fn inner_parse(
    chars: &Vec<char>,
    start: usize,
    current: usize,
    state: &mut Vec<usize>,
    lookups: &HashMap<String, &str>,
) -> () {
    let max = chars.len();
    if start > max {
        return;
    }
    if current > max {
        return inner_parse(chars, start + 1, start + 2, state, lookups);
    }
    let current_chars = &chars[start..current];
    let current_s: String = current_chars.iter().collect();
    let num = current_s.parse::<usize>();
    match num {
        Ok(int) => {
            state.push(int);
            inner_parse(chars, current, current + 1, state, lookups)
        }
        Err(_e) => {
            let lookup_result = lookups.get(&current_s);
            match lookup_result {
                Some(v) => {
                    state.push(v.parse::<usize>().unwrap());
                    inner_parse(chars, current - 1, current, state, lookups)
                }
                None => inner_parse(chars, start, current + 1, state, lookups),
            }
        }
    };
}
