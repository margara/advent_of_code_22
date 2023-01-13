use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use rayon::prelude::*;

const NUM_RES: usize = 4;

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

// Amount of resources
type Resources = [u8; NUM_RES];

// Each row represents the cost for a robot
type Blueprint = [Resources; NUM_RES];

#[derive(Clone, Debug)]
struct State {
    remaining_time: u8,
    available_res: Resources,
    available_robots: Resources,
}

fn parse_input() -> HashMap<usize, Blueprint> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();

    lines.map(|line| {
        let line = line.unwrap();
        let mut split = line.split_whitespace();

        let id = split.nth(1).unwrap();
        let id = id.replace(":", "").parse::<usize>().unwrap();

        let ore_cost = split.nth(4).unwrap();
        let ore_cost = ore_cost.parse::<u8>().unwrap();
        let mut ore = [0; NUM_RES];
        ore[ORE] = ore_cost;

        let ore_cost = split.nth(5).unwrap();
        let ore_cost = ore_cost.parse::<u8>().unwrap();
        let mut clay = [0; NUM_RES];
        clay[ORE] = ore_cost;

        let ore_cost = split.nth(5).unwrap();
        let ore_cost = ore_cost.parse::<u8>().unwrap();
        let clay_cost = split.nth(2).unwrap();
        let clay_cost = clay_cost.parse::<u8>().unwrap();
        let mut obsidian = [0; NUM_RES];
        obsidian[ORE] = ore_cost;
        obsidian[CLAY] = clay_cost;

        let ore_cost = split.nth(5).unwrap();
        let ore_cost = ore_cost.parse::<u8>().unwrap();
        let obsidian_cost = split.nth(2).unwrap();
        let obsidian_cost = obsidian_cost.parse::<u8>().unwrap();
        let mut geode = [0; NUM_RES];
        geode[ORE] = ore_cost;
        geode[OBSIDIAN] = obsidian_cost;

        let blueprint = [ore, clay, obsidian, geode];
        (id, blueprint)
    }).collect()
}

impl State {
    fn new(remaining_time: u8) -> Self {
        Self {
            remaining_time,
            available_res: [0; NUM_RES],
            available_robots: [1, 0, 0, 0],
        }
    }

    fn can_build(&self, res: usize, blueprint: &Blueprint) -> bool {
        for i in 0..NUM_RES {
            if self.available_res[i] < blueprint[res][i] {
                return false;
            }
        }
        true
    }

    // Requires: can_build(res, blueprint) == true
    fn build(&self, res: usize, blueprint: &Blueprint) -> Self {
        let mut new_state = self.clone();
        new_state.remaining_time -= 1;
        for i in 0..NUM_RES {
            new_state.available_res[i] -= blueprint[res][i];
            new_state.available_res[i] += new_state.available_robots[i];
        }
        new_state.available_robots[res] += 1;
        new_state
    }

    fn advance_time(&self) -> Self {
        let mut new_state = self.clone();
        new_state.remaining_time -= 1;
        for i in 0..NUM_RES {
            new_state.available_res[i] += new_state.available_robots[i];
        }
        new_state
    }

    fn remaining_time(&self) -> u8 {
        self.remaining_time
    }

    fn num_geodes(&self) -> u8 {
        self.available_res[GEODE]
    }
}

fn max_geodes(state: &State, blueprint: &Blueprint, parallel: bool) -> u8 {
    if state.remaining_time() == 0 {
        state.num_geodes()
    } else {
        // Reachable states assuming we can build a single robot at each time
        let mut new_states = (0..NUM_RES)
            .filter(|res| state.can_build(*res, blueprint))
            .map(|res| state.build(res, blueprint))
            .collect::<Vec<_>>();
        new_states.push(state.advance_time());

        if parallel && state.remaining_time() > 2 {
            new_states.par_iter()
                .map(|s| max_geodes(s, blueprint, parallel))
                .max().unwrap()
        } else {
            new_states.iter()
                .map(|s| max_geodes(s, blueprint, parallel))
                .max().unwrap()
        }
    }
}

// The simplified version uses a heuristics
// (I could not prove it is always correct, but it brings the right results for part 1 and 2)
fn max_geodes_simplified(state: &State, blueprint: &Blueprint, parallel: bool) -> u8 {
    if state.remaining_time() == 0 {
        state.num_geodes()
    } else {
        let new_states = if state.can_build(GEODE, blueprint) {
            Vec::from([state.build(GEODE, blueprint)])
        } else if state.can_build(OBSIDIAN, blueprint) {
            Vec::from([state.build(OBSIDIAN, blueprint)])
        } else {
            let mut new_states= Vec::new();
            if state.can_build(CLAY, blueprint) {
                new_states.push(state.build(CLAY, blueprint));
            }
            if state.can_build(ORE, blueprint) {
                new_states.push(state.build(ORE, blueprint));
            }
            new_states.push(state.advance_time());
            new_states
        };

        if parallel && state.remaining_time() > 2 {
            new_states.par_iter()
                .map(|s| max_geodes_simplified(s, blueprint, parallel))
                .max().unwrap()
        } else {
            new_states.iter()
                .map(|s| max_geodes_simplified(s, blueprint, parallel))
                .max().unwrap()
        }
    }
}

fn compute_max_geodes(id: usize, remaining_time: u8, blueprint: &Blueprint, parallel: bool, simplified: bool) -> (usize, usize) {
    let init = State::new(remaining_time);
    let max_geodes = if simplified {
        max_geodes_simplified(&init, blueprint, parallel)
    } else {
        max_geodes(&init, blueprint, parallel)
    } as usize;
    let quality_level = id * max_geodes;
    println!("Blueprint {}, geodes {}, quality level {}", id, max_geodes, quality_level);

    (max_geodes, quality_level)
}

fn main() {
    let blueprints = parse_input();
    println!("Blueprints");
    blueprints.iter().for_each(|(id, blueprint)| {
       println!("{:?} {:?}", id, blueprint);
    });
    println!();

    // Part 1
    let res = blueprints.par_iter()
        .map(|(id, blueprint)| {
            compute_max_geodes(*id, 24, blueprint, false, true)
        })
        .map(|(_geodes, quality)| quality)
        .sum::<usize>();
    println!("Sum of the quality levels: {res}");

    // Part 2
    let res = blueprints.par_iter()
        .filter(|(id, _)| **id <= 3)
        .map(|(id, blueprint)| {
            compute_max_geodes(*id, 32, blueprint, true, true)
        })
        .map(|(geodes, _quality)| geodes)
        .product::<usize>();
    println!("Product of the number of geodes: {res}");
}