advent_of_code::solution!(13);

fn compare(a: isize, b: isize, part2: bool, mode: &mut u8) -> bool {
    return if !part2 {
        a == b
    } else {
        match (a ^ b).count_ones() {
            0 => true,
            1 => match *mode {
                2 => { *mode = 1; true },
                0 => true,
                _ => false,
            },
            _ => false,
        }
    };
}

// fn get_mirror_pos2(map: Vec<isize>, part2: bool) -> usize {
//     let mut fix = 0;
//     for i in 1..map.len() {
//         if compare(map[i], map[i-1], part2, &mut fix) {
//             let (l, r) = map.split_at(i);
//             fix = if part2 {2} else {0};
//             if l.iter().rev().zip(r.iter()).filter(|&(l, r)| !compare(*l, *r, part2, &mut fix)).count() == 0 {
//                 if part2 { if fix == 1 { return i; } } else { return i; };
//             }
//             // if part2 && count == 0 && fix == 1 {
//             //     return i;
//             // } else if count == 0 {
//             //     return i;
//             // }
//         }
//     }
//     // (1..map.len()).for_each(|i| {
//     //     if compare(map[i], map[i-1], part2, &mut fix) {
//     //         let (l, r) = map.split_at(i);
//     //         fix = if part2 {2} else {0};
//     //         let count = l.iter().rev().zip(r.iter()).filter(|&(l, r)| !compare(*l, *r, part2, &mut fix)).count();
//     //         if part2 && count == 0 && fix == 1 {
//     //             return i;
//     //         } else if count == 0 {
//     //             return i;
//     //         }
//     //     }
//     // });
//     0
// }

fn get_mirror_pos(map: Vec<isize>, part2: bool) -> usize {
    let mut fix = 0;
    let pos = (1..map.len()).filter_map(|i| {
        if compare(map[i], map[i-1], part2, &mut fix) {
            Some(i)
        } else {
            None
        }
    }).collect::<Vec<_>>();
    //println!("POS {:?}", pos);
    for p in pos.iter() {
        let (l, r) = map.split_at(*p);
        fix = if part2 {2} else {0};
        let count = l.iter().rev().zip(r.iter()).filter(|&(l, r)| !compare(*l, *r, part2, &mut fix)).count();
        if part2 {
            if count == 0 && fix == 1 {
                //debug_map(&map, *p);
                return *p;
            }
        } else if count == 0 {
            return *p;
        }
    }
    0
}

fn eval(input: &str, part2: bool) -> usize {
    let maps = input.split("\n\n").map(|map| {
        map.lines().map(|line| {
            line.chars().map(|c| { match c { '#' => '1', _ => '0' }}).collect::<String>()
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    maps.iter().map(|map| {
        let h = get_mirror_pos(map.iter().map(|row| {
            isize::from_str_radix(row, 2).unwrap()
        }).collect::<Vec<_>>(), part2);
        if h > 0 {
            //if part2 {println!("{} {}", "-".repeat(40), h*100);}
            return h * 100;
        } /*else if part2 {
            for row in map.iter() {
                println!("{}", row);
            }
            println!("{}", "-".repeat(40));
        }*/
        let v = get_mirror_pos((0..map[0].len()).map(|i| {
            isize::from_str_radix(map.iter().map(|row| {
                row.chars().nth(i).unwrap()
            }).collect::<String>().as_str(), 2).unwrap()
        }).collect::<Vec<_>>(), part2);
        //if part2 {println!("{} {}", "-".repeat(36), v);}
        v
    }).collect::<Vec<_>>().iter().sum()
}

// fn debug_map(map: &Vec<isize>, pos: usize) {
//     let width = format!("{:b}", map.iter().max().unwrap()).len();
//     for i in 0..map.len() {
//         if i > 0  {
//             if i == pos {
//                 println!("{}", ".".repeat(width));
//             }
//             println!("{:0width$b} ^ {:0width$b}", map[i], map[i] ^ map[i-1]);
//         } else {
//             println!("{:0width$b}", map[i]);
//         }
//     }
// }

pub fn part_one(input: &str) -> Option<usize> {
    Some(eval(input, false))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(eval(input, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
