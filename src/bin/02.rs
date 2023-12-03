advent_of_code::solution!(2);

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<[u32;3]>,
}
impl Game {
    fn new(id: u32) -> Game {
        Game {
            id: id,
            draws: Vec::new(),
        }
    }
    fn from(line: &str) -> Game {
        if line.starts_with("Game") {
            let offset = line.find(":").unwrap();
            let (first, last) = line.split_at(offset);
            // first contains "Game #"
            let mut game = Game::new(first[5..].parse::<u32>().unwrap());
            // last contains ": ..."
            let last = String::from(&last[2..]);
            let s = last.split(";");
            for draws in s {
                game.parse_draw(draws);
            }
            return game
        } else {
            Game::new(0)
        }
    }
    fn parse_draw(&mut self, draw: &str) {
        let mut cubes = [0u32;3];
        let draw = draw.trim();
        let draws = draw.split(",");
        for draw in draws {
            let draw = draw.trim();
            let mut draw = draw.split(" ");
            let d = draw.next().unwrap().parse::<u32>();
            if d.is_err() {
                continue;
            }
            match draw.last() {
                Some("red") => {
                    cubes[0] = d.unwrap();
                },
                Some("green") => {
                    cubes[1] = d.unwrap();
                },
                Some("blue") => {
                    cubes[2] = d.unwrap();
                },
                _ => ()
            }
        }
        self.add_draw(cubes);
    }
    fn add_draw(&mut self, draw: [u32;3]) {
        self.draws.push(draw);
    }
    fn is_valid(&self) -> bool {
        let max = [12u32, 13u32, 14u32];
        for draw in &self.draws {
            if draw[0] > max[0] || draw[1] > max[1] || draw[2] > max[2] {
                return false
            };
        }
        true
    }
    fn get_min_valid(&self) -> [u32;3] {
        let mut min = [0u32;3];
        for draw in &self.draws {
            for i in 0..3 {
                if draw[i] > min[i] {
                    min[i] = draw[i];
                }
            }
        }
        min
    }
    fn get_power(&self) -> u32 {
        let min = self.get_min_valid();
        let power = min[0] * min[1] * min[2];
        power
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let sum: u32 = input.lines().map(|line| {
        let game = Game::from(line);
        //println!("{:?}", game);
        if game.is_valid() {
            return game.id
        }
        0
    }).sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum: u32 = input.lines().map(|line| {
        let game = Game::from(line);
        //println!("{:?}", game);
        game.get_power()
    }).sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
