//use advent_of_code::template::ANSI_RESET;
use aho_corasick::AhoCorasick;

advent_of_code::solution!(1);

// fn replace_spelled_numbers(haystack: &str) -> String {
//     let patterns = &[
//         "zero", "one", "two", "three", "four",
//         "five", "six", "seven", "eight", "nine"
//     ];
//     let replace_with = &[
//         "0", "1", "2", "3", "4",
//         "5", "6", "7", "8", "9"
//     ];
//     let ac = AhoCorasick::new(patterns).unwrap();
//     ac.replace_all(haystack, replace_with)
// }

// function to find first match from a list of patterns
// and return the match and the rest of the string
fn replace_at(haystack: &str, start: bool) -> String {
    let patterns = &[
        "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine",
        "0", "1", "2", "3", "4",
        "5", "6", "7", "8", "9"
    ];
    let replace_with = &[
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"
    ];
    let ac = AhoCorasick::new(patterns).unwrap();
    let mut result = String::new();
    for mat in ac.find_iter(haystack) {
        println!("Match: {:?}", mat);
        result = String::from(replace_with[mat.pattern()]);
        if start {
            break;
        }
    }
    result
}

fn replace_at_start(haystack: &str) -> (String, String) {
    let data = String::from(haystack);
    let mut result = String::new();
    let mut rest = String::new();
    let offset: usize;
    if data.starts_with("one") {
        result.push_str("1");
        offset = 3;
    } else if data.starts_with("two") {
        result.push_str("2");
        offset = 3;
    } else if data.starts_with("three") {
        result.push_str("3");
        offset = 5;
    } else if data.starts_with("four") {
        result.push_str("4");
        offset = 4;
    } else if data.starts_with("five") {
        result.push_str("5");
        offset = 4;
    } else if data.starts_with("six") {
        result.push_str("6");
        offset = 3;
    } else if data.starts_with("seven") {
        result.push_str("7");
        offset = 5;
    } else if data.starts_with("eight") {
        result.push_str("8");
        offset = 5;
    } else if data.starts_with("nine") {
        result.push_str("9");
        offset = 4;
    } else if data.starts_with("zero") {
        result.push_str("0");
        offset = 4;
    } else {
        result.push_str(&data[0..1]);
        offset = 1;
    }
    if data.len() > offset {
        rest.push_str(&data[offset..]);
    }
    //println!("Data: {:?} -> {:?} + {:?}", data, result, rest);
    ( result, rest )
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
        println!("Line: {:?}", line);
        // get first digit
        let mut data = replace_at(line, true);
        // get last digit
        data.push_str(&replace_at(line, false));
        println!("Data: {:?}", data);
        // convert to u32
        let val = data.parse::<u32>();
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
