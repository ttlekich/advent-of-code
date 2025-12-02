use std::fs;

pub fn part01() {
    let result = fs::read_to_string("./input/day02.txt")
        .unwrap()
        .split(',')
        .fold(0, |total: i64, range| {
            let numbers = parse_range(range);
            let sum: i64 = numbers.iter().filter(|&&number| is_mirror(number)).sum();

            total + sum
        });
    println!("{}", result);
}

pub fn part02() {
    let result = fs::read_to_string("./input/day02.txt")
        .unwrap()
        .split(',')
        .fold(0, |total: i64, range| {
            let numbers = parse_range(range);
            let sum: i64 = numbers
                .iter()
                .filter(|&&number| is_mirror(number) || has_repeats(number))
                .sum();

            total + sum
        });
    println!("{}", result);
}

fn has_repeats(number: i64) -> bool {}

fn is_mirror(number: i64) -> bool {
    let number_string = number.to_string();
    if number_string.len() % 2 != 0 {
        return false;
    }

    let char_count = number_string.chars().count();
    let half_count = char_count / 2;

    let first_half: String = number_string.chars().take(half_count).collect();
    let last_half: String = number_string.chars().skip(half_count).collect();

    return first_half == last_half;
}

fn parse_range(range: &str) -> Vec<i64> {
    let mut parts = range.split("-");
    let low: i64 = parts.next().unwrap().parse().unwrap();
    let high: i64 = parts.next().unwrap().parse().unwrap();

    let numbers: Vec<i64> = (low..=high).collect();
    numbers
}
