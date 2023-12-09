#![feature(iter_map_windows)]
advent_of_code::solution!(9);

fn parse_game(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(|line|
        line.split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
    ).collect::<Vec<_>>()
}

fn history(row: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut rows: Vec<Vec<i32>> = Vec::new();
    let mut row = row.clone();
    while !row.iter().all(|&n| n == 0) {
        rows.push(row.clone());
        row = row.iter().map_windows(|&[a, b]| b - a).collect::<Vec<_>>();
    }
    rows
}

fn predict(row: &Vec<i32>) -> i32 {
    history(row).iter().rev().map(|row|
        row.iter().last().unwrap()
    ).collect::<Vec<_>>()
    .iter().fold(0, |acc, &n| acc + n)
}

fn back_predict(row: &Vec<i32>) -> i32 {
    history(row).iter().rev().map(|row|
        row.iter().next().unwrap()
    ).collect::<Vec<_>>()
    .iter().fold(0, |acc, &n| n - acc)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse_game(input).iter().map(|line|
        predict(line)
    ).sum::<i32>() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(parse_game(input).iter().map(|line|
        back_predict(line)
    ).sum::<i32>() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
