use std::collections::{BTreeMap, BinaryHeap, HashMap};
use std::cmp::Reverse;
//use std::ops::Add;

// type V = (usize, usize);
// type E = u32;
// type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;


advent_of_code::solution!(17);

// fn dijkstra<V: Ord + Copy, E: Ord + Copy + Add<Output = E>>(
//     graph: &Graph<V, E>,
//     start: &V,
// ) -> BTreeMap<V, Option<(V, E)>> {
//     let mut ans = BTreeMap::new();
//     let mut prio = BinaryHeap::new();
//     ans.insert(*start, None);
//     for (new, weight) in &graph[start] {
//         ans.insert(*new, Some((*start, *weight)));
//         prio.push(Reverse((*weight, new, start)));
//     }
//     while let Some(Reverse((dist_new, new, prev))) = prio.pop() {
//         match ans[new] {
//             Some((p, d)) if p == *prev && d == dist_new => {}
//             _ => continue,
//         }
//         for (next, weight) in &graph[new] {
//             match ans.get(next) {
//                 Some(Some((_, dist_next))) if dist_new + *weight >= *dist_next => {}
//                 Some(None) => {}
//                 _ => {
//                     ans.insert(*next, Some((*new, *weight + dist_new)));
//                     prio.push(Reverse((*weight + dist_new, next, new)));
//                 }
//             }
//         }
//     }
//     ans
// }

// fn step_dijkstra(
//     graph: &Graph<V, E>,
//     start: &V,
//     end: &V,
//     steps: (i32, i32),
// ) -> u32 {
//     let mut ans = BTreeMap::new();
//     let mut prio = BinaryHeap::new();
//     let (minsteps, maxsteps) = steps;
//     ans.insert(*start, None);
//     for (&new, loss) in &graph[start] {
//         ans.insert(new, Some((*start, *loss)));
//         prio.push(Reverse((*loss, new, *start)));
//     }
//     while let Some(Reverse((loss, new, prev))) = prio.pop() {
//         let dir = (new.0 as i32 - prev.0 as i32, new.1 as i32 - prev.1 as i32);
//         println!("({},{})-({},{}) ({},{}) {}", prev.0, prev.1, new.0, new.1, dir.0, dir.1, loss);
//         if (new.0, new.1) == (end.0, end.1) {
//             break;
//         }
//         match ans[&new] {
//             Some((p, l)) if p == prev && l == loss => {}
//             _ => continue,
//         }
//         for (dx, dy) in [(0,1), (1,0), (0,-1), (-1,0)] as [(i32, i32); 4] {
//             if dx.abs_diff(dir.0) == 0 && dy.abs_diff(dir.1) == 0 {
//                 continue;
//             }
//             for step in 1..maxsteps {
//                 let next = (new.0 as i32 + dx * step, new.1 as i32 + dy * step);
//                 if next.0 < 0 || next.1 < 0 || next.0 > end.0 as i32 || next.1 > end.1 as i32 {
//                     //println!("OUT OF BOUNDS {}.{}", next.0, next.1);
//                     continue;
//                 }
//                 let next = (next.0 as usize, next.1 as usize);
//                 let new_loss = loss + graph[&new].get(&next).unwrap_or(&(u32::MAX - loss));
//                 if new_loss < u32::MAX { println!("NEW LOSS {}", new_loss); }
//                 if minsteps <= step {
//                     match ans.get(&next) {
//                         Some(Some((_, next_loss))) if new_loss >= *next_loss => {}
//                         Some(None) => {}
//                         _ => {
//                             ans.insert(next, Some((new, new_loss)));
//                             prio.push(Reverse((new_loss, next, new)));
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     ans[end].unwrap().1
// }

fn dijkstra(grid: &Vec<Vec<u8>>, minstep: isize, maxstep: isize) -> i64 {
    let mut dists = HashMap::new();
    let mut q = BinaryHeap::from_iter([(0, (0,0,(0,0)))]);
    while let Some((cost, (r, c, d))) = q.pop() {
      if (r,c) == (grid.len() - 1, grid[0].len() - 1) {
        for (k, v) in &dists {
          println!("{:?} {:?}", k, v);
        }
        //dbg!(dists);
        return -cost;
      }
      if dists.get(&(r, c, d)).is_some_and(|&c| -cost > c) {
        continue;
      }
      for (dr, dc) in [(-1,0), (1,0), (0,-1), (0,1)] {
        if d == (dr, dc) || d == (-dr, -dc) {
          continue;
        }
        let mut next_cost = -cost;
        for dist in 1..=maxstep {
          let rr = (r as isize + dr * dist) as usize;
          let cc = (c as isize + dc * dist) as usize;
          if rr >= grid.len() || cc >= grid[0].len() {
            continue;
          }
          next_cost += grid[rr][cc] as i64;
          let key = (rr, cc, (dr, dc));
          if minstep <= dist && next_cost < *dists.get(&key).unwrap_or(&10000000) {
            dists.insert(key, next_cost);
            q.push((-next_cost, key));
          }
        }
      }
    }
    unreachable!()
}


