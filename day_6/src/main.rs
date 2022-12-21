use std::fs::File;
use std::io::{self, BufRead};

fn first_no_rep(s: &str, size: usize) -> usize {
    let s = s.as_bytes();
    let mut res = 0;
    for i in size-1..s.len() {
        let mut v: Vec<_> = (i+1-size..i+1).into_iter().map(|id| s[id]).collect();
        v.sort();
        v.dedup();
        if v.len() == size {
            res = i+1;
            break;
        }
    }
    
    res
}

fn start_of_packet(s: &str) -> usize {
    first_no_rep(s, 4)
}

fn start_of_message(s: &str) -> usize {
    first_no_rep(s, 14)
}

fn main() {
    let f = File::open("input/input.txt").unwrap();
    let mut lines = io::BufReader::new(f).lines();
    let line = lines.next().unwrap().unwrap();
    // Part 1
    println!("SoP: {}", start_of_packet(&line));
    // Part 2
    println!("SoM: {}", start_of_message(&line));
}
