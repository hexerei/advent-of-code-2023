use regex::Regex;

advent_of_code::solution!(3);



#[derive(Debug)]
struct Value {
    line: usize,
    start: usize,
    end: usize,
    value: u32
}
impl Value {
    fn new(line: usize, start: usize, end: usize, value: u32) -> Value {
        Value {
            line: line,
            start: start,
            end: end,
            value: value
        }
    }
}

#[derive(Debug)]
struct Symbol {
    line: usize,
    start: usize,
    end: usize,
}
impl Symbol {
    fn new(line: usize, start: usize, end: usize) -> Symbol {
        Symbol {
            line: line,
            start: start,
            end: end,
        }
    }
}

#[derive(Debug)]
struct Map {
    symbols: Vec<Symbol>,
    values: Vec<Value>,
}
impl Map {
    fn new() -> Map {
        Map {
            symbols: Vec::new(),
            values: Vec::new()
        }
    }
    fn add_symbol(&mut self, line: usize, start: usize, end: usize) {
        self.symbols.push(Symbol::new(line, start, end));
    }
    fn add(&mut self, line: usize, start: usize, end: usize, value: u32) {
        self.values.push(Value::new(line, start, end, value));
    }
    fn is_valid(&self, line: usize, start: usize, end: usize) -> bool {
        let rows = [(line as i32 - 1), line as i32, line as i32 + 1];
        let cols = [start as i32 - 1, end as i32 + 1];
        for symbol in &self.symbols {
            // if rows.contains(&(symbol.line as i32)) {
            //     println!("LIN {:?} IS in {:?}", symbol.line, rows);
            // } else {
            //     println!("LIN {:?} IS NOT in {:?}", symbol.line, rows);
            // }
            // if symbol.start as i32 >= cols[0]
            // && symbol.start as i32 <= cols[1] {
            //     println!("STA {:?} IS in {:?}", symbol.start, cols);
            // } else {
            //     println!("STA {:?} IS NOT in {:?}", symbol.start, cols);
            // }
            // if symbol.end as i32 >= cols[0]
            // && symbol.end as i32 <= cols[1] {
            //     println!("END {:?} IS in {:?}", symbol.end, cols);
            // } else {
            //     println!("END {:?} IS NOT in {:?}", symbol.end, cols);
            // }
            if rows.contains(&(symbol.line as i32))
            && symbol.start as i32 >= cols[0]
            && symbol.start as i32 <= cols[1]
            && symbol.end as i32  >= cols[0]
            && symbol.end as i32  <= cols[1] {
                // println!("--- MATCH ---");
                return true;
            }
            // println!("--- NO MATCH ---");
        }
        false
    }
    fn parse_symbols(&mut self, line: &str, offset: usize, strict: bool) {
        let mut regex =Regex::new(r"[^\d\.]").unwrap();
        if strict {
            regex = Regex::new(r"\*").unwrap();
        };
        // result will be an iterator over tuples containing the start and end indices for each match in the string
        let result = regex.captures_iter(line);
        for mat in result {
            let start = mat.get(0).unwrap().start();
            let end = mat.get(0).unwrap().end();
            self.add_symbol(offset, start, end);
        }
    }
    fn parse(&mut self, line: &str, offset: usize) {
        let regex = Regex::new(r"[\d]+").unwrap();
        // result will be an iterator over tuples containing the start and end indices for each match in the string
        let result = regex.captures_iter(line);
        for mat in result {
            let start = mat.get(0).unwrap().start();
            let end = mat.get(0).unwrap().end();
            let value = mat.get(0).unwrap().as_str().parse::<u32>().unwrap();
            if !self.is_valid(offset, start, end) {
                continue;
            }
            self.add(offset, start, end, value);
        }
    }
    fn sum(&self) -> u32 {
        let mut sum = 0;
        for value in &self.values {
            sum += value.value;
        }
        sum
    }

    fn sum_gears(&self) -> u32 {
        let mut sum = 0;

        for symbol in &self.symbols {
            let mut lhs = 0u32;
            let mut rhs = 0u32;
            let rows = [(symbol.line as i32 - 1), symbol.line as i32, symbol.line as i32 + 1];
            for value in &self.values {
                if rows.contains(&(value.line as i32)) {
                    if lhs == 0 {
                        lhs = value.value;
                    } else {
                        rhs = value.value;
                    }
                }
            }
            if lhs != 0 && rhs != 0 {
                sum += lhs * rhs;
            }
        }
        sum
    }

}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::new();
    let mut offset = 0;
    for line in input.lines() {
        map.parse_symbols(line, offset, false);
        offset += 1;
    }
    offset = 0;
    for line in input.lines() {
        map.parse(line, offset);
        offset += 1;
    }
    //println!("{:?}", map);
    Some(map.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = Map::new();
    let mut offset = 0;
    for line in input.lines() {
        map.parse_symbols(line, offset, true);
        offset += 1;
    }
    offset = 0;
    for line in input.lines() {
        map.parse(line, offset);
        offset += 1;
    }
    println!("{:?}", map);
    Some(map.sum_gears())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
