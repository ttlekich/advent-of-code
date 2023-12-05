use std::fs;

fn main() {
    part_1();
}

fn part_1() {
    let lines = fs::read_to_string("1.txt").unwrap();
    let parts: Vec<_> = lines.split("\n\n").collect();
    let seeds = parse_seeds(&parts);
    let maps: &Vec<Vec<Map>> = &parts[1..].iter().map(|&part| parse_map(&part)).collect();
    let locations: Vec<i64> = seeds
        .iter()
        .map(|seed| seed_to_location(&seed, maps))
        .collect();
    println!("{:?}", locations.iter().min());
}

#[derive(Debug)]
struct Seed {
    number: i64,
}

#[derive(Debug)]
struct Map {
    source_range_start: i64,
    destination_range_start: i64,
    range_length: i64,
}

fn parse_seeds(parts: &Vec<&str>) -> Vec<Seed> {
    let raw_seeds = parts[0];
    let raw_seeds_parts: Vec<&str> = raw_seeds.split(": ").collect();
    let raw_seed_numbers = raw_seeds_parts[1];
    let seeds = raw_seed_numbers
        .split_whitespace()
        .map(|seed_number| {
            let number = seed_number.parse::<i64>().unwrap();
            Seed { number: number }
        })
        .collect();
    seeds
}

fn parse_map(part: &str) -> Vec<Map> {
    let parts: Vec<&str> = part.split("\n").collect();
    let raw_maps = &parts[1..];
    let maps = raw_maps
        .iter()
        .map(|raw_map| {
            let values: Vec<_> = raw_map.split_whitespace().collect();
            Map {
                source_range_start: values[1].parse::<i64>().unwrap(),
                destination_range_start: values[0].parse::<i64>().unwrap(),
                range_length: values[2].parse::<i64>().unwrap(),
            }
        })
        .collect();
    maps
}

fn seed_to_location(seed: &Seed, list_of_maps: &Vec<Vec<Map>>) -> i64 {
    let mut current_source = seed.number;
    for maps in list_of_maps.iter() {
        let destination = map_source(maps, current_source);
        current_source = destination;
    }
    current_source
}

fn map_source(maps: &Vec<Map>, source: i64) -> i64 {
    let maps_filtered: Vec<_> = maps
        .iter()
        .filter(|&map| is_in_range(map.source_range_start, map.range_length, source))
        .collect();
    // If no mapping exists for source:
    if maps_filtered.len() == 0 {
        return source;
    }
    // Else:
    let map = maps_filtered[0];
    let difference = map.source_range_start - map.destination_range_start;
    source - difference
}

fn is_in_range(start: i64, length: i64, number: i64) -> bool {
    number >= start && number < start + length
}
