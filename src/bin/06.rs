
advent_of_code::solution!(6);

fn wins(t: u64, d: u64) -> usize {
    (1..t)
        .map(|s| s * (t - s))
        .filter(|n| *n>d )
        .collect::<Vec<_>>()
        .len()
}


fn parse_numbers(line: &str, part2: bool) -> Vec<u64> {
    if part2 {
        return Vec::from([
            line.split_whitespace().skip(1)
                .collect::<String>()
                .parse::<u64>().unwrap()
        ]);
    }
    line.split_whitespace().skip(1)
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
}

fn parse_game(input: &str, part2: bool) -> (Vec<u64>, Vec<u64>) {
    let (times, dists) = input.split_once("\n").unwrap();
    let times = parse_numbers(times, part2);
    let dists = parse_numbers(dists, part2);
    (times, dists)
}

fn eval_races(times: Vec<u64>, dists: Vec<u64>, part2: bool) -> usize {
    if part2 {
        return wins(times[0], dists[0]);
    }
    times.iter().zip(dists)
        .map(|(&t, d)| wins(t,d))
        .product()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (times, dists) = parse_game(input, false);
    let result = eval_races(times, dists, false);
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (times, dists) = parse_game(input, true);
    let result = eval_races(times, dists, true);
    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
