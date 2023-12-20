// 523190878 <- too low - lo: 11951, hi: 43778
// 580340560 <- too low - lo: 11951, hi: 48560

use std::collections::HashMap;
use std::fmt;

advent_of_code::solution!(20);

const VERBOSE: bool = true;
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
                    //modules.lo |= mapper.get(k);
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
                    let v = modules.inputs.get(&k).unwrap();
                    modules.inputs.insert(k, *v | *i);
                    //*v |= *i;
                }
            }
        }
        modules
    }
    fn push(&mut self, times: u64) -> (u64, u64) {
        let mut lo_total = 0;
        let mut hi_total = 0;
        //let n = 64 - self.broadcaster.leading_zeros();
        for _ in 0..times {
            if VERBOSE {println!("=== PUSH =====================")};
            lo_total += 1; // button signal
            self.lo |= self.broadcaster;
            lo_total += self.broadcaster.count_ones() as u64;
            // for i in 0..n {
            //     let m = 1 << i;
            //     if self.broadcaster & m != 0 {
            //         self.lo |= m;
            //     }
            // }
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
        let losig = self.lo.count_ones() as u64;
        let hisig = self.hi.count_ones() as u64;
        //println!("{}signals: {}-{}\n--------------", self, losig, hisig);
        if VERBOSE {println!("{}--------------", self)};
        if losig + hisig == 0 {
            return (0, 0);
        }
        let (lo, hi) = (self.lo, self.hi);
        (self.lo, self.hi) = (0, 0);
        let mut conjunctions = Vec::new();
        let mut count_lo = 0;
        let mut count_hi = 0;
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
                            // let p = 64 - out.leading_zeros();
                            // for i in 0..p {
                            //     let n = 1 << i;
                            //     if out & n != 0 {
                            //         self.lo |= out;
                            //     }
                            // }
                            self.lo |= out;
                            count_lo += out.count_ones();
                        } else {
                            self.config |= m;
                            // let p = 64 - out.leading_zeros();
                            // for i in 0..p {
                            //     let n = 1 << i;
                            //     if out & n != 0 {
                            //         self.hi |= out;
                            //     }
                            // }
                            self.hi |= out;
                            count_hi += out.count_ones();
                        }
                    }
                }
            }
        }
        // for all conjunctions that received a low signal
        // send high signal to the outputs
        for m in conjunctions {
            let &out = self.outputs.get(&m).unwrap();
            self.hi |= out;
            count_hi += out.count_ones();
            // let p = 64 - out.leading_zeros();
            // for i in 0..p {
            //     let n = 1 << i;
            //     if out & n != 0 {
            //         self.hi |= out;
            //     }
            // }
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
                    //let mut count = inp.count_ones() as i32;
                    //let t = self.config & inp;
                    if self.config & inp == inp {
                        self.lo |= out;
                        count_lo += out.count_ones();
                    } else {
                        self.hi |= out;
                        count_hi += out.count_ones();
                    }
                    // let p = 64 - inp.leading_zeros();
                    // for i in 0..p {
                    //     let n = 1 << i;
                    //     if inp & n != 0 {
                    //         if self.config & n != 0 {
                    //             count -= 1;
                    //         }
                    //     }
                    // }
                    // let &out = self.outputs.get(&m).unwrap();
                    // let p = 64 - out.leading_zeros();
                    // for i in 0..p {
                    //     let n = 1 << i;
                    //     if out & n != 0 {
                    //         if count == 0 {
                    //             self.lo |= n;
                    //         } else {
                    //             self.hi |= n;
                    //         }
                    //     }
                    // }
                }
            }
        }
        //(losig, hisig)
        (count_lo as u64, count_hi as u64)
    }
}

impl fmt::Display for Modules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "broadcaster: {:b}", self.broadcaster)?;
        writeln!(f, "inputs: {:?}", self.inputs)?;
        writeln!(f, "outputs: {:?}", self.outputs)?;
        writeln!(f, "config: {:b}", self.config)?;
        writeln!(f, "lohi: {:b}-{:b}",self.lo, self.hi)?;
        // writeln!(f, "{:b} <- {:b}-{:b}", self.config, self.lo, self.hi)?;
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