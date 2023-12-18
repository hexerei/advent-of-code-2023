use std::str;

advent_of_code::solution!(18);

fn min_max_poly(poly: &Vec<(isize, isize)>) -> ((isize, isize), (isize, isize)) {
    let mut min_p = (isize::MAX, isize::MAX);
    let mut max_p = (0, 0);
    for p in poly {
        min_p.0 = min_p.0.min(p.0);
        min_p.1 = min_p.1.min(p.1);
        max_p.0 = max_p.0.max(p.0);
        max_p.1 = max_p.1.max(p.1);
    }
    (min_p, max_p)
}

fn normalize_poly(poly: &Vec<(isize, isize)>) -> Vec<(isize, isize)> {
    let (min_p, _) = min_max_poly(poly);
    poly.iter().map(|p| (p.0 - min_p.0, p.1 - min_p.1)).collect()
}

fn parse_poly(input: &str, all: bool) -> Vec<(isize, isize)> {
    let poly = input
        .lines()
        .fold(Vec::new(), |mut acc, line| {
            let dig = line.split_whitespace().collect::<Vec<_>>();
            let dug = *acc.last().unwrap_or(&(0, 0));
            if dig.len() > 1 {
                let l = dig[1].parse::<isize>().unwrap();
                let (x, y) = (dug.0, dug.1);
                if all {
                    // all points
                    match dig[0] {
                        "U" => (0..=l).for_each(|i| if !acc.contains(&(x,y+i)) {acc.push((x, y + i))}),
                        "D" => (0..=l).for_each(|i| if !acc.contains(&(x,y-i)) {acc.push((x, y - i))}),
                        "L" => (0..=l).for_each(|i| if !acc.contains(&(x-i,y)) {acc.push((x - i, y))}),
                        "R" => (0..=l).for_each(|i| if !acc.contains(&(x+i,y)) {acc.push((x + i, y))}),
                        _ => unreachable!(),
                    }
                } else {
                // only endpoints
                    match dig[0] {
                        "U" if !acc.contains(&(x,y+l)) => acc.push((x,y+l)),
                        "D" if !acc.contains(&(x,y-l)) => acc.push((x,y-l)),
                        "L" if !acc.contains(&(x-l,y)) => acc.push((x-l,y)),
                        "R" if !acc.contains(&(x+l,y)) => acc.push((x+l,y)),
                        _ => unreachable!()
                    }
                }
            }
            acc.dedup();
            acc
        });
        normalize_poly(&poly)
}

fn parse_poly2(input: &str, all: bool) -> Vec<(isize, isize)> {
    let poly = input
        .lines()
        .fold(Vec::new(), |mut acc, line| {
            let (_, hex) = line.split_once("#").unwrap();
            let (hex, _) = hex.split_once(")").unwrap();
            let mut chars = hex.chars();
            let mut direction = 0u8;
            if let Some(x) = chars.next_back() {
                match x {
                    '0'..='3' => direction = x.to_digit(16).unwrap() as u8,
                    _ => unreachable!()
                }
            }
            let l = isize::from_str_radix(chars.as_str(), 16).unwrap();
            println!("{} {}", match direction {
                0 => "R",
                1 => "D",
                2 => "L",
                3 => "U",
                _ => unreachable!()
            }, l);
            let (x, y) = *acc.last().unwrap_or(&(0, 0));
            if all {
                // all points
                match direction {
                    3 => (0..=l).for_each(|i| if !acc.contains(&(x,y+i)) {acc.push((x, y + i))}),
                    1 => (0..=l).for_each(|i| if !acc.contains(&(x,y-i)) {acc.push((x, y - i))}),
                    2 => (0..=l).for_each(|i| if !acc.contains(&(x-i,y)) {acc.push((x - i, y))}),
                    0 => (0..=l).for_each(|i| if !acc.contains(&(x+i,y)) {acc.push((x + i, y))}),
                    _ => unreachable!(),
                }
            } else {
            // only endpoints
                match direction {
                    3 if !acc.contains(&(x,y+l)) => acc.push((x,y+l)),
                    1 if !acc.contains(&(x,y-l)) => acc.push((x,y-l)),
                    2 if !acc.contains(&(x-l,y)) => acc.push((x-l,y)),
                    0 if !acc.contains(&(x+l,y)) => acc.push((x+l,y)),
                    _ => unreachable!()
                }
            }
            acc
        });
        normalize_poly(&poly)
}

fn print_poly(poly: &Vec<(isize, isize)>) {
    println!("*************** {}", poly.len());
    for p in poly {
        println!("({}, {})", p.0, p.1);
    }
    println!("*************** {}", poly.len());
    let (min_p, max_p) = min_max_poly(poly);
    for y in min_p.1..=max_p.1 {
        for x in min_p.0..=max_p.0 {
            if poly.contains(&(x, y)) {
                print!("#")
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn dig(poly: &Vec<(isize, isize)>) -> usize {
    // print_poly(poly);
    // println!("***************");
    let (min_p, max_p) = min_max_poly(poly);
    let mut count: usize = 0;
    for y in min_p.1..=max_p.1 {
        for x in min_p.0..=max_p.0 {
            if poly.contains(&(x, y)) || point_in_poly(poly, (x, y)) {
                count += 1;
            //     print!("#")
            // } else {
            //     print!(".");
            }
        }
        //println!();
    }
    count
}

fn point_in_poly(poly: &Vec<(isize, isize)>, point: (isize, isize)) -> bool {
    let mut inside = false;
    let mut j = poly.len() - 1;
    let (tx, ty) = (point.1, point.0);
    for i in 0..poly.len() {
        let (px, py) = (poly[i].1, poly[i].0);
        let (lpx, lpy) = (poly[j].1, poly[j].0);
        if ((py > ty) != (lpy > ty))
        && (tx < (lpx - px) * (ty - py) / (lpy - py) + px) {
            inside = !inside;
        }
        j = i;
    }
    inside
}

// fn points_in_poly(poly: &Vec<(i32, i32)>) -> usize {
//     let mut inside = false;
//     let mut j = poly.len() - 1;
//     let mut count: usize = 0;
//     let (tx, ty) = (point.1, point.0);
//     for i in 0..poly.len() {
//         let (px, py) = (poly[i].1, poly[i].0);
//         let (lpx, lpy) = (poly[j].1, poly[j].0);
//         if (py != lpy)
//         && (tx < (lpx - px) * (ty - py) / (lpy - py) + px) {
//             inside = !inside;
//         }
//         j = i;
//     }
//     count
// }

pub fn part_one(input: &str) -> Option<usize> {
    Some(dig(&parse_poly(input, true)))
}

pub fn part_two(input: &str) -> Option<usize> {
    let poly = parse_poly2(input, false);
    dbg!(&poly);
    let (min_p, max_p) = min_max_poly(&poly);
    println!("{:?} {:?}", min_p, max_p);

    //print_poly(&p);
    Some(0)
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
