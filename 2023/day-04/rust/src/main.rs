use std::fs;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let file = fs::read_to_string("1.txt").unwrap();
    let lines = file.lines();
    let cards: Vec<&str> = lines
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            parts[1]
        })
        .collect();
    let sum: i32 = cards
        .iter()
        .map(|&card| {
            let parts: Vec<&str> = card.split("| ").collect();
            parts
        })
        .map(|card| {
            let winning: Vec<i32> = card[0].split_whitespace().map(|n| to_int(n)).collect();
            let my: Vec<i32> = card[1].split_whitespace().map(|n| to_int(n)).collect();
            return vec![winning, my];
        })
        .map(|card| {
            let mut score = 0;
            let winning = &card[0];
            let my = &card[1];
            for n in my.iter() {
                if winning.contains(n) {
                    if score == 0 {
                        score = 1;
                        continue;
                    }
                    score = score * 2
                }
            }
            score
        })
        .sum();
    println!("{:?}", sum);
}

fn part_2() {
    let file = fs::read_to_string("1.txt").unwrap();
    let lines = file.lines();
    let cards: Vec<&str> = lines
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            parts[1]
        })
        .collect();
    let mut adds: Vec<(i32, i32)> = cards
        .iter()
        .map(|&card| {
            let parts: Vec<&str> = card.split("| ").collect();
            parts
        })
        .map(|card| {
            let winning: Vec<i32> = card[0].split_whitespace().map(|n| to_int(n)).collect();
            let my: Vec<i32> = card[1].split_whitespace().map(|n| to_int(n)).collect();
            return vec![winning, my];
        })
        .map(|card| {
            let mut matches = 0;
            let winning = &card[0];
            let my = &card[1];
            for n in my.iter() {
                if winning.contains(n) {
                    matches = matches + 1
                }
            }
            (1, matches)
        })
        .collect();
    for i in 0..adds.len() {
        let current = adds[i];
        for j in 1..=current.1 {
            let next = adds[i + j as usize];
            adds[i + j as usize] = (next.0 + current.0, next.1)
        }
    }
    let sum: i32 = adds.iter().map(|t| t.0).sum();
    println!("{}", sum)
}

/******************************************************************************/

fn to_int(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}
