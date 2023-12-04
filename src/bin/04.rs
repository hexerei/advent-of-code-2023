advent_of_code::solution!(4);

fn parse_line(line: &str) -> (Vec<u32>, Vec<u32>) {
    // get first and last part of line
    let (_, line) = line.split_at(line.find(":").unwrap());
    let (first, last) = line.split_at(line.find("|").unwrap());
    //println!("First: {:?}\nLast: {:?}", first, last);
    let winners = parse_numbers(first);
    let numbers = parse_numbers(last);
    //println!("Winners: {:?}\nNumbers: {:?}", winners, numbers);
    (winners, numbers)
}
fn parse_numbers(line: &str) -> Vec<u32> {
    let mut numbers = Vec::new();
    let line = line.trim().replace("  ", " ");
    let line = line.split(" ");
    for n in line {
        let n = n.trim();
        let n = n.parse::<u32>();
        if n.is_err() {
            continue;
        }
        numbers.push(n.unwrap());
    }
    numbers
}
fn score(winners: &Vec<u32>, numbers: &Vec<u32>) -> u32 {
    let mut score: f32 = 0.5;
    for winner in winners {
        for number in numbers {
            if winner == number {
                score *= 2.;
            }
        }
    }
    if score == 0.5 {
        score = 0.;
    }
    //println!("Score: {}", score as u32);
    score as u32
}
fn count(winners: &Vec<u32>, numbers: &Vec<u32>) -> u32 {
    let mut score: u32 = 0;
    for winner in winners {
        for number in numbers {
            if winner == number {
                score += 1;
            }
        }
    }
    score
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut scores = Vec::new();
    for line in input.lines() {
        if line.starts_with("Card") {
            let (winners, numbers) = parse_line(line);
            let score = score(&winners, &numbers);
            scores.push(score);
        }
    }
    Some(scores.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut scores = Vec::new();
    let mut total_cards: u32 = 0;
    for line in input.lines() {
        if line.starts_with("Card") {
            let (winners, numbers) = parse_line(line);
            let score = count(&winners, &numbers);
            scores.push(score);
            total_cards += 1;
        }
    }
    println!("Scores: {:?} = {} cards", scores, total_cards);
    let mut cards = Vec::with_capacity(scores.len());
    for _ in 0..scores.len() {
        cards.push(1);
    }
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
    //println!("Cards: {:?}", cards);
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
