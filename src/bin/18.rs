advent_of_code::solution!(18);

fn do_dig(instructions: impl Iterator<Item = (u8, u32)>) -> usize {
    let mut count: isize = 0;
    let (mut x, mut y) = (0isize, 0isize);
    for (dir, len) in instructions {
        let (px, py) = (x, y);
        match dir {
            0 => x += len as isize,
            1 => y += len as isize,
            2 => x -= len as isize,
            3 => y -= len as isize,
            _ => unreachable!(),
        }
        count += (x + px) * (y - py) + len  as isize;
    }
    ((count / 2) + 1) as usize
}

pub fn part_one(input: &str) -> Option<usize> {
    let queue = input.lines().map(|line| {
        let i = line.split_whitespace().collect::<Vec<_>>();
        (match i[0] {
            "R" => 0,
            "D" => 1,
            "L" => 2,
            "U" => 3,
            _ => unreachable!(),
        } as u8, i[1].parse::<u32>().unwrap())
    });
    Some(do_dig(queue))
}

pub fn part_two(input: &str) -> Option<usize> {
    let queue = input.lines().map(|line| {
        let (_, i) = line.split_once("#").unwrap();
        (i.as_bytes()[i.len()-2] - b'0',
        u32::from_str_radix(&i[0..i.len()-2], 16).unwrap())
    });
    Some(do_dig(queue))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
