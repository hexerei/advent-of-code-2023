use std::collections::HashMap;

advent_of_code::solution!(16);


const UP: u8 = 1;
const LEFT: u8 = 2;
const RIGHT: u8 = 4;
const DOWN: u8 = 8;

fn parse_grid(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| {
        line.as_bytes().iter().map(|&b| {
            b
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>()
}

fn run_beam(grid: &Vec<Vec<u8>>, x: usize, y: usize, direction: u8,
            path: &mut HashMap<(usize, usize), u8>) -> usize {
    let (mut x, mut y) = (x, y);
    let mut direction = direction;
    loop {
        if let Some(&d) = path.get(&(x, y)) {
            if d & direction > 0 { break; }
            path.insert((x, y), d | direction);
        } else {
            path.insert((x, y), direction);
        }
        match grid[y][x] {
            b'\\' => direction = match direction {
                RIGHT => DOWN,
                LEFT => UP,
                UP => LEFT,
                DOWN => RIGHT,
                _ => direction,
            },
            b'/' => direction = match direction {
                RIGHT => UP,
                LEFT => DOWN,
                UP => RIGHT,
                DOWN => LEFT,
                _ => direction,
            },
            b'|' => if direction == LEFT || direction == RIGHT {
                run_beam(grid, x, y, UP, path);
                run_beam(grid, x, y, DOWN, path);
                break;
            },
            b'-' => if direction == UP || direction == DOWN {
                run_beam(grid, x, y, LEFT, path);
                run_beam(grid, x, y, RIGHT, path);
                break;
            },
            _ => (),
        }
        match direction {
            UP => if y > 0 { y -= 1; } else { break; },
            LEFT => if x > 0 { x -= 1; } else { break; },
            RIGHT => if x < grid[y].len() - 1 { x += 1; } else { break; },
            DOWN => if y < grid.len() - 1 { y += 1; } else { break;},
            _ => unreachable!(),
        }
    }
    path.len()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let mut path: HashMap<(usize, usize), u8> = HashMap::new();
    Some(run_beam(&grid, 0, 0, RIGHT, &mut path) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let mut results: Vec<usize> = Vec::new();
    let mut path: HashMap<(usize, usize), u8>;
    for y in 0..grid.len() {
        path = HashMap::new();
        results.push(run_beam(&grid, 0, y, RIGHT, &mut path));
        path = HashMap::new();
        results.push(run_beam(&grid, grid[0].len() - 1, y, LEFT, &mut path));
    }
    for x in 0..grid[0].len() {
        path = HashMap::new();
        results.push(run_beam(&grid, x, 0, DOWN, &mut path));
        path = HashMap::new();
        results.push(run_beam(&grid, x, grid.len() - 1, UP, &mut path));
    }
    Some(*results.iter().max().unwrap() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
