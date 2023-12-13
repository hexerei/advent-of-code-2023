advent_of_code::solution!(13);

fn get_hv_maps(map: &Vec<String>) -> (Vec<isize>, Vec<isize>) {
    let h_map = map.iter().map(|row| {
        isize::from_str_radix(row, 2).unwrap()
    }).collect::<Vec<_>>();
    let v_map = (0..map[0].len()).map(|i| {
        let mut v = 0;
        for j in 0..map.len() {
            v <<= 1;
            if map[j].chars().nth(i).unwrap() == '1' {
                v |= 1;
            }
        }
        v
    }).collect::<Vec<_>>();
    (h_map, v_map)
}

fn compare(a: isize, b: isize, part2: bool) -> bool {
    if a == b { return true; }
    else if part2 {
        let mask = a ^ b;
        return mask.count_ones() == 1;
    }
    false
}

fn get_mirror_pos(map: &Vec<isize>, part2: bool) -> usize {
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
fn eval_hv_maps(h_map: &Vec<isize>, v_map: &Vec<isize>, part2: bool) -> usize {
    let h = get_mirror_pos(h_map, part2);
    if h > 0 {
        let width = format!("{:b}", h_map.iter().max().unwrap()).len();
        for i in 0..h_map.len() {
            if i > 0  {
                println!("{:0width$b} ^ {:0width$b}", h_map[i], h_map[i] ^ h_map[i-1]);
            } else {
                println!("{:0width$b}", h_map[i]);
            }
        }
        return h * 100;
    }
    let v = get_mirror_pos(v_map, part2);
    if v > 0 {
        let width = format!("{:b}", v_map.iter().max().unwrap()).len();
        for i in 0..v_map.len() {
            if i > 0  {
                println!("{:0width$b} ^ {:0width$b}", v_map[i], v_map[i] ^ v_map[i-1]);
            } else {
                println!("{:0width$b}", v_map[i]);
            }
        }
    }
    //println!("{} {}", h, v);
    v
}

fn parse_game(input: &str) -> Vec<Vec<String>> {
    let mut maps = Vec::new();
    let mut map = Vec::new();
    input.lines().for_each(|line| {
        if line.is_empty() {
            maps.push(map.clone());
            map = Vec::new();
        } else {
            map.push(line.chars().map(|c| { match c { '#' => '1', _ => '0' }}).collect::<String>());
        }
    });
    if map.len() > 0 { maps.push(map); }
    maps
}

pub fn part_one(input: &str) -> Option<usize> {
    let maps = parse_game(input);
    Some(maps.iter().map(|map| {
        let (h_map, v_map) = get_hv_maps(map);
        eval_hv_maps(&h_map, &v_map, false)
    }).collect::<Vec<_>>().iter().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let maps = parse_game(input);
    let sum: usize = maps.iter().map(|map| {
        let (h_map, v_map) = get_hv_maps(map);
        let (row, col) = (map.len(), map[0].len());
        for row in map {
            println!("{}", row);
        }
        println!("{}", "-".repeat(16));
        let score = eval_hv_maps(&h_map, &v_map, true);
        println!("{}", "-".repeat(16));
        let h_mirror = get_mirror_pos(&h_map, true);
        let v_mirror = get_mirror_pos(&v_map, true);
        println!("{} {}", h_mirror, v_mirror);
        println!("{}", "=".repeat(16));
        score
    }).collect::<Vec<_>>().iter().sum();
    println!("RESULT {}", sum);
    Some(sum)
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
        assert_eq!(result, None);
    }
}
