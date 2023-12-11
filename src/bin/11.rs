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

fn expand(game: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
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
    let mut new_game = Vec::new();
    for row in 0..game.len() {
        let mut new_row = game[row].clone();
        let row_sum = new_row.iter().sum::<u8>();
        let mut inserted: usize = 0;
        for col in expand_cols.iter() {
            if *col < width - 1 {
                new_row.insert(*col + inserted, 0);
            } else {
                new_row.push(0);
            }
            inserted += 1;
        }
        if row_sum == 0 {
            new_game.push(new_row.clone());
            new_game.push(new_row);
        } else {
            new_game.push(new_row);
        }
    }
    new_game
}

fn get_points(game: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    game.iter().enumerate().map(|(row, line)|
        line.iter().enumerate().filter_map(|(col, &cell)|
            if cell == 1 {
                Some((row, col))
            } else {
                None
            }
        ).collect::<Vec<_>>()
    ).collect::<Vec<_>>().concat()
}

fn print_game(game: &Vec<Vec<u8>>) {
    for row in game.iter() {
        for col in row.iter() {
            print!("{}", if *col == 1 { '#' } else { '.' });
        }
        println!();
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let game = parse_game(input);
    let game = expand(&game);
    print_game(&game);
    let points = get_points(&game);
    println!("Extracted {} galaxies", points.len());
    let mut matched = Vec::new();
    let pairs = points.iter().enumerate().map(|(i, p1)|
        points.iter().enumerate().filter_map(|(j, p2)|
            if i == j || matched.contains(&(j, i)) || matched.contains(&(i, j)) {
                None
            } else {
                matched.push((i, j));
                Some(((p1.0 as i32 - p2.0 as i32).abs() + (p1.1 as i32 - p2.1 as i32).abs()) as u32)
            }
        ).collect::<Vec<_>>()
    ).collect::<Vec<_>>().concat();
    Some(pairs.iter().sum::<u32>())
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
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
