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
    // collect columns to expand
    let mut expand_cols = Vec::new();
    let width = game[0].len();
    for col in 0..width {
        if game.iter().map(|row|
            *row.iter().nth(col).unwrap()
        ).collect::<Vec<_>>().iter().sum::<u8>() == 0 {
            expand_cols.push(col);
        }
    }
    let mut expand_rows = Vec::new();
    for row in 0..game.len() {
        if game[row].iter().sum::<u8>() == 0 {
            expand_rows.push(row);
        }
    }

    let points = game.iter().enumerate().map(|(row, line)|
        line.iter().enumerate().filter_map(|(col, &cell)|
            if cell == 1 {
                Some((row, col))
            } else {
                None
            }
        ).collect::<Vec<_>>()
    ).collect::<Vec<_>>().concat();

    let mut new_game = points.clone();
    let step = steps - 1;
    // let mut col_step = steps;
    // let mut row_step = steps;
    //let t = expand_cols.iter().map(|&c| c).collect::<Vec<_>>();
    for (i, &(row, col)) in points.iter().enumerate() {
        let (mut nrow, mut ncol) = (row, col);
        for c in expand_cols.iter() {
            if *c < col {
                ncol += step;
                //col_step += steps;
            }
        }
        for r in expand_rows.iter() {
            if *r < row {
                nrow += step;
                //row_step += steps;
            }
        }
        new_game[i] = (nrow, ncol);
    }
    new_game
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
