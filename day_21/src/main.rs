use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use crate::Val::{Num, Var};
use crate::Op::{Add, Div, Mul, Sub};

#[derive(Debug)]
enum Val {
    Num(i64),
    Var(String),
}

#[derive(Debug)]
enum Op {
    Add, Sub, Mul, Div
}

#[derive(Debug)]
struct Monkey {
    lhs: Val,
    rhs: Val,
    op: Op,
}

impl Monkey {
    pub fn new(lhs: Val, rhs: Val, op: Op) -> Self {
        Self { lhs, rhs, op }
    }
}

fn parse_input() -> (HashMap<String, Monkey>, HashMap<String, i64>) {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();

    let mut monkeys = HashMap::new();
    let mut numbers = HashMap::new();

    lines.for_each(|line| {
        let line = line.unwrap();
        let mut split = line.split_whitespace().collect::<Vec<_>>();
        let name = split[0].clone().replace(":", "");
        let monkey = if split.len() == 2 {
            let val = split[1].parse::<i64>().expect("Invalid value");
            numbers.insert(name, val);
        } else {
            let v1 = split[1].to_string();
            let op = match split[2] {
                "+" => Add,
                "-" => Sub,
                "*" => Mul,
                "/" => Div,
                _ => panic!("Unknown operation")
            };
            let v2 = split[3].to_string();
            let monkey = Monkey::new(Var(v1), Var(v2), op);
            monkeys.insert(name, monkey);
        };
    });

    (monkeys, numbers)
}

fn compute_value_for_root(monkeys: &mut HashMap<String, Monkey>, numbers: &mut HashMap<String, i64>) -> i64 {
    fn get_number(v: &Val, numbers: &HashMap<String, i64>) -> Option<i64> {
        match v {
            Num(n) => Some(*n),
            Var(name) => numbers.get(name).cloned(),
        }
    }

    while !monkeys.is_empty() {
        let mut to_remove = Vec::new();
        monkeys.iter().for_each(|(name, monkey)| {
            let lhs = get_number(&monkey.lhs, numbers);
            let rhs = get_number(&monkey.rhs, numbers);

            if let Some(lhs) = lhs {
                if let Some(rhs) = rhs {
                    let res = match monkey.op {
                        Add => lhs + rhs,
                        Sub => lhs - rhs,
                        Mul => lhs * rhs,
                        Div => lhs / rhs,
                    };
                    numbers.insert(name.clone(), res);
                    to_remove.push(name.clone());
                }
            }
        });
        to_remove.iter().for_each(|x| {
            monkeys.remove(x).unwrap();
        });
    }

    let root_val = *numbers.get(&String::from("root")).unwrap();
    root_val
}

// Not particularly proud of this solution, but quick and dirty.
// Found a starting point with larger steps and then decreased the granularity
fn find_missing_value() -> i64 {
    let root = String::from("root");
    let humn = String::from("humn");
    let mut res = 0;
    for i in 3876907160000.. {
        let (mut monkeys, mut numbers) = parse_input();
        let mut new_root = monkeys.remove(&root).unwrap();
        new_root.op = Sub;
        monkeys.insert(root.clone(), new_root);
        numbers.remove(&humn);
        numbers.insert(humn.clone(), i);
        let root_val = compute_value_for_root(&mut monkeys, &mut numbers);
        println!("i: {}, root: {}", i, root_val);
        if root_val == 0 {
            res = i;
            break
        }
    }

    res
}

fn main() {
    let (mut monkeys, mut numbers) = parse_input();

    // Part 1
    let root_val = compute_value_for_root(&mut monkeys, &mut numbers);
    println!("Value of root: {root_val}.");

    // Part 2
    let missing_val = find_missing_value();
    println!("Missing val: {missing_val}.");
}
