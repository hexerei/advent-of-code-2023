advent_of_code::solution!(13);

fn get_hv_maps(map: &Vec<String>) -> (Vec<isize>, Vec<isize>) {
    let h_map = map.iter().map(|row| {
        isize::from_str_radix(row, 2).unwrap()
    }).collect::<Vec<_>>();
    let v_map = (0..map[0].len()).map(|i| {
        isize::from_str_radix(map.iter().map(|row| {
            row.chars().nth(i).unwrap()
        }).collect::<String>().as_str(), 2).unwrap()
    }).collect::<Vec<_>>();
    (h_map, v_map)
}



fn compare(a: isize, b: isize, part2: bool) -> bool {
    if a == b { return true; }
    else if part2 {
        // let mask = a ^ b;
        // return mask.count_ones() < 2;
        return (a ^ b).count_ones() < 2;
    }
    false
}

fn get_mirror_pos(map: Vec<isize>, part2: bool) -> usize {
    // let mut pos = 0;
    // for i in 1..map.len() {
    //     if map[i] == map[i-1] {
    //         pos = i;
    //         break;
    //     }
    // }
    let pos = (1..map.len()).filter_map(|i| {
        if compare(map[i], map[i-1], part2) {
        //if map[i] == map[i-1] {
            Some(i)
        } else {
            None
        }
        // if map[i] == map[i-1] {
        //     Some(i)
        // } else if part2 {
        //     let width = format!("{:b}", map[i]).len().max(format!("{:b}", map[i-1]).len());
        //     for j in 0..width {
        //         let mask = 1 << j;
        //         if map[i] ^ map[i-1] == mask {
        //             return Some(i);
        //         }
        //     }
        //     None
        // } else {
        //     None
        // }
    }).collect::<Vec<_>>();
    for p in pos.iter() { //.iter().rev() {
        let (l, r) = map.split_at(*p);
        let count = l.iter().rev().zip(r.iter()).filter(|&(l, r)| !compare(*l, *r, part2)).count();
        //println!("{:?} <> {:?} ./. {}", l, r, count);
        if count == 0 {
            return *p;
        }
    }
    0
    //println!("{:?} {:?}", l, r);
    // if l.iter().rev().zip(r.iter()).filter(|(l, r)| l != r).count() == 0 {
    //     pos
    // } else {
    //     0
    // }
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
        if h > 0 { return h * 100; }
        get_mirror_pos((0..map[0].len()).map(|i| {
            isize::from_str_radix(map.iter().map(|row| {
                row.chars().nth(i).unwrap()
            }).collect::<String>().as_str(), 2).unwrap()
        }).collect::<Vec<_>>(), part2)
    }).collect::<Vec<_>>().iter().sum()
}

fn eval_maps(input: &str, part2: bool) -> usize {
    let maps = input.split("\n\n").map(|map| {
        map.lines().map(|line| {
            line.chars().map(|c| { match c { '#' => '1', _ => '0' }}).collect::<String>()
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    maps.iter().map(|map| {
        let h_map = map.iter().map(|row| {
            isize::from_str_radix(row, 2).unwrap()
        }).collect::<Vec<_>>();
        let h = get_mirror_pos(h_map, part2);
        if h > 0 {
            return h * 100;
        }
        let v_map = (0..map[0].len()).map(|i| {
            isize::from_str_radix(map.iter().map(|row| {
                row.chars().nth(i).unwrap()
            }).collect::<String>().as_str(), 2).unwrap()
        }).collect::<Vec<_>>();
        get_mirror_pos(v_map, part2)
    }).collect::<Vec<_>>().iter().sum()
}
fn debug_map(map: &Vec<isize>) {
    let width = format!("{:b}", map.iter().max().unwrap()).len();
    for i in 0..map.len() {
        if i > 0  {
            println!("{:0width$b} ^ {:0width$b}", map[i], map[i] ^ map[i-1]);
        } else {
            println!("{:0width$b}", map[i]);
        }
    }
    println!("{}", "-".repeat(width));
}
fn eval_hv_maps(h_map: &Vec<isize>, v_map: &Vec<isize>, part2: bool) -> usize {
    //let mut sum = 0;
    let h = get_mirror_pos(h_map.clone(), part2);
    if h > 0 {
        // let width = format!("{:b}", h_map.iter().max().unwrap()).len();
        // for i in 0..h_map.len() {
        //     if i > 0  {
        //         if i == h {
        //             println!("{}", ".".repeat(width));
        //         }
        //         println!("{:0width$b} ^ {:0width$b}", h_map[i], h_map[i] ^ h_map[i-1]);
        //     } else {
        //         println!("{:0width$b}", h_map[i]);
        //     }
        // }
        // println!("{}\n{}\n{}", "-".repeat(width), h * 100, "=".repeat(width));
        return h * 100;
        //sum += h * 100;
    }
    let width = format!("{:b}", h_map.iter().max().unwrap()).len();
    for i in 0..h_map.len() {
        if i > 0  {
            if i == h {
                println!("{}", ".".repeat(width));
            }
            println!("{:0width$b} ^ {:0width$b}", h_map[i], h_map[i] ^ h_map[i-1]);
        } else {
            println!("{:0width$b}", h_map[i]);
        }
    }
    println!("{}", "-".repeat(width));
    let v = get_mirror_pos(v_map.clone(), part2);
    if v > 0 {
        let width = format!("{:b}", v_map.iter().max().unwrap()).len();
        for i in 0..v_map.len() {
            if i > 0  {
                if i == v {
                    println!("{}", ".".repeat(width));
                }
                println!("{:0width$b} ^ {:0width$b}", v_map[i], v_map[i] ^ v_map[i-1]);
            } else {
                println!("{:0width$b}", v_map[i]);
            }
        }
        println!("{}\n{}\n{}", "-".repeat(width), v, "=".repeat(width));
    } else {
        println!("*** ZERO *** ZERO *** ZERO *** ZERO *** ZERO *** ZERO {} {}", h, v);
    }
    //sum + v
    v
}

fn parse_game(input: &str) -> Vec<Vec<String>> {
    let maps = input.split("\n\n").map(|map| {
        map.lines().map(|line| {
            line.chars().map(|c| { match c { '#' => '1', _ => '0' }}).collect::<String>()
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    maps
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(eval_maps(input, false))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(eval_maps(input, true))
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
