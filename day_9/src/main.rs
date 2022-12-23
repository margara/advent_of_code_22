use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use crate::Move::{R, L, U, D};

#[derive(Debug)]
enum Move {
    R, L, U, D
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn apply_move(&self, m: &Move) -> Pos {
        match m {
            R => Pos { x: self.x+1, y: self.y },
            L => Pos { x: self.x-1, y: self.y },
            U => Pos { x: self.x, y: self.y+1 },
            D => Pos { x: self.x, y: self.y-1 },
        }
    }

    fn is_close(&self, other: &Pos) -> bool {
        self.x.abs_diff(other.x) <= 1 &&
            self.y.abs_diff(other.y) <= 1
    }

    fn catch_up(&self, other: &Pos) -> Pos {
        if self.is_close(other) {
            self.clone()
        } else {
            let diff_x = other.x - self.x;
            let diff_y = other.y - self.y;
            let to_add_x = if diff_x != 0 { diff_x / diff_x.abs() } else { 0 };
            let to_add_y = if diff_y != 0 { diff_y / diff_y.abs() } else { 0 };
            Pos { x: self.x + to_add_x, y: self.y + to_add_y }
        }
    }
}

struct Rope {
    knots: Vec<Pos>,
    len: usize
}

impl Rope {
    fn new(len: usize) -> Self {
        Self {
            knots: vec![Pos::default(); len],
            len
        }
    }

    fn apply_move(&mut self, m: &Move) {
        self.knots[0] = self.knots[0].apply_move(m);
        for i in 1..self.len {
            self.knots[i] = self.knots[i].catch_up(&self.knots[i-1]);
        }
    }

    fn get_tail_position(&self) -> Pos {
        self.knots[self.len-1].clone()
    }
}

fn parse_input() -> Vec<Move> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    lines.flat_map(|line| {
        let line = line.unwrap();
        let dist = &line[2..].parse::<u32>().unwrap();
        (0..*dist).map(move |_| {
            match &line[0..1] {
                "R" => R,
                "L" => L,
                "U" => U,
                "D" => D,
                _ => panic!("Unknown command")
            }
        }).into_iter()
    }).collect()
}

fn compute_visited(rope_len: usize, input: &Vec<Move>) -> usize {
    let mut rope = Rope::new(rope_len);
    let mut visited = HashSet::from([rope.get_tail_position()]);
    input.iter().for_each(|m| {
        rope.apply_move(m);
        visited.insert(rope.get_tail_position());
    });
    visited.len()
}

fn main() {
    let input = parse_input();
    // Part 1
    println!("Visited positions (rope len=2): {}", compute_visited(2, &input));
    // Part 2
    println!("Visited positions (rope len=10): {}", compute_visited(10, &input));
}
