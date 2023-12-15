use std::collections::HashMap;
use itertools::Itertools;

advent_of_code::solution!(12);

fn permute(row: &[u8], groups: &[usize], is_in: usize, cache: &mut HashMap<(usize, usize, usize), usize> ) -> usize {

    if row.is_empty() {
        return if (groups.is_empty() && is_in == 0)
        || (groups.len() == 1 && is_in == groups[0])
        { 1 } else { 0 };
    }

    if groups.is_empty() && is_in > 0 {
        return 0;
    }

    let key = (row.len(), groups.len(), is_in);
    if let Some(&value) = cache.get(&key) {
        //println!("{:?} = {} CACHE HIT", key, value);
        return value;
    }

    //println!("permute {:?} {:?} {:?}", std::str::from_utf8(row).unwrap(), groups, key);
    let sum = match row[0] {
        b'?' => if is_in > 0 {
            if is_in == groups[0] {
                permute(&row[1..], groups, is_in + 1, cache) +
                permute(&row[1..], &groups[1..], 0, cache)
            } else {
                permute(&row[1..], groups, is_in + 1, cache)
            }
        } else {
            permute(&row[1..], groups, 1, cache) +
            permute(&row[1..], groups, 0, cache)
        },
        b'#' => if is_in > 0 {
            permute(&row[1..], groups, is_in + 1, cache)
        } else {
            permute(&row[1..], groups, 1, cache)
        },
        b'.' => if is_in > 0 {
            if is_in != groups[0] { 0 } else {
                permute(&row[1..], &groups[1..], 0, cache)
            }
        } else {
            permute(&row[1..], groups, 0, cache)
        },
        _ => unreachable!(),
    };
    //println!("{:?} = {}", key, sum);
    cache.insert(key, sum);
    sum
}

fn solve(row: &str, groups: Vec<usize>, part2: bool) -> usize {
    let mut cache = HashMap::new();
    let result: usize;

    if part2 {
        let row = (0..5).map(|_| row).join("?");
        let groups = (0..5).flat_map(|_| &groups).copied().collect::<Vec<_>>();
        result = permute(row.as_bytes(), &groups, 0, &mut cache);
        //println!("{} {:?} ... {} permutations", row, groups, result);
    } else {
        result = permute(row.as_bytes(), &groups, 0, &mut cache);
        //println!("{} {:?} ... {} permutations", row, groups, result);
    }

    result
}

pub fn part_one(input: &str) -> Option<usize> {
    let sum = input.lines().fold(0, |acc, line| {
        let (first, last) = line.split_once(" ").unwrap();
        acc + solve(first, last.split(",").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>(), false)
    });
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let sum = input.lines().fold(0, |acc, line| {
        let (first, last) = line.split_once(" ").unwrap();
        acc + solve(first, last.split(",").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>(), true)
    });
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
