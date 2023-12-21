use std::collections::HashMap;
use itertools::Itertools;

const VERBOSE: bool = false;
const MAX_STEPS: u32 = 500;

advent_of_code::solution!(21);

fn parse_map(input: &str) -> (Vec<Vec<u8>>, (usize, usize)) {
    let mut start_pos = (0, 0);
    let map = input
        .lines().enumerate()
        .map(|(y, line)| {
            line.chars().enumerate()
                .map(|(x, c)| match c {
                    '.' => 0,
                    '#' => 1,
                    'S' => {
                        start_pos = (x, y);
                        0
                    },
                    _ => panic!("Invalid character"),
                }).collect::<Vec<_>>()
            })
        .collect::<Vec<_>>();
    (map, start_pos)
}

fn next_steps(map: &Vec<Vec<u8>>, moves: Vec<(usize, usize)>, max_steps: u32) -> Vec<(usize, usize)> {
    if max_steps == 0 {
        return moves;
    }
    if VERBOSE {println!("next_steps({:?}, {:?})", moves, max_steps)};
    let mut steps = HashMap::new();
    for pos in moves {
        let (x, y) = pos;
        for dir in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let (dx, dy) = (x as i32 + dir.0, y as i32 + dir.1);
            if dx < 0 || dx >= map[y].len() as i32 || dy < 0 || dy >= map.len() as i32 { continue; }
            let (dx, dy) = (dx as usize, dy as usize);
            if map[dy][dx] == 0 { steps.insert((dx, dy), 1); }
        }
    }
    return next_steps(map, steps.keys().cloned().collect_vec(), max_steps-1);
}

fn infinite_steps(map: &Vec<Vec<u8>>, moves: Vec<((usize, usize), (i32, i32))>, max_steps: u32) -> Vec<((usize, usize), (i32, i32))> {
    if max_steps == 0 {
        return moves;
    }
    if VERBOSE {println!("next_steps({:?}, {:?})", moves, max_steps)};
    let mut steps = HashMap::new();
    let (rows, cols) = (map.len() as i32, map[0].len() as i32);
    for ((x, y), (mx, my)) in moves {
        for dir in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let (mut dx, mut dy) = (x as i32 + dir.0, y as i32 + dir.1);
            let (mut mx, mut my) = (mx, my);
            if dx < 0 { mx -= 1; dx += cols as i32 } else if dx > cols-1 { mx += 1; dx -= cols as i32 };
            if dy < 0 { my -= 1; dy += rows as i32 } else if dy > rows-1 { my += 1; dy -= rows as i32 };
            let (dx, dy) = (dx as usize, dy as usize);
            if map[dy][dx] == 0 {
                steps.insert(((dx, dy), (mx, my)), 1);
            }
        }
    }
    let steps = steps.keys().cloned().collect_vec();
    return infinite_steps(map, steps, max_steps-1);
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, start_pos) = parse_map(input);
    let steps = next_steps(&map, vec![start_pos], 64);
    Some(steps.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, start_pos) = parse_map(input);
    let steps = infinite_steps(&map, vec![(start_pos, (0, 0))], MAX_STEPS);
    Some(steps.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
