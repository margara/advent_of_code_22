use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use itertools::Itertools;

#[derive(Debug)]
struct Valve {
    name: String,
    flow: u32,
    tunnels: Vec<String>,
}

impl Valve {
    pub fn new(name: String, flow: u32, tunnels: Vec<String>) -> Self {
        Self { name, flow, tunnels }
    }
}

fn parse_valve(line: String) -> Valve {
    let mut split = line.split_whitespace();

    let name = split.nth(1).unwrap().to_string();

    let rate = split.nth(2).unwrap();
    let rate = rate.replace("rate=", "").replace(";", "");
    let rate = rate.parse::<u32>().unwrap();

    let mut tunnels = Vec::new();
    let _next = split.nth(3);
    while let Some(tunnel) = split.next() {
        let tunnel = tunnel.replace(",", "");
        tunnels.push(tunnel);
    }

    Valve::new(name, rate, tunnels)
}

fn parse_input() -> HashMap<String, Valve> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    lines.map(|line| {
        let line = line.unwrap();
        let valve = parse_valve(line);
        (valve.name.clone(), valve)
    }).collect()
}

fn compute_best_flow(valves: &HashMap<String, Valve>, opened: &mut HashSet<String>, valve: &Valve, remaining_time: u32) -> u32 {
    match remaining_time {
        1 => 0,
        _ => {
            let move_flow = valve.tunnels.iter()
                .map(|tunnel| {
                    let reached_valve = valves.get(tunnel).unwrap();
                    let mut opened_clone = opened.clone();
                    opened_clone.insert(valve.name.clone());
                    compute_best_flow(valves, &mut opened_clone, reached_valve, remaining_time - 1)
                })
                .max()
                .unwrap_or(0);

            if valve.flow == 0 || opened.contains(&valve.name) {
                move_flow
            } else {
                let mut opened_clone = opened.clone();
                opened_clone.insert(valve.name.clone());
                let open_flow = valve.flow * (remaining_time - 1) + compute_best_flow(valves, &mut opened_clone, valve, remaining_time - 1);
                max(open_flow, move_flow)
            }
        }
    }
}

fn compute_best_flow_with_elephant(valves: &HashMap<String, Valve>, opened: &mut HashSet<String>, v1: &Valve, v2: &Valve, remaining_time: u32) -> u32 {
    match remaining_time {
        1 => 0,
        _ => {
            if opened.len() == valves.len() {
                return 0;
            }

            let reachable_from_v1 = v1.tunnels.iter()
                .filter(|&tunnel| *tunnel != v2.name);
            let reachable_from_v2 = v2.tunnels.iter()
                .filter(|&tunnel| *tunnel != v1.name);
            let reachable = reachable_from_v1.cartesian_product(reachable_from_v2)
                .filter(|(t1, t2)| *t1 != *t2);

            let both_move = reachable
                .map(|(t1, t2)| {
                    let r1 = valves.get(t1).unwrap();
                    let r2 = valves.get(t2).unwrap();
                    let mut opened_clone = opened.clone();
                    compute_best_flow_with_elephant(valves, &mut opened_clone, r1, r2, remaining_time - 1)
                })
                .max()
                .unwrap_or(0);

            if (v1.flow == 0 || opened.contains(&v1.name)) && (v2.flow == 0 || opened.contains(&v2.name)) {
                return both_move;
            }

            let flow2 = v2.flow * (remaining_time - 1);
            let flow1 = v1.flow * (remaining_time - 1);
            let compute_only1_moves = v1.flow != 0 && !opened.contains(&v1.name);
            let compute_only2_moves = v2.flow != 0 && !opened.contains(&v2.name);

            let only1_moves = if compute_only1_moves {
                v1.tunnels.iter()
                    .map(|t1| {
                        let r1 = valves.get(t1).unwrap();
                        let mut opened_clone = opened.clone();
                        opened_clone.insert(v2.name.clone());
                        compute_best_flow_with_elephant(valves, &mut opened_clone, r1, v2, remaining_time - 1)
                    })
                    .max()
                    .unwrap_or(0) + flow2
            } else {
                0
            };

            let only2_moves = if compute_only2_moves {
                v2.tunnels.iter()
                    .map(|t2| {
                        let r2 = valves.get(t2).unwrap();
                        let mut opened_clone = opened.clone();
                        opened_clone.insert(v1.name.clone());
                        compute_best_flow_with_elephant(valves, &mut opened_clone,  v1, r2, remaining_time - 1)
                    })
                    .max()
                    .unwrap_or(0) + flow1
            } else {
                0
            };

            if !compute_only2_moves {
                only1_moves
            } else if !compute_only1_moves {
                only2_moves
            } else {
                let mut opened_clone = opened.clone();
                opened_clone.insert(v1.name.clone());
                opened_clone.insert(v2.name.clone());
                let both_open = flow1 + flow2 + compute_best_flow_with_elephant(valves, &mut opened_clone, v1, v2, remaining_time - 1);

                max(max(only1_moves, only2_moves), max(both_move, both_open))
            }
        }
    }
}

fn main() {
    let valves = parse_input();
    let start_valve = valves.get(&String::from("AA")).unwrap();

    // Part 1
    let mut opened = HashSet::new();
    let best_flow = compute_best_flow(&valves, &mut opened, start_valve, 30);
    println!("Best flow: {}", best_flow);

    // Part 2
    let mut opened = HashSet::new();
    let best_flow = compute_best_flow_with_elephant(&valves, &mut opened, start_valve, start_valve, 26);
    println!("Best flow with elephant: {}", best_flow);
}
