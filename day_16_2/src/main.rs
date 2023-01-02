use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use petgraph::graph::{NodeIndex, UnGraph};
use itertools::Itertools;
use petgraph::algo::dijkstra;

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
}
