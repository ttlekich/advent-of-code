use std::{collections::HashMap, fs};

fn main() {
    part_1();
}

fn part_1() {
    let limits: HashMap<String, usize> = HashMap::from([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]);
    let sum: usize = fs::read_to_string("part_1_input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let game_id = parse_game_id(line);
            println!("====={:?}======", game_id);
            let sets = parse_game_sets(line);
            for set in sets.iter() {
                let plays: Vec<&str> = set.trim().split(",").collect();
                let mut scores: HashMap<String, usize> = HashMap::new();
                for play in plays.iter() {
                    let parts: Vec<&str> = play.trim().split(" ").collect();
                    println!("{:?}", parts);
                    let count = parts[0].parse::<usize>().unwrap();
                    let color = parts[1];

                    let existing_score = scores.get(&String::from(color));
                    match existing_score {
                        Some(s) => scores.insert(String::from(color), count + s),
                        None => scores.insert(String::from(color), count),
                    };
                    println!("{:?}", scores);
                }
                println!("{:?}", scores);
                println!("{:?}", limits);
                let is_possible = check_is_possible(&scores, &limits);
                println!("{:?}", is_possible);
                if !is_possible {
                    return None;
                }
            }
            let game_id_int = game_id.parse::<usize>().unwrap();
            return Some(game_id_int);
        })
        .flatten()
        .sum();
    println!("{:?}", sum);
}

fn parse_game_id(line: &str) -> &str {
    let parts: Vec<&str> = line.split(":").collect();
    let game_label = parts[0];
    let game_label_parts: Vec<&str> = game_label.split(" ").collect();
    let game_id = game_label_parts[1];
    game_id
}

fn parse_game_sets(line: &str) -> Vec<&str> {
    let parts: Vec<&str> = line.split(":").collect();
    let sets: Vec<&str> = parts[1].split(";").collect();
    sets
}

fn check_is_possible(scores: &HashMap<String, usize>, limits: &HashMap<String, usize>) -> bool {
    for (limit_color, limit_value) in limits.into_iter() {
        let score = scores.get(limit_color);
        let is_pass = match score {
            Some(v) => limit_value >= v,
            None => true,
        };
        if !is_pass {
            return false;
        };
        continue;
    }
    return true;
}
