advent_of_code::solution!(15);

const VERBOSE: bool = false;

// calculate holiday ASCII string helper algorithm
fn calc_hash(input: &str) -> u32 {
    input.as_bytes().iter().fold(0, |acc, &b| {
        ((acc + (b as u32)) * 17) % 256
    })
}

// calculate holiday ASCII string helper manual arrangement procedure
fn calc_hashmap(n: &str, boxes: &mut Vec<Vec<(String, u8)>>) {
    if n.contains("=") {
        let (label, value) = n.split_at(n.find("=").unwrap());
        let focal = value[1..].parse::<u8>().unwrap_or(0);
        let hash = calc_hash(label);
        let mut found = false;
        boxes[hash as usize].iter_mut().for_each(|(l, f)| {
            if l == label {
                found = true;
                *f = focal;
            }
        });
        if !found {
            boxes[hash as usize].push((label.to_string(), focal));
        }
    } else if n.contains("-") {
        let label = n[..n.len()-1].to_string();
        let hash = calc_hash(label.as_str());
        boxes[hash as usize].retain(|(l, _)| l != &label);
    }
    if VERBOSE {
        for (i, b) in boxes.iter().enumerate() {
            print_box(i, b);
        }
        println!();
    }
}

// get total focal power of all lenses in all boxes
fn get_focal_power(boxes: &Vec<Vec<(String, u8)>>) -> u32 {
    boxes.iter().enumerate().map(|(i, b)| {
        b.iter().enumerate().map(|(j, &(_, f))| 
            ((i+1) * (j+1) * f as usize) as u32
        ).sum::<u32>()
    }).sum()
}

fn print_box(index: usize, lenses: &Vec<(String, u8)>) {
    if lenses.len() == 0 { return; }
    print!("Box {}:", index);
    for (lens, focal) in lenses.iter() {
        print!(" [{} {}]", lens, focal);
    }
    println!();
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.replace("\n", "").split(",").map(|n|{
        calc_hash(n)
    }).collect::<Vec<_>>().iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut boxes: Vec<Vec<(String, u8)>> = vec![Vec::new(); 256];
    input.replace("\n", "").split(",").for_each(|n|{
        calc_hashmap(n, &mut boxes);
    });
    Some(get_focal_power(&boxes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
