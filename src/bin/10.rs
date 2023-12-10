advent_of_code::solution!(10);

// .....
// .S-7.
// .|.|.
// .L-J.
// .....

#[derive(Debug, PartialEq)]
enum Direction {
    None,
    North,
    East,
    South,
    West,
}
type Segment = (Direction, Direction);

fn parse_game(input: &str) -> Vec<Vec<Option<Segment>>> {
    input.lines().map(|line| {
        line.chars().map(|c| {
            match c {
                '|' => Some((Direction::North, Direction::South)),
                '-' => Some((Direction::West, Direction::East)),
                'L' => Some((Direction::North, Direction::East)),
                'J' => Some((Direction::North, Direction::West)),
                '7' => Some((Direction::West, Direction::South)),
                'F' => Some((Direction::East, Direction::South)),
                'S' => Some((Direction::None, Direction::None)),
                _ => None,
            }
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>()
}

fn match_neighbour(game: &Vec<Vec<Option<Segment>>>, row: i32, col: i32, dir: Direction, any: Option<usize>) -> bool {
    if row >= 0 && row < game.len()  as i32
    && col >= 0 && col < game[row as usize].len() as i32 {
        let segment = game[row as usize][col as usize].as_ref();
        if segment.is_some() {
            //dbg!(&segment);
            let (s, e) = segment.unwrap();
            // start position is joker
            if *s == Direction::None && *e == Direction::None {
                return true;
            }
            match any {
                Some(n) => if n == 0 && *s == dir {
                    return true;
                } else if n == 1 && *e == dir {
                    return true;
                },
                None => if *s == dir || *e == dir {
                    return true;
                },
            }
        }
    }
    false
}

fn check_segment(game: &Vec<Vec<Option<Segment>>>, row: usize, col: usize) -> bool {
    let segment = game[row][col].as_ref();
    if segment.is_some() {
        let (s, e) = segment.unwrap();
        match *s {
            Direction::North => if !match_neighbour(game, row as i32 - 1, col as i32, Direction::South, None) {
                return false;
            },
            Direction::West => if !match_neighbour(game, row as i32, col as i32 - 1, Direction::East, None) {
                return false;
            },
            Direction::East => if !match_neighbour(game, row as i32, col as i32 + 1, Direction::West, None) {
                return false;
            },
            _ => return false, // never starts with south
        }
        match *e {
            Direction::East => if !match_neighbour(game, row as i32, col as i32 + 1, Direction::West, None) {
                return false;
            },
            Direction::South => if !match_neighbour(game, row as i32 + 1, col as i32, Direction::North, None) {
                return false;
            },
            Direction::West => if !match_neighbour(game, row as i32, col as i32 - 1, Direction::East, None) {
                return false;
            },
            _ => return false, // never ends with north
        }
    }
    true
}

fn fix_pipe(game: &mut Vec<Vec<Option<Segment>>>) -> (usize, usize, (Direction, Direction)) {
    let mut start: (usize, usize) = (0, 0);
    for row in 0..game.len() {
        for col in 0..game[row].len() {
            if game[row][col].is_some() {
                let (s, e) = game[row][col].as_ref().unwrap();
                // start position
                if *s == Direction::None && *e == Direction::None {
                    start = (row, col);
                    continue;
                }
                if !check_segment(game, row, col) {
                    game[row][col] = None;
                }
            }
        }
    }
    let (row, col) = start;
    let mut start = (Direction::None, Direction::None);
    if game[row][col].is_some() {
        // north
        if match_neighbour(game, row as i32 - 1, col as i32, Direction::South, None) {
            start = ( Direction::North, start.1 );
        }
        // west
        if match_neighbour(game, row as i32, col as i32 - 1, Direction::East, None) {
            start = if start.0 != Direction::None {
                ( start.0, Direction::West )
            } else {
                ( Direction::West, start.1 )
            }
        }
        // east
        if match_neighbour(game, row as i32, col as i32 + 1, Direction::West, None) {
            start = if start.0 != Direction::None {
                ( start.0, Direction::East )
            } else {
                ( Direction::East, start.1 )
            }
        }
        // south
        if match_neighbour(game, row as i32 + 1, col as i32, Direction::North, None) {
            start = ( start.0, Direction::South );
        }
    }
    (row, col, start)
}

fn print_game(game: &Vec<Vec<Option<Segment>>>) {
    for row in game.iter() {
        for segment in row.iter() {
            match segment {
                Some((Direction::North, Direction::South)) => print!("|"),
                Some((Direction::West, Direction::East)) => print!("-"),
                Some((Direction::North, Direction::East)) => print!("L"),
                Some((Direction::North, Direction::West)) => print!("J"),
                Some((Direction::West, Direction::South)) => print!("7"),
                Some((Direction::East, Direction::South)) => print!("F"),
                Some((Direction::North, Direction::North)) => print!("S"),
                None => print!("."),
                _ => panic!("invalid segment"),
            }
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut game = parse_game(input);
    //dbg!(&game);
    let (row, col, start) = fix_pipe(&mut game);
    game[row][col] = Some(start);
    print_game(&game);
    None
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
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
