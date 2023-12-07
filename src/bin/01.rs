//use advent_of_code::template::ANSI_RESET;
use aho_corasick::AhoCorasick;
use regex::Regex;

advent_of_code::solution!(1);

// function to find first or last match from a list of patterns
// and return the match and the rest of the string
fn numbers_only(haystack: &str) -> String {
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
    ac.replace_all(haystack, replace_with)
}

fn match_at(haystack: &str, start: bool) -> String {
    let regex = if start {
        Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine).*$").unwrap()
    } else {
        Regex::new(r".*(\d|one|two|three|four|five|six|seven|eight|nine).*$").unwrap()
    };
    let result = regex.captures(haystack);

    let mut start = 0;
    let mut end = haystack.len()-1;

    match result {
        Some(caps) => {
            start = caps.get(1).unwrap().start();
            end = caps.get(1).unwrap().end();
        },
        None => ()
    };
    let matched = numbers_only(&haystack[start..end]);

    //println!("{:?}", matched);
    matched
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
        //println!("Line: {:?}", line);
        // get first digit
        let mut data = match_at(line, true);
        // get last digit
        data.push_str(&match_at(line, false));
        //println!("Data: {:?}", data);
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
        let result = part_one("1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet");
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(332));
    }
}
