use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use crate::Cmd::{CD, DIR, FILE, LS};

#[derive(Debug)]
enum Cmd {
    CD {
        path: String
    },
    LS,
    FILE {
        size: usize,
        name: String,
    },
    DIR {
        name: String,
    }
}

#[derive(Debug)]
struct Node {
    name: String,
    size: usize,
    children: HashMap<String, Node>,
    dir: bool,
}

impl Node {
    fn new_dir(name: &str) -> Self {
        Self {
            name: String::from(name),
            size: 0,
            children: HashMap::new(),
            dir: true
        }
    }

    fn new_file(name: &str, size: usize) -> Self {
        Self {
            name: String::from(name),
            size,
            children: HashMap::new(),
            dir: false
        }
    }

    fn parse(&mut self, it: &mut impl Iterator<Item = Cmd>) {
        if let Some(cmd) = it.next() {
            match cmd {
                LS => { },
                CD { path } => {
                    match path.as_str() {
                        ".." => { return; },
                        "/" => { },
                        _ => {
                            let dir = self.children.get_mut(&path).expect("Dir not listed");
                            dir.parse(it);
                            self.size += dir.size;
                        }
                    }
                },
                DIR { name} => {
                    self.children.insert(name.clone(), Node::new_dir(&name.clone()));
                },
                FILE { size, name } => {
                    self.children.insert(name.clone(), Node::new_file(&name.clone(), size));
                    self.size += size;
                }
            }
            self.parse(it);
        }
    }

    fn print_visit(&self) {
        println!("{} --> {:?}", self.name, self.children);
        self.children.iter().for_each(|(_, child)| child.print_visit());
    }

    fn part1_visit(&self, acc: &mut usize) {
        if self.dir && self.size < 100000 {
            *acc += self.size;
        }
        self.children.iter().for_each(|(_, child)| child.part1_visit(acc));
    }

    fn part2_visit(&self, to_delete: usize, acc: &mut usize) {
        if self.dir && self.size >= to_delete && self.size < *acc {
            *acc = self.size;
        }
        self.children.iter().for_each(|(_, child)| child.part2_visit(to_delete, acc));
    }
}

fn parse_line(line: &str) -> Cmd {
    match &line[0..4] {
        "$ cd" => CD {
            path: String::from(&line[5..line.len()])
        },
        "$ ls" => LS,
        "dir " => DIR {
            name: String::from(&line[4..line.len()])
        },
        _ => {
            let mut it = line.split_whitespace();
            let size = it.next().unwrap().parse::<usize>().unwrap();
            let name = String::from(it.next().unwrap());
            FILE {
                size,
                name,
            }
        }
    }
}

fn parse_input() -> impl Iterator<Item = Cmd> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    lines.map(|line| parse_line(&line.unwrap()))
}

fn main() {
    let mut input_iter = parse_input();
    let mut root = Node::new_dir("/");
    root.parse(&mut input_iter);
    root.print_visit();

    // Part 1
    let mut part1_res: usize = 0;
    root.part1_visit(&mut part1_res);
    println!("Part 1: {}", part1_res);

    // Part 2
    let disk_space: usize = 70000000;
    let required_space: usize = 30000000;
    let max_occupied: usize = disk_space - required_space;
    let currently_occupied: usize = root.size;
    let to_delete = currently_occupied - max_occupied;
    println!("Current size: {} - Max: {} - To delete: {}", currently_occupied, max_occupied, to_delete);
    let mut part2_res: usize = root.size;
    root.part2_visit(to_delete, &mut part2_res);
    println!("Part 2: {}", part2_res);
}
