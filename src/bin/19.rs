use std::collections::HashMap;

use advent_of_code::template::aoc_cli::AocCommandError;

advent_of_code::solution!(19);

fn parse_game(input: &str) -> (HashMap<&str, Vec<(Option<(usize, bool, u32)>,&str)>>, Vec<Vec<u32>>) {
    let mut flows = HashMap::new();
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    workflows.lines().for_each(|line| {
        let (id, workflow) = line.split_once("{").unwrap();
        flows.insert( id, workflow.trim_matches('}').split(",").map(|part| {
            let (rule, goto) = part.split_once(":").unwrap_or(("",part));
            if rule.is_empty() {
                (None, goto)
            } else {
                let mut chars = rule.chars();
                let rule = (
                    match chars.next() {
                        Some('x') => 0,
                        Some('m') => 1,
                        Some('a') => 2,
                        Some('s') => 3,
                        _ => unreachable!("{} {:?}", rule, chars)
                    } as usize,
                    match chars.next() {
                        Some('<') => false,
                        Some('>') => true,
                        _ => unreachable!("{} {:?}", rule, chars)
                    },
                    chars.as_str().parse::<u32>().unwrap()
                );
                (Some(rule), goto)
            }
        }).collect::<Vec<_>>());
    });
    // for (k, v) in flows {
    //     println!("{:?} -> {:?}", k, v);
    // }
    let parts = parts.lines().map(|line| {
        line.trim_matches(['{','}']).split(",").map(|p| {
            p.chars().filter(|c| c.is_numeric()).collect::<String>().parse::<u32>().unwrap()
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    //println!("{:?}", parts);
    (flows, parts)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (flows, parts) = parse_game(input);
    let mut sum: u32 = 0;
    for part in parts {
        let mut inflow = "in";
        loop {
            if inflow == "A" {
                sum += part.iter().sum::<u32>();
                // println!("ACCEPT {:?}", part);
                break;
            } else if inflow == "R" {
                // println!("REJECT {:?}", part);
                break;
            }
            let rules = flows.get(inflow).unwrap();
            for rule in rules {
                let (rule, goto) = rule;
                if let Some((i, greater, n)) = rule {
                    if (*greater && part[*i] > *n) || (!*greater && part[*i] < *n) {
                        inflow = *goto;
                        break;
                    }
                } else {
                    inflow = *goto;
                    break;
                }
            }
        }
    }
    Some(sum)
}

fn eval_rule(inflow: &str, vals: &[usize; 4], rule: &(Option<(usize, bool, u32)>,&str)) -> [usize; 4] {
    print!("rule {} -> ", inflow);
    let mut vals = *vals;
    let (rule, goto) = *rule;
    if let Some((i, greater, n)) = rule {
        if greater {
            vals[i] = (4000 - n) as usize;
            if goto == "A" {
                println!("{:?} GT ACCEPTED", vals);
                return vals;
            } else if goto == "R" {
                vals[i] = (n - 1) as usize;
                println!("{:?} GT REJECTED", vals);
                return vals;
            }
        } else {
            vals[i] = (n - 1) as usize;
            if goto == "A" {
                println!("{:?} LT ACCEPTED", vals);
                return vals;
            } else if goto == "R" {
                vals[i] = (4000 - n) as usize;
                println!("{:?} LT REJECTED", vals);
                return vals;
            }
        }
    } else {
        if goto == "A" {
            println!("{:?} NN ACCEPTED", vals);
            return vals;
        } else if goto == "R" {
            println!("{:?} NN REJECTED", vals);
            return [0,0,0,0];
        }
    }
    println!("{:?}", vals);
    vals
}

fn eval_flow(inflow: &str, vals: &[usize; 4], accepted: &mut Vec<[usize; 4]>, flows: &HashMap<&str, Vec<(Option<(usize, bool, u32)>,&str)>>) {
    //let accepted:Vec<[u32; 4]> = Vec::new();
    print!("flow {} {:?}\n-> ", inflow, vals);
    let mut new_vals = *vals;
    let rules = flows.get(inflow).unwrap();
    for rule in rules {
        new_vals = eval_rule(inflow, &vals, rule);
        if new_vals == [0,0,0,0] {
            return;
        }
        let (_, goto) = rule;
        if *goto == "A" {
            println!("{:?} FLOW ACCEPTED", new_vals);
            accepted.push(new_vals);
        } else if *goto == "R" {
            println!("{:?} FLOW REJECTED", new_vals);
            continue;
        } else {
            eval_flow(goto, &new_vals, accepted, flows);
        }
    }
    println!("Accepted:");
    for val in accepted.iter() {
        println!("{:?}", val);
    }
    println!("");
}

pub fn part_two(input: &str) -> Option<usize> {
    let (flows, _) = parse_game(input);
    for (k, v) in flows.iter() {
        print!("{}:\t", *k);
        for &(r, g) in v.iter() {
            if r.is_some() {
                let (i, greater, n) = r.unwrap();
                print!("({} {} {:4} -> {:3})\t", i, match greater {true=>'>',false=>'<'}, n, g);
            } else {
                println!("(-> {})", g);
            }
        }
        //println!("]");
    }
    let mut accepted:Vec<[usize; 4]> = Vec::new();

    eval_flow("in", &[4000,4000,4000,4000], &mut accepted, &flows);
    let mut total: usize = 0;
    for val in accepted.iter() {
        let sum = val.iter().product::<usize>() as usize;
        total += sum;
        println!("{:?} = {}", val, sum);
    }
    println!("=== {} ===", total);
    println!("--- 167409079868000 ---");
    println!("+/- {}", total - 167409079868000);

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
