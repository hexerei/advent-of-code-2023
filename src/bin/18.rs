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

fn fmin_max_poly(poly: &Vec<(f64, f64)>) -> ((f64, f64), (f64, f64)) {
    let mut min_p = (f64::MAX, f64::MAX);
    let mut max_p = (f64::MIN, f64::MIN);
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

fn downscale_poly(poly: &Vec<(isize, isize)>, d: isize) -> Vec<(f64, f64)> {
    poly.iter().map(|p| (p.0 as f64 / d as f64, p.1 as f64 / d as f64)).collect()
}

fn parse_poly(input: &str, all: bool) -> Vec<(isize, isize)> {
    let poly = input
        .lines()
        .fold(vec![(0,0)], |mut acc, line| {
            let dig = line.split_whitespace().collect::<Vec<_>>();
            let dug = *acc.last().unwrap_or(&(0, 0));
            if dig.len() > 1 {
                let l = dig[1].parse::<isize>().unwrap();
                let (x, y) = (dug.0, dug.1);
                if all {
                    // all points
                    match dig[0] {
                        "U" => (0..=l).for_each(|i| acc.push((x, y + i))),
                        "D" => (0..=l).for_each(|i| acc.push((x, y - i))),
                        "L" => (0..=l).for_each(|i| acc.push((x - i, y))),
                        "R" => (0..=l).for_each(|i| acc.push((x + i, y))),
                        _ => unreachable!(),
                    }
                } else {
                // only endpoints
                    match dig[0] {
                        "U" => acc.push((x,y+l)),
                        "D" => acc.push((x,y-l)),
                        "L" => acc.push((x-l,y)),
                        "R" => acc.push((x+l,y)),
                        _ => ()
                    }
                }
            }
            acc
        });
        normalize_poly(&poly)
}

fn parse_poly2(input: &str) -> (Vec<(isize, isize)>, isize) {
    let mut nums: Vec<isize> = Vec::new();
    let poly = input.lines()
        .fold(vec![(0,0)], |mut acc, line| {
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
            nums.push(l);
            println!("{} {}", match direction {
                0 => "R",
                1 => "D",
                2 => "L",
                3 => "U",
                _ => unreachable!()
            }, l);
            let (x, y) = *acc.last().unwrap_or(&(0, 0));
            // only endpoints
            match direction {
                3 => acc.push((x,y+l)),
                1 => acc.push((x,y-l)),
                2 => acc.push((x-l,y)),
                0 => acc.push((x+l,y)),
                _ => unreachable!()
            }
            acc
        });
    let d = *nums.iter().min().unwrap_or(&1);
    for num in nums {
        println!("{} / {} = {}", num, d, num as f64/d as f64);
    }
    (normalize_poly(&poly), d)
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
    let (min_p, max_p) = min_max_poly(poly);
    let mut count: usize = 0;
    for y in min_p.1..=max_p.1 {
        for x in min_p.0..=max_p.0 {
            if poly.contains(&(x, y)) || point_in_poly(poly, (x, y)) {
                count += 1;
            }
        }
    }
    count
}

fn fdig(poly: &Vec<(f64, f64)>) -> usize {
    let (min_p, max_p) = fmin_max_poly(poly);
    let mut count: usize = 0;
    let (xs, xe) = (min_p.0.floor() as isize, max_p.0.ceil() as isize);
    let (ys, ye) = (min_p.1.floor() as isize, max_p.1.ceil() as isize);
    for y in ys..=ye {
        for x in xs..=xe {
            if fpoint_in_poly(poly, (x as f64, y as f64)) {
                count += 1;
            }
        }
    }
    count
}

fn point_in_poly(poly: &Vec<(isize, isize)>, point: (isize, isize)) -> bool {
    let mut inside = false;
    let mut j = poly.len() - 1;
    let (tx, ty) = point; //(point.1, point.0);
    for i in 0..poly.len() {
        let (px, py) = poly[i]; //(poly[i].1, poly[i].0);
        let (lpx, lpy) = poly[j]; //(poly[j].1, poly[j].0);
        if ((py > ty) != (lpy > ty))
        && (tx < (lpx - px) * (ty - py) / (lpy - py) + px) {
            inside = !inside;
        }
        j = i;
    }
    inside
}

fn fpoint_in_poly(poly: &Vec<(f64, f64)>, point: (f64, f64)) -> bool {
    let mut inside = false;
    let mut j = poly.len() - 1;
    let (tx, ty) = point; //(point.1, point.0);
    for i in 0..poly.len() {
        let (px, py) = poly[i]; //(poly[i].1, poly[i].0);
        let (lpx, lpy) = poly[j]; //(poly[j].1, poly[j].0);
        if ((py > ty) != (lpy > ty))
        && (tx < (lpx - px) * (ty - py) / (lpy - py) + px) {
            inside = !inside;
        }
        j = i;
    }
    inside
}

fn gcd2(mut a:isize, mut b:isize) -> isize{
    if a==b { return a; }
    if b > a { std::mem::swap(&mut a, &mut b); }
    while b>0 {
        let temp = a;
        a = b;
        b = temp%b;
    }
    return a;
}

fn gcd(a:isize, b:isize) -> isize {
    if a == 0 || b == 0 { return  (a | b).abs(); }
    // common factors of 2
    let shift = (a | b).trailing_zeros() as isize;
    if a == isize::MIN || b == isize::MIN {
        return (1isize << shift).abs();
    }
    let (mut m, mut n) = (a.abs(), b.abs());
    m >>= m.trailing_zeros();
    n >>= n.trailing_zeros();
    while m != n {
        if m > n {
            m -= n;
            m >>= m.trailing_zeros();
        } else {
            n -= m;
            n >>= n.trailing_zeros();
        }
    }
    m << shift
}

fn points_in_poly(poly: &Vec<(isize, isize)>) -> usize {
    let mut inside = 0;
    let mut j = poly.len() - 1;
    for i in 0..poly.len() {
        let (px, py) = poly[i]; //(poly[i].1, poly[i].0);
        let (lpx, lpy) = poly[j]; //(poly[j].1, poly[j].0);
        let dx = lpx.abs_diff(px) as isize;
        let dy = lpy.abs_diff(py) as isize;
        println!("({:8}, {:8}) ({:8}, {:8}) [{:8}] ({}) * {} / {} + {} = {}",
            px, py, lpx, lpy, dx.abs_diff(dy), dx, py, dy, dx,
            match dy + px {
                0 => 0isize,
                _ => (dx * py) / (dy + px)
            });
        inside += dx + dy;
        // if ((py > ty) != (lpy > ty))
        // && (tx < (lpx - px) * (ty - py) / (lpy - py) + px) {
        //     inside = !inside;
        // }
        j = i;
    }
    inside as usize
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(dig(&parse_poly(input, true)))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (poly, d) = parse_poly2(input);
    let poly = downscale_poly(&poly, d);

    for p in &poly {
        println!("({}, {})", p.0, p.1);
    }
    //dbg!(&poly);
    // let (min_p, max_p) = min_max_poly(&poly);
    // println!("{:?} {:?}", min_p, max_p);


    // //print_poly(&p);
    Some(fdig(&poly) * d as usize)
    //None
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
