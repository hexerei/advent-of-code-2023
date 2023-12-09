use std::collections::BTreeMap;

advent_of_code::solution!(8);

fn parse_game(input: &str) -> (Vec<usize>, BTreeMap<String, (String, String)>) {

    let (path, tree) = input.split_once("\n\n").unwrap();

    // parse path into list of usize (0 = left, 1 = right)
    let path = path.chars().filter_map(|c| match c {
            'L' => Some(0), 'R' => Some(1), _ => None,
        }).collect::<Vec<usize>>();

        // parse binary tree with entries like AAA = (BBB, CCC)
    let tree = tree.split(")\n").filter_map(|line| {
        if line.is_empty() {
            return None;
        }
        let (parent, children) = line.split_once(" = (").unwrap();
        let (left, right) = children.trim().split_once(", ").unwrap();
        Some((parent.to_string(), (left.to_string(), right.to_string())))
    }).collect::<BTreeMap<_,_>>();

    (path, tree)
}

fn solve_path(start: &str, end: &str, path: &Vec<usize>, tree: &BTreeMap<String, (String, String)>) -> usize {
    let mut current = tree.get(start).unwrap();
    let mut count: usize = 1;
    loop {
        for step in path.iter() {
            let next_key = match step {
                0 => &current.0,
                1 => &current.1,
                _ => panic!("invalid step"),
            };
            if next_key.ends_with(end) {
                return count;
            }
            count += 1;
            current = tree.get(next_key).unwrap();
        }
    }
}

// calculate greatest common divisor
fn gcd(mut a:usize, mut b:usize) -> usize{
    if a==b { return a; }
    if b > a { std::mem::swap(&mut a, &mut b); }
    while b>0 {
        let temp = a;
        a = b;
        b = temp%b;
    }
    return a;
}

// calculate least common multiple
fn lcm(a:usize, b:usize) -> usize{
    return a*(b/gcd(a,b));
}

// NOT USED: as LCM is 5 to 30 times faster
// NOTE: this is a naive implementation of prime factorization
// fn get_prime(n:usize) -> usize {
//     let mut i = 2;
//     while i*i<=n {
//         if n%i == 0 { return i; }
//         i+=1;
//     }
//     return 0;
// }

pub fn part_one(input: &str) -> Option<u32> {
    let (path, tree) = parse_game(input);
    Some(solve_path("AAA", "ZZZ", &path, &tree) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (path, tree) = parse_game(input);
    let result = tree.keys()
        .filter_map(|parent| match parent.ends_with("A") {
            true => Some(parent.to_string()),
            false => None,
        }).collect::<Vec<_>>().iter()
        .map(|parent| solve_path(parent, "Z", &path, &tree))
        .collect::<Vec<_>>()
        .iter().fold(1, |acc, &count| lcm(acc, count));
    // .iter().fold(1, |acc, &count| acc * get_prime(count)) * path.len();
    println!("Got {}", result);
    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one("LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)");
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
