use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use petgraph::graph::{NodeIndex, UnGraph};
use itertools::Itertools;
use petgraph::algo::dijkstra;
use petgraph::visit::Walker;

#[derive(Debug)]
struct Valve {
    name: u32,
    flow: i32,
    tunnels: Vec<u32>,
}

impl Valve {
    pub fn new(name: u32, flow: i32, tunnels: Vec<u32>) -> Self {
        Self { name, flow, tunnels }
    }
}

#[derive(Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Path {
    // (valve, time when opened, total flow released)
    opened: Vec<(u32, i32, i32)>,
}

impl Path {
    pub fn new(opened: Vec<(u32, i32, i32)>) -> Self {
        Self { opened }
    }

    pub fn merged_flow(&self, other: &Path) -> i32 {
        let res = self.opened.iter().map(|(_, _, f)| *f).sum::<i32>();
        let map: HashMap<_, _> = self.opened.iter()
            .map(|(v, t, f)| (*v, (*t, *f)))
            .collect();
        let res = res + other.opened.iter().map(|(other_v, other_t, other_f)| {
            match map.get(other_v) {
                Some((t, f)) => if *t > *other_t { 0 } else { *other_f - *f },
                None => *other_f,
            }
        }).sum::<i32>();

        res
    }
}

fn create_dict() -> HashMap<String, u32> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    let mut idx = 0;
    lines.map(|line| {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        let name = split.nth(1).unwrap().to_string();
        let res = (name, idx);
        idx += 1;
        res
    }).collect()
}

fn parse_valve(line: String, dict: &HashMap<String, u32>) -> Valve {
    let mut split = line.split_whitespace();

    let name = split.nth(1).unwrap().to_string();
    let name = *dict.get(&name).unwrap();

    let rate = split.nth(2).unwrap();
    let rate = rate.replace("rate=", "").replace(";", "");
    let rate = rate.parse::<i32>().unwrap();

    let mut tunnels = Vec::new();
    let _next = split.nth(3);
    while let Some(tunnel) = split.next() {
        let tunnel = tunnel.replace(",", "");
        let tunnel = *dict.get(&tunnel).unwrap();
        tunnels.push(tunnel);
    }

    Valve::new(name, rate, tunnels)
}

fn parse_input(dict: &HashMap<String, u32>) -> HashMap<u32, Valve> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    lines.map(|line| {
        let line = line.unwrap();
        let valve = parse_valve(line, dict);
        (valve.name, valve)
    }).collect()
}

fn compute_graph(valves: &HashMap<u32, Valve>) -> UnGraph<u32, ()> {
    let edges: Vec<_> = valves.iter()
        .flat_map(|(v_id, v)| {
            v.tunnels.iter().filter_map(|t| {
                if *v_id > *t { Some((*v_id, *t)) } else { None }
            })
    }).collect();

    UnGraph::<u32, ()>::from_edges(edges)
}

fn compute_distances(g: &UnGraph<u32, ()>, all_src: &Vec<u32>, all_dst: &Vec<u32>) -> HashMap<(u32, u32), i32> {
    all_src.iter()
        .cartesian_product(all_dst.iter())
        .filter(|(&src, &dst)| src != dst)
        .map(|(&src, &dst)| {
            let dist = dijkstra(g, src.into(), Some(dst.into()), |_| 1);
            let dist = *dist.get(&dst.into()).unwrap();
            ((src, dst), dist)
    }).collect()
}

fn non_zero_valves(valves: &HashMap<u32, Valve>) -> Vec<u32> {
    valves.iter().filter_map(|(_, v)| {
        if v.flow > 0 { Some(v.name) } else { None }
    }).collect()
}

fn compute_best_flow(valves: &HashMap<u32, Valve>, dist_map: &HashMap<(u32, u32), i32>, to_open: &Vec<u32>, opened: &Vec<u32>, current_valve: u32, remaining_time: i32) -> i32 {
    to_open.iter()
        .filter(|&next| !opened.contains(next) && current_valve != *next)
        .map(|&next| {
            let dist = *dist_map.get(&(current_valve, next)).unwrap();
            if dist >= remaining_time as i32 {
                0
            } else {
                let flow = valves.get(&next).unwrap().flow;
                let mut opened_clone = opened.clone();
                opened_clone.push(current_valve);
                let next_time = remaining_time - dist - 1;
                next_time * flow + compute_best_flow(valves, dist_map, to_open, &opened_clone, next, next_time)
            }
    }).max().unwrap()
}

fn compute_paths(valves: &HashMap<u32, Valve>, dist_map: &HashMap<(u32, u32), i32>, to_open: &Vec<u32>, opened: &Vec<(u32, i32, i32)>, current_valve: u32, remaining_time: i32, res: &mut HashSet<Path>) {
    to_open.iter()
        .filter(|&next| opened.iter().find(|(n, _, _)| *n == *next).is_none() && current_valve != *next)
        .for_each(|&next| {
            let dist = *dist_map.get(&(current_valve, next)).unwrap();
            if dist < remaining_time as i32 {
                let mut opened_clone = opened.clone();
                let next_time = remaining_time - dist - 1;
                let flow = valves.get(&next).unwrap().flow * next_time;
                opened_clone.push((next, next_time, flow));
                compute_paths(valves, dist_map, to_open, &opened_clone, next, next_time, res);
            } else {
                res.insert(Path::new(opened.clone()));
            }
        });
}

fn main() {
    // Parsing
    let dict = create_dict();
    let valves = parse_input(&dict);

    // Compute distances
    let g = compute_graph(&valves);
    let start_valve = *dict.get(&String::from("AA")).unwrap();
    let mut src = non_zero_valves(&valves);
    src.push(start_valve);
    let dst = non_zero_valves(&valves);
    let dist_map = compute_distances(&g, &src, &dst);

    // Part 1
    let to_open = non_zero_valves(&valves);
    let opened = Vec::new();
    let best_flow = compute_best_flow(&valves, &dist_map, &to_open, &opened, start_valve, 30);
    println!("Best flow: {}", best_flow);

    let to_open = non_zero_valves(&valves);
    let opened = Vec::new();
    let mut res = HashSet::new();
    compute_paths(&valves, &dist_map, &to_open, &opened, start_valve, 26, &mut res);

    let mut count = 0;
    // TODO: shame on me for iterating over 1B elements!
    let best_flow = res.iter().cartesian_product(res.iter())
        .filter(|(p1, p2)| *p1 < *p2)
        .map(|(p1, p2)| {
            count += 1;
            if count % 1000000 == 0 {
                println!("... iteration {} ...", count);
            }
            p1.merged_flow(p2)
        })
        .max().unwrap();
    println!("Best flow with elephant: {}", best_flow);
}
