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

fn main() {
    let mut input = parse_input();

    input.sort_by(|(x1, y1, z1), (x2, y2, z2)| (y1, z1).cmp(&(y2, z2)));
    let connected_x = input.iter()
        .group_by(|(_x, y, z)| (y, z))
        .into_iter()
        .map(|(_key, points)| points
            .into_iter()
            .map(|(x, _y, _z)| x)
            .sorted()
            .adjacent_pairs()
            .filter(|(&x1, &x2)| x1.abs_diff(x2) == 1)
            .count() * 2)
        .sum::<usize>();

    input.sort_by(|(x1, y1, z1), (x2, y2, z2)| (x1, z1).cmp(&(x2, z2)));
    let connected_y = input.iter()
        .group_by(|(x, _y, z)| (x, z))
        .into_iter()
        .map(|(_key, points)| points
            .into_iter()
            .map(|(_x, y, _z)| y)
            .sorted()
            .adjacent_pairs()
            .filter(|(&y1, &y2)| y1.abs_diff(y2) == 1)
            .count() * 2)
        .sum::<usize>();

    input.sort_by(|(x1, y1, z1), (x2, y2, z2)| (x1, y1).cmp(&(x2, y2)));
    let connected_z = input.iter()
        .group_by(|(x, y, _z)| (x, y))
        .into_iter()
        .map(|(_key, points)| points
            .into_iter()
            .map(|(_x, _y, z)| z)
            .sorted()
            .adjacent_pairs()
            .filter(|(&z1, &z2)| z1.abs_diff(z2) == 1)
            .count() * 2)
        .sum::<usize>();

    let tot_faces = input.len() * 6;
    let connected = connected_x + connected_y + connected_z;
    let not_connected = tot_faces - connected;

    println!("Faces connected over x: {}", connected_x);
    println!("Faces connected over y: {}", connected_y);
    println!("Faces connected over z: {}", connected_z);
    println!("Total number of faces: {}. Connected: {}. Not connected: {}", tot_faces, connected, not_connected);


}
