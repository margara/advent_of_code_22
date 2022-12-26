use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use itertools::Itertools;
use crate::OP::{ADD, MUL, SQUARE};

#[derive(Debug)]
enum OP {
    ADD(usize),
    MUL(usize),
    SQUARE
}

#[derive(Debug)]
struct Monkey {
    monkey_id: usize,
    items: Vec<usize>,
    op: OP,
    test: usize,
    test_true: usize,
    test_false: usize,
    inspections: usize,
}

impl Monkey {
    fn parse(lines: &mut Lines<BufReader<File>>) -> Self {
        // Monkey
        let next = lines.next().unwrap().unwrap();
        let mut id_split = next.split_whitespace();
        let monkey_id = id_split.nth(1).unwrap()
            .replace(":", "").parse::<usize>().unwrap();

        // Items
        let next = lines.next().unwrap().unwrap();
        let items_split = next.split_whitespace();
        let items = items_split.skip(2).map(|val| {
            let val = val.replace(",", "");
            val.parse::<usize>().unwrap()
        }).collect();

        // Operation
        let next = lines.next().unwrap().unwrap();
        let mut op_line_split = next.split_whitespace();
        let op = match op_line_split.nth(4).unwrap() {
            "*" => {
                let val = op_line_split.nth(0).unwrap();
                if val.eq("old") {
                    SQUARE
                } else {
                    let val = val.parse::<usize>().unwrap();
                    MUL(val)
                }
            },
            "+" => {
                let val = op_line_split.nth(0).unwrap();
                let val = val.parse::<usize>().unwrap();
                ADD(val)
            },
            _ => {
                panic!("Unknown operation")
            }
        };

        // Test
        let next = lines.next().unwrap().unwrap();
        let mut test_split = next.split_whitespace();
        let test = test_split.nth(3).unwrap();
        let test = test.parse::<usize>().unwrap();

        // Test true
        let next = lines.next().unwrap().unwrap();
        let mut test_true_split = next.split_whitespace();
        let test_true = test_true_split.nth(5).unwrap();
        let test_true = test_true.parse::<usize>().unwrap();

        // Test false
        let next = lines.next().unwrap().unwrap();
        let mut test_false_split = next.split_whitespace();
        let test_false = test_false_split.nth(5).unwrap();
        let test_false = test_false.parse::<usize>().unwrap();

        // New line
        lines.next();

        Monkey {
            monkey_id,
            items,
            op,
            test,
            test_true,
            test_false,
            inspections: 0
        }
    }

    fn receive_messages(&mut self, mailbox: &mut HashMap<usize, Vec<usize>>) {
        let my_inbox = mailbox.entry(self.monkey_id).or_insert_with(|| Vec::new());
        my_inbox.iter().for_each(|m| self.items.push(*m));
        my_inbox.clear();
    }

    fn process_round(&mut self, mailbox: &mut HashMap<usize, Vec<usize>>, modulo: usize, div3: bool) {
        self.items.iter()
            .map(|item| self.perform_op(*item, modulo))
            .map(|item| if div3 { item / 3} else { item })
            .map(|item| (item, self.select_monkey(item)))
            .for_each(|(item, monkey)| self.sent_to_monkey(monkey, item, mailbox));
        self.inspections += self.items.len() as usize;
        self.items.clear();
    }

    fn perform_op(&self, item: usize, modulo: usize) -> usize {
        match self.op {
            ADD(x) => (item + x) % modulo,
            MUL(x) => (item * x) % modulo,
            SQUARE => (item * item) % modulo,
        }
    }

    fn select_monkey(&self, item: usize) -> usize {
        if item % self.test == 0 {
            self.test_true
        } else {
            self.test_false
        }
    }

    fn sent_to_monkey(&self, monkey: usize, item: usize, mailbox: &mut HashMap<usize, Vec<usize>>) {
        mailbox.entry(monkey)
            .or_insert_with(|| Vec::new())
            .push(item);
    }
}

fn parse_input() -> Vec<Monkey> {
    let f = File::open("input/input.txt").unwrap();
    let mut lines = io::BufReader::new(f).lines();
    (0..8).map(|_| Monkey::parse(&mut lines)).collect()
}

fn compute_monkey_business(monkeys: &mut Vec<Monkey>) -> usize {
    monkeys.iter()
        .map(|monkey| monkey.inspections)
        .sorted()
        .rev()
        .take(2)
        .product::<usize>()
}

fn make_rounds(mut monkeys: &mut Vec<Monkey>, num_rounds: usize, div3: bool) {
    let modulo = monkeys.iter().map(|m| m.test).product::<usize>();
    let mut mailbox: HashMap<usize, Vec<usize>> = HashMap::new();
    (0..num_rounds).for_each(|_| {
        monkeys.iter_mut().for_each(|monkey| {
            monkey.receive_messages(&mut mailbox);
            monkey.process_round(&mut mailbox, modulo, div3);
        });
    });
}

fn main() {
    // Part 1
    let mut monkeys = parse_input();
    make_rounds(&mut monkeys, 20, true);
    let monkey_business = compute_monkey_business(&mut monkeys);
    println!("Monkey business (part 1): {}", monkey_business);

    // Part 2
    let mut monkeys = parse_input();
    make_rounds(&mut monkeys, 10000, false);
    let monkey_business = compute_monkey_business(&mut monkeys);
    println!("Monkey business (part 1): {}", monkey_business);
}


