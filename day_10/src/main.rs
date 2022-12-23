use std::fs::File;
use std::io::{self, BufRead};
use crate::Cmd::{Addx, Noop};
use crate::Status::{Processing, WaitingCmd};

#[derive(Debug)]
enum Cmd {
    Noop,
    Addx(i32),
}

#[derive(Debug, Copy, Clone)]
struct Change {
    delta: i32,
    end_cycle: u32,
}

enum Status {
    WaitingCmd,
    Processing,
}

struct Device {
    val: i32,
    cycle: u32,
    pending_change: Option<Change>,
}

impl Device {
    fn new() -> Self {
        Self {
            val: 1,
            cycle: 0,
            pending_change: None,
        }
    }

    // returns true if the simulation is finished
    fn clock_tick(&mut self) -> Status {
        self.cycle += 1;
        match self.pending_change {
            None => { WaitingCmd },
            Some(Change{delta, end_cycle}) => {
                if self.cycle == end_cycle {
                    self.val += delta;
                    self.pending_change = None;
                    WaitingCmd
                } else {
                    Processing
                }
            }
        }
    }

    fn submit_command(&mut self, cmd: &Cmd) {
        self.pending_change = match *cmd {
            Noop => Some(Change {
                delta: 0,
                end_cycle: self.cycle + 1,
            }),
            Addx(delta) => Some(Change {
                delta,
                end_cycle: self.cycle + 2,
            })
        }
    }
}

fn parse_input() -> Vec<Cmd> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    lines.map(|line| {
        let line = line.unwrap();
        match &line[0..4] {
            "noop" => Noop,
            "addx" => {
                let val = &line[5..].parse::<i32>().unwrap();
                Addx(*val)
            }
            _ => panic!("Unknown command")
        }
    }).collect()
}

fn main() {
    let input = parse_input();

    let mut input_it = input.iter();
    let mut device = Device::new();

    let values: Vec<_> = (1..241).map(|cycle| {
        match device.clock_tick() {
            WaitingCmd => {
                if let Some(cmd) = input_it.next() {
                    device.submit_command(cmd);
                }
            },
            Processing => { }
        }
        (cycle, device.val)
    }).collect();

    // Part 1
    let signal_strength = values.iter()
        .skip(19)
        .step_by(40)
        .map(|(cycle, val)| cycle*val)
        .sum::<i32>();
    println!("Signal strength: {}", signal_strength);

    // Part 2
    values.iter().for_each(|(cycle, val)| {
        let pos = (*cycle-1)%40;
        if val.abs_diff(pos) <= 1 {
            print!("#");
        } else {
            print!(".")
        }
        if pos==39 {
            println!();
        }
    });
}