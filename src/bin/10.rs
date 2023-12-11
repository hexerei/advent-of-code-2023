advent_of_code::solution!(10);

// .....
// .S-7.
// .|.|.
// .L-J.
// .....

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    None,
    North,
    East,
    South,
    West,
}
impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            _ => Direction::None,
        }
    }
    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
            _ => (0, 0),
        }
    }
}
impl Default for Direction {
    fn default() -> Self { Direction::None }
}
type Segment = (Direction, Direction);

fn parse_game(input: &str) -> (Vec<Vec<Option<Segment>>>, (usize, usize)) {
    let mut start: (usize, usize) = (0,0);
    (input.lines().enumerate().map(|(row, line)| {
        line.chars().enumerate().map(|(col, c)| {
            match c {
                '|' => Some((Direction::North, Direction::South)),
                '-' => Some((Direction::West, Direction::East)),
                'L' => Some((Direction::North, Direction::East)),
                'J' => Some((Direction::North, Direction::West)),
                '7' => Some((Direction::West, Direction::South)),
                'F' => Some((Direction::East, Direction::South)),
                'S' => {
                    start = (row, col);
                    Some((Direction::None, Direction::None))
                },
                _ => None,
            }
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>(), start)
}

fn match_neighbour(game: &Vec<Vec<Option<Segment>>>, row: i32, col: i32, dir: Direction) -> bool {
    if row >= 0 && row < game.len()  as i32
    && col >= 0 && col < game[row as usize].len() as i32 {
        let segment = game[row as usize][col as usize].as_ref();
        if segment.is_some() {
            let (s, e) = segment.unwrap();
            if (*s == Direction::None && *e == Direction::None) // start position is joker
            || (*s == dir || *e == dir) {
                // println!("match_neighbour ({}, {}) - {:?} = TRUE", row, col, dir);
                return true;
            }
        }
    }
    // println!("match_neighbour ({}, {}) - {:?} = FALSE", row, col, dir);
    false
}

fn check_segment(game: &Vec<Vec<Option<Segment>>>, row: usize, col: usize) -> bool {
    let segment = game[row][col].as_ref();
    if segment.is_some() {
        let (s, e) = segment.unwrap();
        let (sr, sc) = s.offset();
        if !match_neighbour(game, row as i32 + sr, col as i32 + sc, s.opposite()) {
            return false;
        }
        let (er, ec) = e.offset();
        if !match_neighbour(game, row as i32 + er, col as i32 + ec, e.opposite()) {
            return false;
        }
    }
    true
}

fn fix_pipe(game: &mut Vec<Vec<Option<Segment>>>) -> (usize, usize) {
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
    start
 }

 fn map_pipe(game: &Vec<Vec<Option<Segment>>>, poly: &Vec<(usize, usize)>) -> Vec<Vec<Option<Segment>>> {
    let mut new_game: Vec<Vec<Option<Segment>>> = vec![
        vec![None; poly.iter().fold(0, |acc, &(_, col)| acc.max(col)) + 1];
        poly.iter().fold(0, |acc, &(row, _)| acc.max(row)) + 1
    ];
    for &(row, col) in poly.iter() {
        new_game[row][col] = game[row][col];
    }
    new_game
 }

fn fix_start(game: &Vec<Vec<Option<Segment>>>, row: usize, col: usize) -> (Direction, Direction) {
    let mut start = (Direction::None, Direction::None);
    if game[row][col].is_some() {
        // north
        if match_neighbour(game, row as i32 - 1, col as i32, Direction::South) {
            start = ( Direction::North, start.1 );
        }
        // west
        if match_neighbour(game, row as i32, col as i32 - 1, Direction::East) {
            start = if start.0 != Direction::None {
                ( start.0, Direction::West )
            } else {
                ( Direction::West, start.1 )
            }
        }
        // east
        if match_neighbour(game, row as i32, col as i32 + 1, Direction::West) {
            start = if start.0 != Direction::None {
                ( start.0, Direction::East )
            } else {
                ( Direction::East, start.1 )
            }
        }
        // south
        if match_neighbour(game, row as i32 + 1, col as i32, Direction::North) {
            start = ( start.0, Direction::South );
        }
    }
    start
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

fn print_pipe(points: &Vec<(usize, usize)>) {
    let max_row = points.iter().fold(0, |acc, &(row, _)| acc.max(row));
    let max_col = points.iter().fold(0, |acc, &(_, col)| acc.max(col));
    for row in 0..max_row + 1 {
        for col in 0..max_col + 1 {
            print!("{}", if points.contains(&(row, col)) { '#' } else { '.' });
        }
        println!();
    }
}

fn walk_pipe(game: &Vec<Vec<Option<Segment>>>, start: (usize, usize)) -> (usize, Vec<(usize, usize)>) {
    let mut steps: usize = 0;
    let mut poly: Vec<(usize, usize)> = vec![start];
    let mut next = game[start.0][start.1].unwrap().0;
    let mut row = start.0;
    let mut col = start.1;
    loop {
        let (sr, sc) = next.offset();
        row = (row as i32 + sr) as usize;
        col = (col as i32 + sc) as usize;
        let (s, e) = game[row][col].unwrap();
        next = if s == next.opposite() { e } else { s };
        steps += 1;
        if (row, col) == start {
            break;
        }
        poly.push((row, col));
    }
    (steps, poly)
 }

fn point_in_poly(poly: &Vec<(usize, usize)>, point: (usize, usize)) -> bool {
    let mut inside = false;
    let mut j = poly.len() - 1;
    let (tx, ty) = (point.1 as i32, point.0 as i32);
    for i in 0..poly.len() {
        let (px, py) = (poly[i].1 as i32, poly[i].0 as i32);
        let (lpx, lpy) = (poly[j].1 as i32, poly[j].0 as i32);
        if ((py > ty) != (lpy > ty))
        && (tx < (lpx - px) * (ty - py) / (lpy - py) + px) {
            inside = !inside;
        }
        j = i;
    }
    inside
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut game, (row, col)) = parse_game(input);
    game[row][col] = Some(fix_start(&game, row, col));
    let (steps, _) = walk_pipe(&game, (row, col));
    Some((steps/2) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut game, (row, col)) = parse_game(input);
    game[row][col] = Some(fix_start(&game, row, col));
    let (_, poly) = walk_pipe(&game, (row, col));
    let game = map_pipe(&game, &poly);
    //print_game(&game);
    let checked = game.iter().enumerate().map(|(row, line)|
        line.iter().enumerate().filter_map(|(col, segment)|
            match segment {
                Some(_) => None,
                None => if point_in_poly(&poly, (row, col)) { Some(1) } else { None }
            }
        ).collect::<Vec<_>>()
    ).flatten().collect::<Vec<_>>();
    //åprintln!("{:?}\n{:?}", poly, checked);
    Some(checked.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one("..F7.
.FJ|.
SJ.L7
|F--J
LJ...");
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
