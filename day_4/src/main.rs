use std::fs::File;
use std::io::{self, BufRead};

fn total_overlap(a: (u32, u32), b: (u32, u32)) -> bool {
    a.0 <= b.0 && a.1 >= b.1 || b.0 <= a.0 && b.1 >= a.1
}

fn partial_overlap(a: (u32, u32), b: (u32, u32)) -> bool {
    a.0 <= b.0 && a.1 >= b.0 || b.0 <= a.0 && b.1 >= a.0
}

fn main() {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();

    let mut total: u32 = 0;
    let mut partial: u32 = 0;

    for line in lines {
        let line = line.unwrap();
        let mut split = line.split(",");
        let elf1 = split.next().unwrap().split_once("-").unwrap();
        let elf1 = (elf1.0.parse::<u32>().unwrap(), elf1.1.parse::<u32>().unwrap());
        let elf2 = split.next().unwrap().split_once("-").unwrap();
        let elf2 = (elf2.0.parse::<u32>().unwrap(), elf2.1.parse::<u32>().unwrap());

        if total_overlap(elf1, elf2) {
            total += 1;
        }
        if partial_overlap(elf1, elf2) {
            partial += 1;
        }
    }

    // Part 1
    println!("Total overlap: {total}");
    // Part 2
    println!("Partial overlap: {partial}");
}
