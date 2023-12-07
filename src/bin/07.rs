advent_of_code::solution!(7);

type Int = u32;

#[derive(Debug, PartialEq)]
enum Strength {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

fn parse_game(input: &str, part2: bool) -> Vec<([u8; 5], Int, u8)> {
    input.lines().map(|line| {
        let (cards, bid) = line.split_once(" ").unwrap();
        let mut hand = [0u8; 5];
        for (i, c) in cards.as_bytes().iter().enumerate() {
            match *c {
                b'2'..=b'9' => hand[i] = *c - b'0',
                b'T' => hand[i] = 10,
                b'J' => hand[i] = if part2 {1} else {11},
                b'Q' => hand[i] = 12,
                b'K' => hand[i] = 13,
                b'A' => hand[i] = 14,
                _ => unreachable!(),
            }
        }
        (hand, bid.parse::<Int>().unwrap(), 0u8)
    }).collect::<Vec<_>>()
}

fn eval_hand(hand: &[u8; 5]) -> Strength {
    // count card values
    let mut counts = [0u8; 15];
    for c in hand {
        counts[*c as usize] += 1;
    }
    let jokers = counts[1];
    // find best combination
    match counts.iter().max().unwrap() {
        1 => match jokers {
                1 => Strength::OnePair, // J, a, b, c, d
                _ => Strength::HighCard // a, b, c, d, e
            },
        2 => if counts.iter().filter(|&&c| c == 2).count() == 2 {
                match jokers {
                    0 => Strength::TwoPair, // a, a, b, b, c
                    1 => Strength::FullHouse, // J, a, a, b, b
                    _ => Strength::FourOfAKind // J, J, a, a, b
                }
            } else {
                match jokers {
                    0 => Strength::OnePair, // a, a, b, c, d
                    _ => Strength::ThreeOfAKind // J, a, a, b, c || J, J, a, b, c
                }
            },
        3 => if counts.contains(&2u8) {
                match jokers {
                    0 => Strength::FullHouse, // a, a, a, b, b
                    _ => Strength::FiveOfAKind // J, J, a, a, a || J, J, J, a, a
                }
            } else {
                match jokers {
                    0 => Strength::ThreeOfAKind, // a, a, a, b, c
                    _ => Strength::FourOfAKind // J, a, a, a, b || J, J, J, a, b
                }
            },
        4 => match jokers {
                0 => Strength::FourOfAKind, // a, a, a, a, b
                _ => Strength::FiveOfAKind // J, a, a, a, a || J, J, J, J, a
            },
        5 => Strength::FiveOfAKind,  // a, a, a, a, a || J, J, J, J, J
        _ => unreachable!(),
    }
}

fn eval_game(mut game: Vec<([u8; 5], Int, u8)>) -> u32 {
    // calculate hand strength
    for (hand, _, strength) in game.iter_mut() {
        *strength = eval_hand(hand) as u8;
    }
    // sort by strength
    game.sort_by(|(h1, _, s1), (h2, _, s2)| {
        s1.cmp(s2)
            .then_with(|| h1[0].cmp(&h2[0]))
            .then_with(|| h1[1].cmp(&h2[1]))
            .then_with(|| h1[2].cmp(&h2[2]))
            .then_with(|| h1[3].cmp(&h2[3]))
            .then_with(|| h1[4].cmp(&h2[4]))
    });
    // calculate win
    for (i, (_, bid, _)) in game.iter_mut().enumerate() {
        *bid *= i as u32 + 1;
    }
    // return sum of all wins
    game.iter().map(|(_, bid, _)| *bid).sum()
}

// fn print_game(game: &Vec<([u8; 5], Int, u8)>) {
//     for (hand, bid, strength) in game {
//         println!("{:?} {} : {:?}", hand, bid, strength);
//     }
// }

pub fn part_one(input: &str) -> Option<u32> {
    Some(eval_game(parse_game(input, false)))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(eval_game(parse_game(input, true)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
