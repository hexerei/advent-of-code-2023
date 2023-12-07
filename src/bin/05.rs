advent_of_code::solution!(5);

use itertools::Itertools;
use std::time::Instant;

const OPTIMIZED: bool = true;
// Kudos to @AxlLind for inspiration for the optimize code

//=== shared code =============================================================

fn parse_numbers(line: &str, header: bool) -> Vec<u64> {
    line.split_whitespace()
        .skip(if header {1} else {0})
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
}


fn parse_game(input: &str) -> (Vec<u64>, Vec<Vec<(u64,u64,u64)>>) {
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let seeds = parse_numbers(seeds, true);
    //println!("Seeds: {:?}", seeds);
    let maps = maps.split("\n\n")
        .map(|m| {
            m.split("\n").skip(1).map(|n| {
                parse_numbers(n, false)
                    .iter().map(|n| *n).collect_tuple()
                    .unwrap_or((0,0,0))
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();
    (seeds, maps)
}


//=== unoptimized code - brute-force ==========================================

fn map_value(value: u64, map: &Vec<(u64,u64,u64)>) -> u64 {
    for m in map {
        let (dest, src, len) = *m;
        if value >= src && value < src + len {
            return dest + (value - src);
        }
    }
    value
}

pub fn get_location(seed: u64, maps: &Vec<Vec<(u64,u64,u64)>>) -> u64 {
    let mut tmp = seed;
    for i in 0..maps.len() {
        tmp = map_value(tmp, &maps[i]);
    }
    tmp
}


//=== optimized code - range mapping ==========================================

// part 1
fn get_min_location(seeds: Vec<u64>, maps: &Vec<Vec<(u64,u64,u64)>>) -> u64 {
    let locations = maps.iter()
        .fold(seeds, |seeds, mappings|
            seeds.into_iter().map(|seed| 
                mappings.iter()
                    .find(|(_dist, src, len)|
                        seed >= *src && seed < *src + *len
                    ).map(|(dest, src, _len)|
                        dest + (seed - src)
                    ).unwrap_or(seed)
                ).collect::<Vec<_>>()
            );
    *locations.iter().min().unwrap()
}

// part 2
fn get_min_location_ranged(seeds: Vec<u64>, maps: &Vec<Vec<(u64,u64,u64)>>) -> u64 {
    let seeds = seeds.into_iter().tuples()
        .map(|(seed, len)| (seed, seed + len))
        .collect::<Vec<_>>();
    let locations = maps.iter()
        .fold(seeds, |seeds, mappings|
            seeds.iter().flat_map(|&(start, end)| {
                let mut checked = Vec::new();
                let mut check = vec![(start, end)];
                for &(dest, vstart, vlen) in mappings {
                    let mut m = Vec::new();
                    for (start, end) in check {
                        let vend = vstart+vlen;
                        if start < end.min(vstart) {
                            m.push((start, end.min(vstart)));
                        }
                        if start.max(vstart) < vend.min(end) {
                            checked.push((
                                start.max(vstart) - vstart + dest,
                                vend.min(end) - vstart + dest
                            ));
                        }
                        if vend.max(start) < end {
                            m.push((vend.max(start), end));
                        }
                    }
                    check = m;
                }
                checked.extend(check);
                checked
            }).collect()
        );
    locations.iter().map(|&(t, _)| t).min().unwrap()
}


//=== main ===================================================================

pub fn part_one(input: &str) -> Option<u32> {
    let (seeds, maps) = parse_game(input);
    if OPTIMIZED {
        return Some(get_min_location(seeds, &maps) as u32);
    }
    let lowest = seeds.iter().map(|n| get_location(*n, &maps)).min().unwrap();
    Some(lowest as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (seeds, maps) = parse_game(input);
    if OPTIMIZED {
        return Some(get_min_location_ranged(seeds, &maps) as u32);
    }
    // ATTENTION! brute-force will take quite a while
    let mut lowest: u64 = u64::MAX;
    for i in (0..seeds.len()).step_by(2) {
        let (start, end) = (seeds[i], seeds[i] + seeds[i+1]);
        println!("Processing {} values from {} to {}", seeds[i+1], start, end);
        let start_time = Instant::now();
        lowest = lowest.min((start..end).into_iter().map(|n| get_location(n, &maps)).min().unwrap());
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
