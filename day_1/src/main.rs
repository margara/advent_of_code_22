use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();

    let mut count = 0;
    let mut elves = Vec::new();

    for line in lines {
        let s: String = line.unwrap().into();
        if s.is_empty() {
            elves.push(count);
            count = 0;
        } else {
            let c = s.parse::<u32>().unwrap();
            count += c;
        }
    }

    // First part
    println!("Max: {}", elves.iter().max().unwrap());

    // Second part
    elves.sort();
    println!("Sum first three: {}", elves.iter().rev().take(3).sum::<u32>());
}
