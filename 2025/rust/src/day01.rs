use std::fs;

pub fn part01() {
    let result = fs::read_to_string("./input/day01.txt")
        .unwrap()
        .lines()
        .fold((50, 0), |state: (i64, u64), line| {
            let (position, count) = state;
            let rotation = parse_line(line);
            let next_position = (position + (rotation % 100)) % 100;

            let next_state = match next_position {
                np if np < 0 => (np + 100, count),
                np if np == 0 => (np, count + 1),
                np => (np, count),
            };
            next_state
        });
    println!("part01: {}, {}", result.0, result.1)
}

fn parse_line(line: &str) -> i64 {
    let direction = match line.chars().next().unwrap() {
        'R' => 1,
        'L' => -1,
        _ => panic!("impossible"),
    };
    let volume_string: String = line.chars().skip(1).collect();
    let volume: i64 = volume_string.parse().unwrap();
    volume * direction
}

pub fn part02() {
    let result = fs::read_to_string("./input/day01.txt")
        .unwrap()
        .lines()
        .fold((50, 0), |state: (i64, i64), line| {
            let (position, count) = state;
            let rotation = parse_line(line);
            let rotation_clicks = rotation.abs() / 100;
            let raw_next_position = position + (rotation % 100);
            let next_position = (raw_next_position) % 100;
            let extra_clicks = match raw_next_position {
                rnp if rnp > 100 => 1,                // carry the 1
                rnp if rnp < 0 && position != 0 => 1, // crossed 0 from positive to negative
                _ => 0,
            };
            let clicks = rotation_clicks + extra_clicks;

            let next_state = match next_position {
                np if np < 0 && position != 0 => (np + 100, count + clicks),
                np if np < 0 => (np + 100, count + clicks),
                np if np == 0 => (np, count + clicks + 1), // landed on 0 or 100
                np => (np, count + clicks),
            };

            println!("{}, {}, {}", rotation, next_state.0, next_state.1);
            next_state
        });
    println!("part02: {}, {}", result.0, result.1)
}
