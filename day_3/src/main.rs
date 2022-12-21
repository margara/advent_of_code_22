use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn get_priority(c: char) -> u32 {
    if c.is_lowercase() { c as u32 - 'a' as u32 + 1 }
    else { c as u32 - 'A' as u32 + 27 }
}

fn main() {

    // Part 1
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    let tot: u32 = lines.map(|line| {
        let mut l1 = line.unwrap();
        let l2 = l1.split_off(l1.len() / 2);
        for c in l1.chars() {
            if let Some(_) = l2.chars().find(|x| *x == c) {
                return get_priority(c)
            }
        }
        0
    }).sum();

    println!("{tot}");

    // Part 2
    let f = File::open("input/input.txt").unwrap();
    let mut lines = io::BufReader::new(f).lines();

    let mut tot: u32 = 0;
    loop {
        let line1 = lines.next();
        let mut s1 = HashSet::new();
        match line1 {
            None => {
                break;
            },
            Some(line) => {
                line.unwrap().chars().for_each(|c| { s1.insert(c); });
            }
        }
        let mut s2 = HashSet::new();
        lines.next().unwrap().unwrap().chars().for_each(|c| { s2.insert(c); });
        let s2: HashSet<_> = s1.intersection(& s2).map(|c| *c).collect();

        let mut s3 = HashSet::new();
        lines.next().unwrap().unwrap().chars().for_each(|c| { s3.insert(c); });
        let s3: HashSet<_> = s2.intersection(& s3).map(|c| *c).collect();

        tot += s3.into_iter().map(get_priority).sum::<u32>();
    }

    println!("{tot}");
}
