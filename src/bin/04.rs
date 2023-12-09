advent_of_code::solution!(4);

fn parse_line(line: &str) -> (Vec<u32>, Vec<u32>) {
    let (_, line) = line.split_at(line.find(":").unwrap());
    let (first, last) = line.split_at(line.find("|").unwrap());
    (parse_numbers(first), parse_numbers(last))
}

fn parse_numbers(line: &str) -> Vec<u32> {
    let line = line.trim().replace("  ", " ");
    line.split_whitespace().filter_map(|n| {
        match n.trim().parse::<u32>() {
            Ok(n) => Some(n),
            Err(_) => None,
        }
    }).collect::<Vec<_>>()
}

fn score(winners: &Vec<u32>, numbers: &Vec<u32>) -> u32 {
    winners.iter().fold(0.5, |acc, n|
        acc * (numbers.iter().filter_map(|m| if m==n {Some(2)} else {None}).product::<u32>() as f64)
    ) as u32
}

fn count(winners: &Vec<u32>, numbers: &Vec<u32>) -> u32 {
    winners.iter().fold(0, |acc, n|
        acc + numbers.iter().filter(|&m| m==n).count()
    ) as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().filter_map(|line| {
        if line.starts_with("Card") {
            let (winners, numbers) = parse_line(line);
            Some(score(&winners, &numbers))
        } else {
            None
        }
    }).collect::<Vec<_>>().iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let scores = input.lines().filter_map(|line| {
        if line.starts_with("Card") {
            let (winners, numbers) = parse_line(line);
            Some(count(&winners, &numbers))
        } else {
            None
        }
    }).collect::<Vec<_>>();
    let mut cards = scores.iter().map(|_| 1u32).collect::<Vec<_>>();
    for index in 0..scores.len() {
        let score = scores[index] as usize;
        let lb = index + 1;
        if score > 0 {
            for _ in 0..cards[index] as usize {
                for i in lb..lb+score {
                    if i < cards.len() {
                        cards[i] += 1;
                    }
                }
            }
        }
    }
    Some(cards.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
