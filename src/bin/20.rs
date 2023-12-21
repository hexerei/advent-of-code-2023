// 523190878 <- too low - lo: 11951, hi: 43778
// 580340560 <- too low - lo: 11951, hi: 48560

use std::collections::HashMap;
use std::fmt;

advent_of_code::solution!(20);

const VERBOSE: bool = false;
const DEBUG: bool = false;
const TEST: bool = false;
const REPEAT: u64 = 1000;

struct Mapper {
    module: u64,
    map: HashMap<String, u64>,
}
impl Mapper {
    fn new() -> Self {
        Mapper {
            module: 0,
            map: HashMap::new(),
        }
    }
    fn get(&mut self, key: &str) -> u64 {
        if let Some(v) = self.map.get(key) {
            *v
        } else {
            let v = 1 << self.module;
            self.map.insert(String::from(key), v);
            self.module += 1;
            v
        }
    }
}

struct Modules {
    broadcaster: u64,
    inputs: HashMap<u64,u64>,
    outputs: HashMap<u64,u64>,
    config: u64,
    hi: u64,
    lo: u64,
}
impl Modules {

    fn from(input: &str) -> Self {
        let mut modules = Modules {
            broadcaster: 0,
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            config: 0,
            hi: 0,
            lo: 0,
        };
        let mut mapper = Mapper::new();
        for line in input.lines() {
            let (left, right) = line.split_once(" -> ").unwrap();
            let mut char_iter = left.chars();
            match char_iter.next().unwrap() {
                'b' => right.split(", ").for_each(|k| {
                    modules.broadcaster |= mapper.get(k);
                }),
                '%' => {
                    let m = mapper.get(char_iter.as_str());
                    let mut o = 0;
                    right.split(", ").for_each(|k| {
                        o |= mapper.get(k);
                    });
                    modules.outputs.insert(m, o);
                },
                '&' => {
                    let m = mapper.get(char_iter.as_str());
                    let mut o = 0;
                    right.split(", ").for_each(|k| {
                        o |= mapper.get(k);
                    });
                    modules.outputs.insert(m, o);
                    modules.inputs.insert(m, 0);
                },
                _ => unreachable!("unknown: {}", left),
            };
        }
        let keys = modules.inputs.keys().cloned().collect::<Vec<_>>();
        for k in keys {
            for (i, o) in modules.outputs.iter() {
                if *o & k != 0 {
                    modules.inputs.insert(k,
                        *modules.inputs.get(&k).unwrap()
                        | *i);
                }
            }
        }
        modules
    }

    fn push(&mut self, times: u64) -> (u64, u64) {
        let mut lo_total = 0;
        let mut hi_total = 0;
        for _ in 0..times {
            if VERBOSE {println!("=== PUSH =====================")};
            lo_total += 1; // count button signal
            self.lo |= self.broadcaster;
            // count boradcaster signals
            lo_total += self.broadcaster.count_ones() as u64;
            // and add all other signals
            loop {
                let (losig, hisig)  = self.process();
                if losig + hisig == 0 {
                    break;
                }
                lo_total += losig;
                hi_total += hisig;
            }
        }
        (lo_total, hi_total)
    }

    fn process(&mut self) -> (u64, u64) {
        if VERBOSE {println!("{}--------------", self)};
        if self.lo.count_ones() + self.hi.count_ones() == 0 {
            return (0, 0);
        }
        let mut conjunctions = Vec::new();
        let mut count_lo = 0;
        let mut count_hi = 0;
        // get los and high signals and reset
        let (lo, hi) = (self.lo, self.hi);
        (self.lo, self.hi) = (0, 0);
        // check all low signals
        let n = 64 - lo.leading_zeros();
        for i in 0..n {
            let m = 1 << i;
            if lo & m != 0 {
                // conjunction of all inputs
                if self.inputs.contains_key(&m) {
                    conjunctions.push(m);
                } else {
                    // flip-flop
                    if let Some(out) = self.outputs.get(&m) {
                        if self.config & m != 0 {
                            self.config &= !m;
                            self.lo |= out;
                            count_lo += out.count_ones();
                        } else {
                            self.config |= m;
                            self.hi |= out;
                            count_hi += out.count_ones();
                        }
                    }
                }
            }
        }
        // for all conjunctions that received a low signal
        // send high signal to the outputs
        for m in conjunctions.iter() {
            let &out = self.outputs.get(&m).unwrap();
            self.hi |= out;
            count_hi += out.count_ones();
        }
        // for all conjunctions that received a high signal
        // send low signal to the outputs
        let n = 64 - hi.leading_zeros();
        for i in 0..n {
            let m = 1 << i;
            if hi & m != 0 {
                // conjunction of all inputs
                if self.inputs.contains_key(&m) {
                    let &inp = self.inputs.get(&m).unwrap();
                    let &out = self.outputs.get(&m).unwrap();
                    if self.config & inp == inp {
                        self.lo |= out;
                        count_lo += out.count_ones();
                    } else {
                        self.hi |= out;
                        count_hi += out.count_ones();
                    }
                }
            }
        }
        (count_lo as u64, count_hi as u64)
    }
}

impl fmt::Display for Modules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if DEBUG {
            writeln!(f, "0x{:64b} - broadcaster", self.broadcaster)?;
            writeln!(f, "inputs:")?;
            for (i, o) in self.inputs.iter() {
                writeln!(f, "0x{:64b} - 0x{:64b}", i, o)?;
            }
            writeln!(f, "outputs:")?;
            for (i, o) in self.outputs.iter() {
                writeln!(f, "0x{:64b} - 0x{:64b}", i, o)?;
            }
            writeln!(f, "{}", "-".repeat(66))?;
        }
        if VERBOSE {
            writeln!(f, "0x{:64b} - config", self.config)?;
            writeln!(f, "0x{:64b} - low signals", self.lo)?;
            writeln!(f, "0x{:64b} - high signals", self.hi)?;
        } else {
            writeln!(f, "{:b} <- {:b}-{:b}", self.config, self.lo, self.hi)?;
        }
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = if TEST {
        "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"
    } else { input };
    let mut modules = Modules::from(input);
    let (lo_total, hi_total) = modules.push(REPEAT);
    if VERBOSE {println!("lo: {}, hi: {}", lo_total, hi_total)};
    Some(lo_total * hi_total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let modules = Modules::from(input);
    println!("{}", modules);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
