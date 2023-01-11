use std::fs::File;
use std::io::{self, BufRead};

fn parse_input() -> Vec<i64> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    lines.map(|line| {
        let line = line.unwrap();
        line.parse::<i64>().unwrap()
    }).collect()
}

fn move_element(indexes: &mut Vec<usize>, index: usize, val: i64) {
    let len = indexes.len();
    let pos = indexes.iter().position(|&x| x == index).unwrap();

    let period = len - 1;
    let abs_val = val.abs() as usize % period;
    let new_pos = if val >= 0 {
        (pos + abs_val) % period
    } else {
        if pos > abs_val { pos - abs_val }
        else { period + pos - abs_val }
    };

    let mut indexes_new = Vec::new();
    if new_pos > pos {
        indexes_new.extend_from_slice(&indexes[0..pos]);
        indexes_new.extend_from_slice(&indexes[pos+1..new_pos+1]);
        indexes_new.push(index);
        indexes_new.extend_from_slice(&indexes[new_pos+1..]);
    } else {
        indexes_new.extend_from_slice(&indexes[0..new_pos]);
        indexes_new.push(index);
        indexes_new.extend_from_slice(&indexes[new_pos..pos]);
        indexes_new.extend_from_slice(&indexes[pos+1..]);
    }
    *indexes = indexes_new;
}

fn decode(input: &Vec<i64>, num_mixing: usize) -> Vec<i64> {
    let len = input.len();

    // Index in the input vec -> index in the output vec
    let mut indexes = (0..len).collect();
    for _ in 0..num_mixing {
        for index in 0..len {
            let val = input[index];
            move_element(&mut indexes, index, val);
        }
    }

    (0..len).map(|x| {
        let i = indexes[x];
        input[i]
    }).collect()
}

fn compute_sum(v: &Vec<i64>) -> i64 {
    let pos_0 = v.iter().position(|x| *x == 0).unwrap();
    let len = v.len();
    v[(pos_0+1000) % len] + v[(pos_0+2000) % len] + v[(pos_0+3000) % len]
}

fn main() {
    let input = parse_input();

    // Part 1
    let output = decode(&input, 1);
    let res = compute_sum(&output);
    println!("The sum of coordinates is {}", res);

    // Part 2
    let dec_key = 811589153;
    let input: Vec<i64> = input.iter().map(|&x| x * dec_key).collect();
    let output = decode(&input, 10);
    let res = compute_sum(&output);
    println!("The sum of coordinates is {}", res);
}
