use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use adjacent_pair_iterator::AdjacentPairIterator;
use itertools::Itertools;

fn parse_input() -> Vec<(i32, i32, i32)> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();

    lines.map(|line| {
        let line = line.unwrap();
        line.split(",")
            .map(|el| el.parse::<i32>().unwrap())
            .collect_tuple().unwrap()
    }).collect()
}

fn connected(input: &mut Vec<(i32, i32, i32)>,
             key: fn((i32, i32, i32)) -> (i32, i32),
             val: fn((i32, i32, i32)) -> i32)
    -> usize {

    input.sort_by(|p1, p2| key(*p1).cmp(&key(*p2)));
    input.iter()
        .group_by(|&p| key(*p))
        .into_iter()
        .map(|(_key, points)| points
            .into_iter()
            .map(|&p| val(p))
            .sorted()
            .adjacent_pairs()
            .filter(|(v1, v2)| v1.abs_diff(*v2) == 1)
            .count() * 2)
        .sum::<usize>()
}

fn compute_connected(mut input: &mut Vec<(i32, i32, i32)>) {
    let connected_x = connected(&mut input, |(_x, y, z)| (y, z), |(x, _y, _z)| x);
    let connected_y = connected(&mut input, |(x, _y, z)| (x, z), |(_x, y, _z)| y);
    let connected_z = connected(&mut input, |(x, y, _z)| (x, y), |(_x, _y, z)| z);

    let tot_faces = input.len() * 6;
    let connected = connected_x + connected_y + connected_z;
    let not_connected = tot_faces - connected;

    println!("Faces connected over x: {}", connected_x);
    println!("Faces connected over y: {}", connected_y);
    println!("Faces connected over z: {}", connected_z);
    println!("Total number of faces: {}. Connected: {}. Not connected: {}", tot_faces, connected, not_connected);
}

fn reachable_from(input: &HashSet<(i32, i32, i32)>, current: &HashSet<(i32, i32, i32)>, all: &mut HashSet<(i32, i32, i32)>, max_x: i32, max_y: i32, max_z: i32) {
    fn check_reachable(input: &HashSet<(i32, i32, i32)>, current: &HashSet<(i32, i32, i32)>, all: &mut HashSet<(i32, i32, i32)>, new: &mut HashSet<(i32, i32, i32)>, r: &(i32, i32, i32)) {
        if !all.contains(r) && !current.contains(r) && !input.contains(r) {
            new.insert(*r);
            all.insert(*r);
        }
    }

    let mut new = HashSet::new();
    current.iter().for_each(|(x, y, z)| {
        if *x > 0 { check_reachable(input, current, all, &mut new, &(*x-1, *y, *z)) }
        if *y > 0 { check_reachable(input, current, all, &mut new, &(*x, *y-1, *z)) }
        if *z > 0 { check_reachable(input, current, all, &mut new, &(*x, *y, *z-1)) }
        if *x < max_x { check_reachable(input, current, all, &mut new, &(*x+1, *y, *z)) }
        if *y < max_y { check_reachable(input, current, all, &mut new, &(*x, *y+1, *z)) }
        if *z < max_z { check_reachable(input, current, all, &mut new, &(*x, *y, *z+1)) }
    });

    if !new.is_empty() {
        reachable_from(input, &mut new, all, max_x, max_y, max_z);
    }
}

fn compute_reachable(input: &HashSet<(i32, i32, i32)>) -> HashSet<(i32, i32, i32)> {
    let current = HashSet::from([(0, 0, 0)]);
    let mut reachable = HashSet::new();
    let max_x = input.iter().map(|(x, _y, _z)| *x).max().unwrap() + 1;
    let max_y = input.iter().map(|(_x, y, _z)| *y).max().unwrap() + 1;
    let max_z = input.iter().map(|(_x, _y, z)| *z).max().unwrap() + 1;
    reachable_from(input, &current, &mut reachable, max_x, max_y, max_z);
    reachable
}

fn all(input: &HashSet<(i32, i32, i32)>) -> HashSet<(i32, i32, i32)> {
    let max_x = input.iter().map(|(x, _y, _z)| *x).max().unwrap() + 1;
    let max_y = input.iter().map(|(_x, y, _z)| *y).max().unwrap() + 1;
    let max_z = input.iter().map(|(_x, _y, z)| *z).max().unwrap() + 1;
    let mut res = HashSet::new();
    for x in 0..max_x {
        for y in 0..max_y {
            for z in 0..max_z {
                res.insert((x, y, z));
            }
        }
    }
    res
}

fn main() {
    let mut input = parse_input();
    let mut input_set = input.iter().cloned().collect::<HashSet<_>>();

    // Part 1
    compute_connected(&mut input);

    // Part 2
    let all = all(&input_set);
    let reachable = compute_reachable(&input_set);
    let unreachable = all.difference(&reachable).cloned().collect::<HashSet<_>>();
    let mut input_with_unreachable = input_set.union(&unreachable).cloned().collect::<Vec<_>>();
    compute_connected(&mut input_with_unreachable);
}


