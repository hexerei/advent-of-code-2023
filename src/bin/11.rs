use std::collections::HashSet;

advent_of_code::solution!(11);


fn parse_game(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| {
        line.chars().map(|c| {
            match c {
                '#' => 1,
                _ => 0,
            }
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>()
}

fn expand(game: Vec<Vec<u8>>, steps: usize) -> Vec<(usize, usize)> {
    let step = steps - 1;

    let mut points = game.iter().enumerate().map(|(row, line)|
        line.iter().enumerate().filter_map(|(col, &c)|
            if c == 1 { Some((row, col)) } else { None }
        ).collect::<Vec<_>>()
    ).collect::<Vec<_>>().concat();

    for r in (0..game.len()).filter(|&r| game[r].iter().all(|&c| c == 0)).rev() {
        for g in &mut points {
            if g.0 > r { g.0 += step }
        }
    }
    for c in (0..game[0].len()).filter(|&c| (0..game.len()).all(|r| game[r][c] == 0)).rev() {
        for g in &mut points {
            if g.1 > c { g.1 += step }
        }
    }
    points
}

fn get_pairs(points: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut matched = HashSet::new();
    points.iter().enumerate().map(|(i, p1)|
        points.iter().enumerate().filter_map(|(j, p2)|
            if i == j || matched.contains(&(i.min(j), j.max(i))) {
                None
            } else {
                matched.insert((i.min(j), j.max(i)));
                Some(p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1))
            }
        ).collect::<Vec<_>>()
    ).collect::<Vec<_>>().concat()
}


pub fn part_one(input: &str) -> Option<usize> {
    let points = expand(parse_game(input), 2);
    let pairs = get_pairs(&points);
    Some(pairs.iter().sum::<usize>())
}

pub fn part_two(input: &str) -> Option<usize> {
    let points = expand(parse_game(input), 1000000);
    let pairs = get_pairs(&points);
    Some(pairs.iter().sum::<usize>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
