use std::collections::HashMap;
use itertools::Itertools;

advent_of_code::solution!(19);

const VERBOSE: bool = false;
type FlowMap<'a> = HashMap<&'a str, (Vec<(usize, bool, u32, &'a str)>, &'a str)>;

fn parse_input(input: &str) -> (FlowMap, Vec<(u32,u32,u32,u32)>) {
    let (flows, parts) = input.split_once("\n\n").unwrap();
    let flows = flows.lines().map(|line| {
        let (id, flow) = line.split_once("{").unwrap();
        let (flow, end) = flow[0..flow.len()-1].split_at(flow.rfind(',').unwrap());
        let end = end.trim_matches(',');
        (id, (flow.split(",").map(|part| {
            let (rule, goto) = part.split_once(":").unwrap();
            let mut chars = rule.chars();
            (
                match chars.next() {
                    Some('x') => 0,
                    Some('m') => 1,
                    Some('a') => 2,
                    Some('s') => 3,
                    _ => unreachable!("{} {:?}", rule, chars)
                } as usize,
                match chars.next() {
                    Some('>') => true,
                    Some('<') => false,
                    _ => unreachable!("{} {:?}", rule, chars)
                },
                chars.as_str().parse::<u32>().unwrap(),
                goto
            )
        }).collect::<Vec<_>>(), end))
    }).collect::<HashMap<_,_>>();
    let parts = parts.lines().map(|line| {
        line.trim_matches(['{','}']).split(",").map(|p| {
            p.chars().filter(|c| c.is_numeric()).collect::<String>().parse::<u32>().unwrap()
        }).collect_tuple().unwrap()
    }).collect::<Vec<_>>();
    if VERBOSE {
        for (k, v) in flows.iter() {
            print!("{}:\t", *k);
            for &(i, greater, n, goto) in v.0.iter() {
                print!("({} {} {:4} -> {:3})\t", i, match greater {true=>'>',false=>'<'}, n, goto);
            }
            println!(" -> {}", v.1);
        }
        println!("{:?}", parts);
    }
    (flows, parts)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (flows, parts) = parse_input(input);

    let sum = parts.iter().filter(|&&(x,m,a,s)|{
        let vals = [x,m,a,s];
        let mut inflow = "in";
        while inflow != "A" && inflow != "R" {
            let (rules, goto) = flows.get(inflow).unwrap();
            inflow = rules.iter()
            .find(|&&(i, greater, n, _)| {
                (greater && vals[i] > n) || (!greater && vals[i] < n)
            }).map(|(_,_,_,goto)| *goto).unwrap_or(*goto);
        }
        inflow == "A"
    }).map(|&(x,m,a,s)| (x+m+a+s) as usize )
    .sum();

    Some(sum)
}

fn count_accepted(flows: &FlowMap, inflow: &str, vals: [Vec<u32>; 4]) -> usize {
    if inflow == "A" {
        return vals.iter().map(|v| v.len()).product();
    } else if inflow == "R" {
        return 0;
    }
    let mut vals = vals;
    let mut accepted = 0;
    let (flow, end) = flows.get(inflow).unwrap();
    for &(i, greater, n, goto) in flow.iter() {
        let mut newvals = vals.clone();
        (newvals[i], vals[i]) = vals[i].iter().partition(|&val|
            (greater && *val > n) || (!greater && *val < n));
        accepted += count_accepted(flows, goto, newvals);
    }
    accepted += count_accepted(flows, end, vals);
    accepted
}

pub fn part_two(input: &str) -> Option<usize> {
    let (flows, _) = parse_input(input);
    let valid = (1..=4000).map(|x| x as u32).collect::<Vec<_>>();
    Some(count_accepted(&flows, "in", [valid.clone(),valid.clone(),valid.clone(),valid]))
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