// based on dijkstra
fn solve(grid: &Vec<Vec<u8>>, steps: (i32, i32)) -> u32 {
    let directions: [(i32, i32); 5] = [(0,0), (-1,0), (1,0), (0,-1), (0,1)];
    //let mut ans = BTreeMap::new();
    let mut ans: HashMap<(usize, usize, usize), u32> = HashMap::new();
    let mut prio = BinaryHeap::new();
    let (minsteps, maxsteps) = steps;
    let (tx, ty) = (grid[0].len() - 1, grid.len() - 1);
    //ans.insert(*start, None);
    //for (&new, loss) in &graph[start] {
    //    ans.insert(new, Some((*start, *loss)));
    //    prio.push(Reverse((*loss, new, *start)));
    //}
    prio.push(Reverse((0u32, (0usize, 0usize, 0usize))));
    while let Some(Reverse((loss, (x, y, d)))) = prio.pop() {
    // prio.push((0u32, (0usize, 0usize, 0usize)));
    // while let Some((loss, (x, y, d))) = prio.pop() {
            //let dir = (new.0 as i32 - prev.0 as i32, new.1 as i32 - prev.1 as i32);
        let dir = directions[d];
        //println!("({}, {} , ({}, {})) {}", x, y, dir.0, dir.1, loss);
        // reached end?
        if (x, y) == (tx, ty) {
            for ((x, y, d), v) in &ans {
                println!("({}, {}, ({}, {})) {}", y, x, directions[*d].1, directions[*d].0, v);
            }
            //   dbg!(ans);
            println!("FOUND {}", loss);
            return loss;
        }
        if ans.get(&(x, y, d)).is_some_and(|&c| loss > c) {
            continue;
        }
        // match ans.get(&new) {
        //     Some(Some((p, l))) if *p == prev && loss > *l => continue,
        //     _ => (),
        // }
        for i in 1..5 {
            let (dx, dy) = directions[i];
            // can't go back
            if dx.abs_diff(dir.0) == 0 && dy.abs_diff(dir.1) == 0 {
                continue;
            }
            let mut new_loss = loss;
            for step in 1..=maxsteps {
                let (nx, ny) = (x as i32 + dx * step, y as i32 + dy * step);
                // out of bounds
                if nx < 0 || ny < 0 || nx > tx as i32 || ny > ty as i32 {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);
                new_loss += grid[ny][nx] as u32;
                //print!("-> ({},{}) {}", nx, ny, new_loss);
                if minsteps <= step {
                    if new_loss < *ans.get(&(nx, ny, i)).unwrap_or(&u32::MAX) {
                        ans.insert((nx, ny, i), new_loss);
                        prio.push(Reverse((new_loss, (nx, ny, i))));
                        // prio.push((new_loss, (nx, ny, i)));
                        //println!(" PUSHED"); break;
                    }
                    // match ans.get(&next) {
                    //     Some(Some((_, next_loss))) if new_loss >= *next_loss => {println!(" LONGER"); break; }
                    //     Some(None) => {println!(" NONE");}
                    //     _ => {
                    //         ans.insert((next, new), new_loss);
                    //         prio.push(Reverse((new_loss, next, new)));
                    //         println!(" PUSHED");
                    //     }
                    // }
                }
            }
        }
    }
    dbg!(ans);
    0
}


// fn add_edge<V: Ord + Copy, E: Ord>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E) {
//     graph.entry(v1).or_insert_with(BTreeMap::new).insert(v2, c);
//     graph.entry(v2).or_insert_with(BTreeMap::new);
// }

pub fn part_one(input: &str) -> Option<u32> {
    // let mut graph = Graph::new();
    // let (tx, ty) = (input.lines().next().unwrap().len() - 1, input.lines().count() - 1);
    // input.lines().enumerate().for_each(|(y, line)|
    //     line.chars().enumerate().for_each(|(x, c)| {
    //         let c = c.to_digit(10).unwrap_or(10);
    //         if x == 0 && y == 0 {
    //             add_edge(&mut graph, (x, y), (x, y+1), 0);
    //             add_edge(&mut graph, (x, y), (x+1, y), 0);
    //         } else {
    //             if y > 0 { add_edge(&mut graph, (x, y), (x, y-1), c); }
    //             if x > 0 { add_edge(&mut graph, (x, y), (x-1, y), c); }
    //             if y < ty { add_edge(&mut graph, (x, y), (x, y+1), c); }
    //             if x < tx { add_edge(&mut graph, (x, y), (x+1, y), c); }
    //         }
    //     })
    // );

    let grid = input.lines().map(|line|
        line.as_bytes().iter().map(|b| b - b'0').collect::<Vec<_>>()
    ).collect::<Vec<_>>();

    //let heatloss = solve(&grid, (1,3));

    let heatloss = dijkstra(&grid, 1, 3);

    //let heatloss = step_dijkstra(&graph, &(0, 0), &(tx, ty), (1,3));

    //dbg!(&heatloss);

    Some(heatloss as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input.lines().map(|line|
        line.as_bytes().iter().map(|b| b - b'0').collect::<Vec<_>>()
    ).collect::<Vec<_>>();
    let heatloss = dijkstra(&grid, 4, 10);
    Some(heatloss as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
