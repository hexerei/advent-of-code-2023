use std::collections::HashMap;

use itertools::Itertools;
advent_of_code::solution!(12);

fn eval(row: &str) -> Vec<u32> {
    let mut groups = Vec::new();
    let mut group = 0;
    for c in row.chars() {
        match c {
            '#' => group += 1,
            '?' => {},
            _ => {
                if group > 0 {
                    groups.push(group);
                }
                group = 0;
            }
        }
    }
    if group > 0 {
        groups.push(group);
    }
    groups
}

fn permutations(len: usize) -> Option<Vec<String>> {
    // permutations('ABCD', 2) --> AB AC AD BA BC BD CA CB CD DA DB DC
    // permutations(0..3, 0) --> 012 021 102 120 201 210
    let pool = ".#";
    let n = pool.len();
    let mut len = if len == 0 { n } else { len };
    if len > n {
        return None;
    }
    let mut permuted = Vec::new();
    let mut indices = (0..n).collect::<Vec<_>>();
    let mut cycles = (n..n-len).collect::<Vec<_>>();
    for i in indices[..len].iter().cloned() {
        permuted.push(pool[i..i+1].to_string());
    }
    while n > 0 {
        for i in (0..len).rev() {
            cycles[i] -= 1;
            if cycles[i] == 0 {
                //indices[i..] = indices[i+1..] + indices[i..i+1];
                cycles[i] = n - i;
            } else {
                let j = cycles[i];
                //indices[i..] = indices[n-j..n-j+1] + indices[i..i+1];
                //permuted.push(indices[..len].iter().map(|&i| pool[i..i+1].to_string()).collect::<Vec<_>>().join(""));
                for i in indices[..len].iter().cloned() {
                    permuted.push(pool[i..i+1].to_string());
                }
                break;
            }
        }
        if cycles[0] == 0 {
            break;
        }
    }
    None
}

fn permute(row: &[u8], groups: &[usize], is_in: Option<usize>, cache: &mut HashMap<(usize, usize, usize), usize> ) -> usize {
    if row.is_empty() {
        return match (groups.len(), is_in) {
            (1, Some(n)) if n == groups[0] => 1,
            (0, None) => 1,
            _ => 0,
        };
    }
    if groups.is_empty() && is_in.is_some() {
        return 0;
    }
    let key = (row.len(), groups.len(), is_in.unwrap_or(0));
    if let Some(&value) = cache.get(&key) {
        return value;
    }

    let sum = match (row[0], is_in) {
        (b'.', Some(n)) if n != groups[0] => 0,
        (b'.', Some(_)) => permute(&row[1..], &groups[1..], None, cache),
        (b'.', None) => permute(&row[1..], groups, None, cache),
        (b'#', Some(_)) => permute(&row[1..], groups, is_in.map(|n| n+1), cache),
        (b'#', None) => permute(&row[1..], groups, Some(1), cache),
        (b'?', Some(n)) => {
            let mut count = permute(&row[1..], groups, is_in.map(|n| n+1), cache);
            if n == groups[0] {
                count += permute(&row[1..], &groups[1..], None, cache);
            }
            count
        },
        (b'?', None) => {
            permute(&row[1..], &groups[1..], None, cache) +
            permute(&row[1..], &groups[1..], None, cache)
        },
        _ => unreachable!(),
    };
    cache.insert(key, sum);
    sum
}

fn solve(row: &str, groups: Vec<usize>) -> usize {
    println!("{} {:?}", row, groups);

    // let row = (0..5).map(|_| row).join("?");
    // let groups = (0..5).flat_map(|_| &groups).copied().collect::<Vec<_>>();
    // println!("{} {:?}", row, groups);
    // println!("{}", "=".repeat(20));

    let mut cache = HashMap::new();

    let result = permute(row.as_bytes(), &groups, None, &mut cache);

    println!("{}", "=".repeat(20));
    // let jokers = row.chars().enumerate().filter_map(|(col, c)| if c == '?' {Some(col)} else {None}).collect::<Vec<_>>();
    // dbg!(&jokers);
    // let mut row = row.to_string();
    // for i in 0..jokers.len() {
    //     row.replace_range(jokers[i]..jokers[i]+1, "#");
    //     // for j in i+1..jokers.len() {
    //     //     let mut row = row.to_string();
    //     //     row.replace_range(jokers[j]..jokers[j]+1, "#");
    //     //     for k in j+1..jokers.len() {
    //     //         let mut row = row.to_string();
    //     //         row.replace_range(jokers[k]..jokers[k]+1, "#");
    //     //         let eval = eval(&row);
    //     //     }
    //     // }
    // }
    // let eval = eval(row.as_str());

    // dbg!(eval);
    0
}

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input.lines().fold(0, |acc, line| {
        let (first, last) = line.split_once(" ").unwrap();
        acc + solve(first, last.split(",").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>())
    });
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
