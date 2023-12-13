use std::{cmp, fs, usize};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let output_1 = part_1(&input);
    println!("{:?}", output_1);
    let output_2 = part_2(&input);
    println!("{:?}", output_2);
}

fn part_1(input: &str) -> usize {
    let blocks = parse_blocks(&input);
    blocks
        .iter()
        .map(|block| {
            let solution = solve(block);
            solution
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let blocks = parse_blocks(&input);
    blocks
        .iter()
        .map(|block| {
            let variations = make_variations(block);
            let solution = variations
                .iter()
                .map(|variation| {
                    return solve_2(block, variation);
                })
                .max()
                .unwrap();
            solution
        })
        .sum()
}

fn make_variations(block: &Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let mut variations = vec![];
    for (i, _row) in block.iter().enumerate() {
        for (j, _col) in block[i].iter().enumerate() {
            let mut new_block = block.clone();
            new_block[i][j] = match new_block[i][j] {
                '.' => '#',
                '#' => '.',
                _ => unreachable!(),
            };
            variations.push(new_block);
        }
    }
    variations
}

fn parse_blocks(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .split("\n\n")
        .map(|rb| {
            rb.lines()
                .map(|l| l.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        })
        .collect::<Vec<Vec<Vec<char>>>>()
}

fn solve_2(block: &Vec<Vec<char>>, variant: &Vec<Vec<char>>) -> usize {
    let block_t = &transpose(block);
    let variant_t = &transpose(variant);

    let mut mid = None;
    let mut mid_t = None;

    if let Some(m) = find_mirror_line(block) {
        mid = Some(m)
    }

    if let Some(m) = find_mirror_line(block_t) {
        mid_t = Some(m)
    }

    if let Some((v_low, v_high)) = find_mirror_line_2(variant, mid) {
        return (v_low + 1) * 100;
    }

    if let Some((v_low, v_high)) = find_mirror_line_2(variant_t, mid_t) {
        return v_low + 1;
    }

    return 0;
}

fn solve(block: &Vec<Vec<char>>) -> usize {
    if let Some((low, _)) = find_mirror_line(block) {
        return (low + 1) * 100;
    }

    if let Some((low, _)) = find_mirror_line(&transpose(block)) {
        return low + 1;
    }

    return 0;
}

fn find_mirror_line_2(
    block: &Vec<Vec<char>>,
    invalid: Option<(usize, usize)>,
) -> Option<(usize, usize)> {
    let mut mids = vec![];
    for (i, line) in block.iter().enumerate() {
        if i + 1 == block.len() {
            break;
        }
        if line == &block[i + 1].to_vec() {
            mids.push((i, i + 1));
        }
    }
    for mid in mids.into_iter() {
        if let Some(m) = invalid {
            if m.0 == mid.0 && m.1 == mid.1 {
                continue;
            }
        }
        let min = cmp::min(block.len() - mid.1, mid.0);
        for x in 0..=min {
            if mid.1 + x == block.len() {
                return Some(mid);
            }
            if block[mid.0 - x] == block[mid.1 + x] {
                if mid.0 - x == 0 {
                    return Some(mid);
                }
            } else {
                break;
            }
        }
    }
    return None;
}

fn find_mirror_line(block: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let mut mids = vec![];
    for (i, line) in block.iter().enumerate() {
        if i + 1 == block.len() {
            break;
        }
        if line == &block[i + 1].to_vec() {
            mids.push((i, i + 1));
        }
    }
    for mid in mids.into_iter() {
        let min = cmp::min(block.len() - mid.1, mid.0);
        for x in 0..=min {
            if mid.1 + x == block.len() {
                return Some(mid);
            }
            if block[mid.0 - x] == block[mid.1 + x] {
                if mid.0 - x == 0 {
                    return Some(mid);
                }
            } else {
                break;
            }
        }
    }
    return None;
}

fn transpose<T: Clone>(a: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..a[0].len())
        .map(|r| a.iter().map(|c| c[r].clone()).collect::<Vec<T>>())
        .collect()
}

fn _pretty_print(a: &Vec<Vec<char>>) {
    for row in a.iter() {
        for column in row.iter() {
            print!("{}", column);
        }
        println!();
    }
    println!();
}
