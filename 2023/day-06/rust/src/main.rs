use std::fs;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let file = fs::read_to_string("1.txt").unwrap();
    let lines: Vec<_> = file.lines().collect();
    let times: Vec<i64> = lines[0]
        .split(":")
        .skip(1)
        .flat_map(|raw_times| {
            raw_times.split_whitespace().map(|raw_time| {
                let time_string = raw_time.trim();
                time_string.parse::<i64>().unwrap()
            })
        })
        .collect();
    let distances: Vec<i64> = lines[1]
        .split(":")
        .skip(1)
        .flat_map(|raw_distances| {
            raw_distances.split_whitespace().map(|raw_distance| {
                let distance_string = raw_distance.trim();
                distance_string.parse::<i64>().unwrap()
            })
        })
        .collect();
    let inputs: Vec<(&i64, &i64)> = times.iter().zip(distances.iter()).collect();

    let winning_speeds: i64 = inputs
        .iter()
        .map(|&input| find_winning_speeds(input))
        .map(|winning_speeds| winning_speeds.len() as i64)
        .fold(1, |acc: i64, val| acc * val);

    println!("{:?}", winning_speeds);
    // 41
    // 249
}

fn find_winning_speeds(input: (&i64, &i64)) -> Vec<i64> {
    let (time, distance) = input;
    let mut results = vec![];
    for speed in 1..*time {
        let time_left = time - speed;
        let new_distance = time_left * speed;
        if new_distance > *distance {
            results.push(speed)
        }
    }
    results
}

fn part_2() {
    let file = fs::read_to_string("2.txt").unwrap();
    let lines: Vec<_> = file.lines().collect();
    let times: Vec<i64> = lines[0]
        .split(":")
        .skip(1)
        .flat_map(|raw_times| {
            raw_times.split_whitespace().map(|raw_time| {
                let time_string = raw_time.trim();
                time_string.parse::<i64>().unwrap()
            })
        })
        .collect();
    let distances: Vec<i64> = lines[1]
        .split(":")
        .skip(1)
        .flat_map(|raw_distances| {
            raw_distances.split_whitespace().map(|raw_distance| {
                let distance_string = raw_distance.trim();
                distance_string.parse::<i64>().unwrap()
            })
        })
        .collect();
    let inputs: Vec<(&i64, &i64)> = times.iter().zip(distances.iter()).collect();
    println!("{:?}", inputs);

    let winning_speeds: i64 = inputs
        .iter()
        .map(|&input| find_winning_speeds(input))
        .map(|winning_speeds| winning_speeds.len() as i64)
        .fold(1, |acc: i64, val| acc * val);

    println!("{:?}", winning_speeds);
    // 41
    // 249
}
