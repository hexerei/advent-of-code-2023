advent_of_code::solution!(5);

use std::collections::HashMap;
use std::time::Instant;

fn parse_numbers(line: &str) -> Vec<u64> {
    let mut numbers: Vec<u64> = Vec::new();
    let line = line.trim().replace("  ", " ");
    let line = line.split(" ");
    for n in line {
        let n = n.trim();
        let n = n.parse::<u64>();
        if n.is_err() {
            continue;
        }
        numbers.push(n.unwrap());
    }
    numbers
}

fn parse_input(input: &str) -> (Vec<u64>, HashMap<String, Vec<(u64,u64,u64)>>) {
    let mut in_map: bool = false;
    let mut seeds: Vec<u64> = Vec::new();
    let mut map_key: String = String::new();
    let mut tmp_map: Vec<(u64,u64,u64)> = Vec::new();
    let mut maps: HashMap<String, Vec<(u64,u64,u64)>> = HashMap::new();
    for line in input.lines() {
        if line.starts_with("seeds:") {
            let (_, line) = line.split_at(6);
            seeds = parse_numbers(line);
        } else if line.ends_with("map:") {
            let (line, _) = line.split_at(line.find(" map:").unwrap());
            map_key = String::from(line);
            tmp_map = Vec::new();
            in_map = true;
        } else if line.is_empty() && in_map {
            in_map = false;
            maps.insert(map_key.clone(), tmp_map.clone());
        } else if in_map {
            let m = parse_numbers(line);
            if m.len() != 3 {
                continue;
            }
            tmp_map.push((m[0], m[1], m[2]));
        // } else {
        //     println!("Line: {:?}", line);
        }
    }
    if in_map {
        maps.insert(map_key.clone(), tmp_map.clone());
    }
    (seeds, maps)
}

fn map_value(value: u64, map: &Vec<(u64,u64,u64)>) -> u64 {
    let mut result = value;
    for m in map {
        let (dest, src, len) = *m;
        if value >= src as u64 && value < src as u64 + len as u64 {
            result = dest as u64 + (value - src as u64);
            break;
        }
    }
    result
}

// #[derive(Debug)]
// struct SeedInfo {
//     seed: u64,
//     soil: u64,
//     fertilizer: u64,
//     water: u64,
//     light: u64,
//     temperature: u64,
//     humidity: u64,
//     location: u64
// }
// impl SeedInfo {
//     fn new(seed: u64, maps: &HashMap<String, Vec<(u64,u64,u64)>>) -> SeedInfo {
//         let soil = map_value(seed, &maps["seed-to-soil"]);
//         let fertilizer = map_value(soil, &maps["soil-to-fertilizer"]);
//         let water = map_value(fertilizer, &maps["fertilizer-to-water"]);
//         let light = map_value(water, &maps["water-to-light"]);
//         let temperature = map_value(light, &maps["light-to-temperature"]);
//         let humidity = map_value(temperature, &maps["temperature-to-humidity"]);
//         let location = map_value(humidity, &maps["humidity-to-location"]);
//         SeedInfo {
//             seed, soil, fertilizer, water, light,
//             temperature, humidity, location
//         }
//     }
// }

pub fn get_location(seed: u64, maps: &HashMap<String, Vec<(u64,u64,u64)>>) -> u64 {
    let tmp = map_value(seed, &maps["seed-to-soil"]);
    let tmp = map_value(tmp, &maps["soil-to-fertilizer"]);
    let tmp = map_value(tmp, &maps["fertilizer-to-water"]);
    let tmp = map_value(tmp, &maps["water-to-light"]);
    let tmp = map_value(tmp, &maps["light-to-temperature"]);
    let tmp = map_value(tmp, &maps["temperature-to-humidity"]);
    map_value(tmp, &maps["humidity-to-location"])
}

pub fn print_input(seeds: &Vec<u64>, maps: &HashMap<String, Vec<(u64,u64,u64)>>) {
    println!("seeds:");
    for n in seeds {
        println!("{n} -> location: {}", get_location(*n, &maps));
        //println!("{:?}", SeedInfo::new(*n, &maps));
    }
    println!("");
    for m in maps {
        println!("{} map:", m.0);
        for n in m.1 {
            println!("{:?} {:?} {:?}", n.0, n.1, n.2);
        }
        println!("");
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let (seeds, maps) = parse_input(input);
    let mut lowest: u64 = u64::MAX;
    for n in seeds {
        let location = get_location(n, &maps);
        if location < lowest {
            lowest = location;
        }
    }
    Some(lowest as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (seeds, maps) = parse_input(input);
    let mut lowest: u64 = u64::MAX;
    for i in (0..seeds.len()).step_by(2) {
        let start = seeds[i];
        let end = start + seeds[i+1];
        println!("Processing {} values from {} to {}", seeds[i+1], start, end);
        let start_time = Instant::now();
        for j in start..end {
            let location = get_location(j, &maps);
            if location < lowest {
                lowest = location;
            }
        }
        let elapsed = start_time.elapsed();
        println!("Processing took {:?}", elapsed);
    }
    Some(lowest as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
