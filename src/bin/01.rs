use aho_corasick::AhoCorasick;

advent_of_code::solution!(1);

fn replace_spelled_numbers(haystack: &str) -> String {
    let patterns = &[
        "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine"
    ];
    let replace_with = &[
        "0", "1", "2", "3", "4",
        "5", "6", "7", "8", "9"
    ];
    let ac = AhoCorasick::new(patterns).unwrap();
    ac.replace_all(haystack, replace_with)
}

fn spell_out_numbers(haystack: &str) -> String {
    let mut data = String::new();
    let mut chars = haystack.chars();
    let mut oc = chars.next();
    while let Some(c) = oc {
        match c {
            'e' => println!("8"),
            'f' => println!("4|5"),
            'n' => println!("9"),
            'o' => println!("1"),
            's' => println!("6|7"),
            't' => println!("2|3"),
            'z' => println!("0"),
            _ => ()
        }
        data.push(c);
      println!("c: {c}");
      oc = chars.next();
    }
    data
}

pub fn part_one(input: &str) -> Option<u32> {
    Some( input.lines().map(|line| {
        // get digits only
        let mut data = String::from(line);
        data.retain(|c| r#"0123456789"#.contains(c));
        // collect first and last digit
        let mut number_str = String::from(data.chars().next().unwrap());
        number_str.push_str(&data.chars().last().unwrap().to_string());
        // println!("Line: {:?}", number_str);
        // convert to u32
        let val = number_str.parse::<u32>();
        if val.is_err() {
            0
        } else {
            val.unwrap()
        }
    }).sum() )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some( input.lines().map(|line| {
        // get digits only
        let mut data = spell_out_numbers(line);
        // println!("Data: {data}");
        data.retain(|c| r#"0123456789"#.contains(c));
        // collect first and last digit
        let mut number_str = String::from(data.chars().next().unwrap());
        number_str.push_str(&data.chars().last().unwrap().to_string());
        //println!("Line: {:?}", number_str);
        // convert to u32
        let val = number_str.parse::<u32>();
        if val.is_err() {
            0
        } else {
            val.unwrap()
        }
    }).sum() )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
