use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn init() -> HashMap<usize, Vec<char>> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();

    let init: Vec<String> = lines.map_while(| line| {
        let line = line.unwrap();
        if line.len() <= 1 { None }
        else { Some(line) }
    }).collect();

    let mut res = HashMap::new();
    init.into_iter()
        .rev()
        .skip(1)
        .for_each(| s| {
            s.char_indices()
                .skip(1)
                .step_by(4)
                .map(|(id, c)| ((id-1)/4+1, c))
                .filter(|(_id, c)| c.is_alphabetic()).
                for_each(|(id, c)|
                    res.entry(id)
                        .or_insert_with(Vec::new)
                        .push(c));
        });

    res
}

fn moves() -> Vec<(usize, usize, usize)> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();

    let mut res = Vec::new();

    for line in lines {
        let line = line.unwrap();
        if line.contains("move") {
            let line = line.replace("move ", "");
            let line = line.replace("from ", "");
            let line = line.replace("to ", "");
            let mut split = line.split(" ");
            let num = split.next().unwrap().parse::<usize>().unwrap();
            let from = split.next().unwrap().parse::<usize>().unwrap();
            let to = split.next().unwrap().parse::<usize>().unwrap();
            res.push((num, from, to));
        }
    }

    res
}

fn do_move(m: (usize, usize, usize), stacks: &mut HashMap<usize, Vec<char>>) {
    for _i in 0..m.0 {
        let c = stacks.get_mut(&m.1).unwrap().pop().unwrap();
        stacks.get_mut(&m.2).unwrap().push(c);
    }
}

fn do_move_9001(m: (usize, usize, usize), stacks: &mut HashMap<usize, Vec<char>>) {
    let mut to_move : Vec<char> = (0..m.0).into_iter()
        .map(|_| stacks.get_mut(&m.1).unwrap().pop().unwrap())
        .collect();
    to_move.reverse();
    stacks.get_mut(&m.2).unwrap().extend(to_move);
}

fn print_top(stacks: & HashMap<usize, Vec<char>>) {
    for i in 1..10 {
        print!("{}", stacks.get(&i).unwrap().last().unwrap())
    }
    println!();
}

fn main() {
    let moves = moves();

    // Part 1
    let mut stacks = init();
    moves.iter().for_each(|&m| do_move(m, &mut stacks));
    print_top(&stacks);

    // Part 2
    let mut stacks = init();
    moves.iter().for_each(|&m| do_move_9001(m, &mut stacks));
    print_top(&stacks);
}
