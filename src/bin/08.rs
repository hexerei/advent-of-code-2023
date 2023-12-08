use std::collections::{BTreeMap, HashMap};

advent_of_code::solution!(8);

fn parse_game(input: &str) -> (Vec<usize>, BTreeMap<String, (String, String)>) {
    // seperate path and tree
    let (path, tree) = input.split_once("\n\n").unwrap();
    // parse path into list of usize (0 = left, 1 = right)
    let path = path.chars().filter_map(|c| match c {
            'L' => Some(0), 'R' => Some(1), _ => None,
        }).collect::<Vec<usize>>();
    // parse binary tree with entries like AAA = (BBB, CCC)
    //let lookup: HashMap<String, usize> = HashMap::new();
    //let index: usize = 0;
    let tree = tree.split(")\n").filter_map(|line| {
        if line.is_empty() {
            return None;
        }
        let (parent, children) = line.split_once(" = (").unwrap();
        //lookup.insert(parent.to_string(), 0);
        let (left, right) = children.trim().split_once(", ").unwrap();
        Some((parent.to_string(), (left.to_string(), right.to_string())))
    }).collect::<BTreeMap<_,_>>();

    // let parents = tree.iter().enumerate()
    //     .map(|(i, (parent, _))| (parent.to_string(), i))
    //     .collect::<HashMap<_,_>>();
    // let tree = tree.iter().map(|(parent, (left, right))| {
    //     (parents[parent.to_string()], (parents[left.to_string()], parents[right.to_string()]))
    // }).collect::<BTreeMap<_,_>>();

    dbg!(&tree);
    (path, tree)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (path, tree) = parse_game(input);

    let mut current = tree.get("AAA").unwrap();
    let mut count: usize = 1;
    for step in path {
        let next_key = match step {
            0 => &current.0,
            1 => &current.1,
            _ => panic!("invalid step"),
        };
        if next_key == "ZZZ" {
            println!("found ZZZ in {} steps", count);
            break;
        }
        count += 1;
        current = tree.get(next_key).unwrap();
    }

    dbg!(current);
    Some(count as u32)
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
