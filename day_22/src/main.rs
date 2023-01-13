use std::fs::File;
use std::io::{self, BufRead};
use array2d::Array2D;
use crate::Command::{Forward, Left, Right};
use crate::Tile::{Blank, Open, Wall};
use crate::Dir::{U, R, D, L};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    Blank, Open, Wall
}

#[derive(Debug, Clone, Copy)]
enum Command {
    Left, Right, Forward(usize)
}

#[derive(Debug)]
enum Dir {
    U, R, D, L
}

#[derive(Debug)]
struct State {
    row: usize,
    col: usize,
    dir: Dir
}

impl State {
    fn new(row: usize, col: usize, dir: Dir) -> Self {
        Self { row, col, dir }
    }

    fn execute_command(&mut self, map: &Array2D<Tile>, command: Command) {
        match command {
            Right => self.rotate_right(),
            Left => self.rotate_left(),
            Forward(num_tiles) => {
                for _ in 0..num_tiles {
                    let next_position = self.get_next_position(map);
                    if let Open = *map.get(next_position.0, next_position.1).unwrap() {
                        self.row = next_position.0;
                        self.col = next_position.1;
                    } else {
                        break;
                    }
                }
            },
        }
    }

    fn get_next_position(&self, map: &Array2D<Tile>) -> (usize, usize) {
        let pos = (self.row, self.col);
        match self.dir {
            R => {
                let mut new_pos = (pos.0, pos.1 + 1);
                if let Blank = *map.get(new_pos.0, new_pos.1).unwrap() {
                    new_pos = (pos.0, pos.1);
                    while *map.get(new_pos.0, new_pos.1).unwrap() != Blank {
                        new_pos.1 -= 1;
                    }
                    new_pos.1 += 1;
                }
                new_pos
            },
            L => {
                let mut new_pos = (pos.0, pos.1 - 1);
                if let Blank = *map.get(new_pos.0, new_pos.1).unwrap() {
                    new_pos = (pos.0, pos.1);
                    while *map.get(new_pos.0, new_pos.1).unwrap() != Blank {
                        new_pos.1 += 1;
                    }
                    new_pos.1 -= 1;
                }
                new_pos
            },
            U => {
                let mut new_pos = (pos.0 - 1, pos.1);
                if let Blank = *map.get(new_pos.0, new_pos.1).unwrap() {
                    new_pos = (pos.0, pos.1);
                    while *map.get(new_pos.0, new_pos.1).unwrap() != Blank {
                        new_pos.0 += 1;
                    }
                    new_pos.0 -= 1;
                }
                new_pos
            },
            D => {
                let mut new_pos = (pos.0 + 1, pos.1);
                if let Blank = *map.get(new_pos.0, new_pos.1).unwrap() {
                    new_pos = (pos.0, pos.1);
                    while *map.get(new_pos.0, new_pos.1).unwrap() != Blank {
                        new_pos.0 -= 1;
                    }
                    new_pos.0 += 1;
                }
                new_pos
            },
        }
    }

    fn rotate_right(&mut self) {
        let new_dir = match self.dir {
            U => R,
            R => D,
            D => L,
            L => U,
        };
        self.dir = new_dir;
    }

    fn rotate_left(&mut self) {
        let new_dir = match self.dir {
            U => L,
            L => D,
            D => R,
            R => U,
        };
        self.dir = new_dir;
    }

    fn compute_password(&self) -> usize {
        let dir = match self.dir {
            R => 0,
            D => 1,
            L => 2,
            U => 3,
        };
        1000 * self.row + 4 * self.col + dir
    }
}

fn parse_input() -> (Array2D<Tile>, Vec<Command>) {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    let max_len = lines
        .map(|line| line.unwrap().len())
        .max().unwrap() + 2;

    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    let first_row = vec![Blank; max_len];
    let last_row = first_row.clone();
    let mut rows = Vec::from([first_row]);
    lines.map_while(|line| {
        let line = line.unwrap();
        if line.is_empty() {
            None
        } else {
            let mut row = Vec::from([Blank]);
            line.chars().for_each(|c| {
                let tile = match c {
                    ' ' => Blank,
                    '.' => Open,
                    '#' => Wall,
                    _ => panic!("Parse exception"),
                };
                row.push(tile);
            });
            let len = row.len();
            let to_fill = max_len - len;
            row.extend(vec![Blank; to_fill]);

            Some(row)
        }
    }).for_each(|row| {
       rows.push(row);
    });
    rows.push(last_row);
    let map = Array2D::from_rows(&rows).unwrap();

    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    let commands = lines.last().unwrap().unwrap();
    let commands = commands.replace("L", " L ");
    let commands = commands.replace("R", " R ");
    let split = commands.split_whitespace();
    let commands = split.map(|c| {
        match c {
            "R" => Right,
            "L" => Left,
            n => {
                let val = n.parse::<usize>().expect("Parse error");
                Forward(val)
            },
        }
    }).collect::<Vec<_>>();

    (map, commands)
}

fn main() {
    let (map, commands) = parse_input();

    // Part 1
    let mut state = State::new(1, 50, R);
    println!("{:?}", state);
    commands.iter().for_each(|&command| {
        println!("{:?}", command);
        state.execute_command(&map, command);
        println!("{:?}", state);
    });
    let password = state.compute_password();
    println!("The password is: {password}");
}
