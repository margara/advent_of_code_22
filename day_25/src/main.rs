use std::fs::File;
use std::io::{self, BufRead};

static BASE: i64 = 5;

fn parse_input() -> Vec<i64> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    lines.map(|line| {
        let line = line.unwrap();
        snafu_to_dec(&line)
    }).collect()
}

fn snafu_to_dec(num: &str) -> i64 {
    let len = num.len();
    num.char_indices().map(|(i, c)| {
        let i = (len - i - 1) as u32;
        let pow: i64 = BASE.pow(i);
        let coeff = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '-' => -1,
            '=' => -2,
            _ => panic!("Unknown digit {}", c),
        };
        coeff * pow
    }).sum()
}

fn snafu_inc_digit(n: &mut Vec<char>, d: usize) {
    let len = n.len();
    if d < len {
        let c = n[d];
        match c {
            '0' => n[d] = '1',
            '1' => n[d] = '2',
            '2' => {
                n[d] = '=';
                snafu_inc_digit(n, d + 1);
            },
            '=' => n[d] = '-',
            '-' => n[d] = '0',
            _ => panic!("Unknown digit"),
        }
    } else {
        for i in 0..len {
            n[i] = '='
        }
        n.push('1');
    }
}

// Not proud of this. Will need to fix for part 2.
fn dec_to_snafu_count(num: i64) -> String {
    let mut n = vec!['0'];
    for i in 0..num {
        if i % 1000000000 == 0 {
            println!("{} %", i as f64 * 100.0 / num as f64);
        }
        snafu_inc_digit(&mut n, 0);
    }
    n.iter().rev().collect()
}

fn main() {
    let input = parse_input();

    // Part 1
    let sum = input.iter().sum::<i64>();
    println!("The decimal sum is: {}", sum);
    let sum = dec_to_snafu_count(sum/5);
    println!("The SNAFU sum is: {}0", sum);
}
