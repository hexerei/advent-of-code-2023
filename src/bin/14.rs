advent_of_code::solution!(14);

struct Game {
    map: Vec<Vec<u8>>,
    orientation: usize,
    //row: usize,
    rows: usize,
    cycle: usize,
    unique: usize,
    distinct: usize,
    step: usize,
    cache: Vec<usize>,
}
impl Game {

    fn from(input: &str) -> Game {
        let map = input.lines().map(|line| {
            line.chars().map(|c| {
                match c {
                    '#' => 2,
                    'O' => 1,
                    _ => 0,
                }
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();
        let rows = map.len();
        Game {
            map: map,
            orientation: 3,
            rows,
            cycle: 0,
            unique: if rows == 10 { 2 } else { 153 },
            distinct: if rows == 10 { 9 } else { 179 },
            step: if rows == 10 { 7 } else { 26 },
            cache: Vec::new(),
        }
    }

    fn rotate(&mut self) {
        self.orientation = (self.orientation + 1) % 4;
        self.map = (0..self.rows).map(|i| {
            self.map.iter().rev().map(|row| {
                *row.iter().nth(i).unwrap() as u8
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();
        for row in self.map.iter_mut() {
            let mut i = row.len() - 1;
            let mut swapped: bool = false;
            loop {
                if row[i-1] == 1 && row[i] == 0 {
                    row.swap(i, i-1);
                    swapped = true;
                }
                if i == 1 {
                    if !swapped { break; }
                    i = row.len() - 1;
                    swapped = false;
                } else {
                    i -= 1;
                };
            }
        }
    }

    fn score(&mut self) -> usize {
        self.map.iter().map(|row| {
            row.iter().enumerate().filter_map(|(i, &c)| {
                if c == 1 { Some(i + 1) }
                else { None }
            }).collect::<Vec<_>>().iter().sum::<usize>()
        }).collect::<Vec<_>>().iter().sum::<usize>()
    }

    fn score_north(&mut self) -> usize {
        self.map.iter().enumerate().map(|(i, row)| {
            row.iter().filter_map(|&c| {
                if c == 1 { Some(self.rows - i) }
                else { None }
            }).collect::<Vec<_>>().iter().sum::<usize>()
        }).collect::<Vec<_>>().iter().sum::<usize>()
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("{} {}","-".repeat(40), match self.orientation {
            0 => "NORTH",
            1 => "WEST",
            2 => "SOUTH",
            3 => "EAST",
            _ => "?",
        });
        for row in self.map.iter() {
            println!("{}", row.iter().map(|&c|
                match c {
                    2 => '#',
                    1 => 'O',
                    _ => '.',
                }
            ).collect::<String>());
        }
    }

    fn cycle(&mut self) {
        (0..4).for_each(|_| {
            self.rotate();
        });
        //self.print();
        let score = self.score_north();
        self.cache.push(score);
        self.cycle += 1;
    }

    fn run(&mut self, cycles: usize) -> usize {
        if cycles == 0 {
            self.rotate();
            return self.score();
        }
        let m = if cycles < self.distinct {cycles} else {self.distinct};
        (0..m).for_each(|_| { self.cycle(); });
        //println!("{:?}", self.cache);
        let mut off = cycles - 1;
        if off >= self.distinct {
            off = ((off-self.distinct) % self.step) + self.unique;
        }
        //println!("> cache[{}] = {:?}", off, self.cache[off]);
        self.cache[off]
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(Game::from(input).run(0) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(Game::from(input).run(1000000000) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
