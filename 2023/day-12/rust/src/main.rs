use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut cache = HashMap::new();
    let output_1 = part_x(&input, 1, &mut cache);
    println!("[PART 1] :: {}", output_1);
    cache.clear();
    let output_2 = part_x(&input, 5, &mut cache);
    println!("[PART 2] :: {}", output_2);
}

fn part_x(
    input: &str,
    instances: usize,
    cache: &mut HashMap<(Vec<char>, Vec<usize>), usize>,
) -> usize {
    input
        .lines()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();
            let chars = vec![parts[0]; instances]
                .join("?")
                .chars()
                .collect::<Vec<char>>();
            let numbers = vec![parts[1]; instances]
                .join(",")
                .split(",")
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (chars, numbers)
        })
        .map(|(chars, numbers)| solve(&chars, &numbers, cache))
        .sum()
}

fn solve(
    chars: &Vec<char>,
    numbers: &Vec<usize>,
    cache: &mut HashMap<(Vec<char>, Vec<usize>), usize>,
) -> usize {
    // No more chars left.
    if chars.is_empty() {
        // If no more numbers, we have a match!
        if numbers.is_empty() {
            return 1;
        }
        // If more numbers, we don't have a match!
        // "" 1, 2
        return 0;
    }

    match chars[0] {
        // Skip, nothing we can do.
        '.' => solve(&chars[1..].to_vec(), numbers, cache),
        // Process '.' and '#' possible branches.
        '?' => solve(&chars[1..].to_vec(), numbers, cache) + solve_damaged(chars, numbers, cache),
        // Process #' possible branches.
        '#' => solve_damaged(chars, numbers, cache),
        _ => unreachable!(),
    }
}

fn solve_damaged(
    chars: &Vec<char>,
    numbers: &Vec<usize>,
    cache: &mut HashMap<(Vec<char>, Vec<usize>), usize>,
) -> usize {
    // Value has been computed.
    if let Some(&v) = cache.get(&(chars.clone(), numbers.clone())) {
        return v;
    }

    // No more numbers, but chars not empty, no match!
    // "" _
    if numbers.is_empty() && !chars.is_empty() {
        return 0;
    }

    // Current number.
    let n = numbers[0];

    // Current number is bigger than rest of chars, then no match.
    // "#" 3
    if n > chars.len() {
        return 0;
    }

    // If there is an interruption of parts between here and n, then no match.
    // "#.#" 3
    for i in 0..n {
        if chars[i] == '.' {
            return 0;
        }
    }

    // Correct amount of parts left.
    if chars.len() == n {
        // 1 Number, match! "##" 2
        if numbers.len() == 1 {
            return 1;
        }
        // Too many numbers, no match. "##" 2,1
        return 0;
    }

    // Current number away is a #; cannot have 2 together; no match.
    // "####" 3
    if chars[n] == '#' {
        return 0;
    }

    // char[0..n] was a match, continue processing rest of possibilities after n.
    let value = solve(&chars[(n + 1)..].to_vec(), &numbers[1..].to_vec(), cache);

    // save already calculated states in cache.
    cache.insert((chars.clone(), numbers.clone()), value);

    value
}
