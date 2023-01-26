use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

const N: usize = 0;
const S: usize = 1;
const W: usize = 2;
const E: usize = 3;
const NUM_DIRECTIONS: usize = 4;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn can_move(&self, index: &HashSet<Pos>) -> bool {
        for x in -1..2 {
            for y in -1..2 {
                if x != 0 || y != 0 {
                    let pos_to_check = Pos::new(self.x + x, self.y + y);
                    if index.contains(&pos_to_check) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn north_positions(&self) -> Vec<Pos> {
        Vec::from([
            Pos::new(self.x - 1, self.y - 1),
            Pos::new(self.x, self.y - 1),
            Pos::new(self.x + 1, self.y - 1)
        ])
    }

    fn north_position(&self) -> Pos {
        Pos::new(self.x, self.y - 1)
    }

    fn south_positions(&self) -> Vec<Pos> {
        Vec::from([
            Pos::new(self.x - 1, self.y + 1),
            Pos::new(self.x, self.y + 1),
            Pos::new(self.x + 1, self.y + 1)
        ])
    }

    fn south_position(&self) -> Pos {
        Pos::new(self.x, self.y + 1)
    }

    fn west_positions(&self) -> Vec<Pos> {
        Vec::from([
            Pos::new(self.x - 1, self.y -1),
            Pos::new(self.x - 1, self.y),
            Pos::new(self.x - 1, self.y + 1)
        ])
    }

    fn west_position(&self) -> Pos {
        Pos::new(self.x - 1, self.y)
    }

    fn east_positions(&self) -> Vec<Pos> {
        Vec::from([
            Pos::new(self.x + 1, self.y -1),
            Pos::new(self.x + 1, self.y),
            Pos::new(self.x + 1, self.y + 1)
        ])
    }

    fn east_position(&self) -> Pos {
        Pos::new(self.x + 1, self.y)
    }

    fn propose(&self, round: usize, index: &HashSet<Pos>) -> Option<Pos> {
        if !self.can_move(index) {
            None
        } else {
            for i in 0..NUM_DIRECTIONS {
                let dir = (i + round) % NUM_DIRECTIONS;
                match dir {
                    N => if self.north_positions().iter().all(|pos| !index.contains(pos)) {
                        return Some(self.north_position());
                    },
                    S => if self.south_positions().iter().all(|pos| !index.contains(pos)) {
                        return Some(self.south_position());
                    },
                    W => if self.west_positions().iter().all(|pos| !index.contains(pos)) {
                        return Some(self.west_position());
                    },
                    E => if self.east_positions().iter().all(|pos| !index.contains(pos)) {
                        return Some(self.east_position());
                    },
                    _ => panic!("Wrong direction")
                }
            }
            None // TODO check if this is correct
        }
    }
}

fn build_index(positions: &Vec<Pos>) -> HashSet<Pos> {
    positions.iter().cloned().collect::<HashSet<_>>()
}

fn make_proposals(positions: &Vec<Pos>, round: usize) -> Vec<(Pos, Pos)> {
    let index = build_index(positions);
    positions.iter().map(|&pos| {
        let proposal = pos.propose(round, &index);
        match proposal {
            Some(new_pos) => (pos, new_pos),
            None => (pos, pos)
        }
    }).collect()
}

fn compute_new_position(proposal: (Pos, Pos), proposals: &Vec<(Pos, Pos)>) -> Pos {
    let (old, new) = proposal;
    match proposals.iter().find(|(other_old, other_new)| {
        *other_new == new && *other_old != old
    }) {
        None => new,
        Some(_) => old,
    }
}

fn compute_new_positions(proposals: &Vec<(Pos, Pos)>) -> Vec<Pos> {
    proposals.iter().map(|&proposal| {
        compute_new_position(proposal, proposals)
    }).collect()
}

fn make_rounds(num_rounds: usize, positions: Vec<Pos>) -> Vec<Pos> {
    let mut result = positions;
    for round in 0..num_rounds {
        let proposals = make_proposals(&result, round);
        result = compute_new_positions(&proposals);
    }
    result
}

fn iterate_till_convergence(positions: Vec<Pos>) -> usize {
    let mut round = 0;
    let mut result = positions;
    loop {
        let proposals = make_proposals(&result, round);
        round += 1;
        if proposals.iter().find(|(old, new)| old != new).is_none() {
            break;
        }
        result = compute_new_positions(&proposals);
    }
    round
}

fn compute_free_positions(positions: &Vec<Pos>) -> i32 {
    let min_x = positions.iter().map(|p| p.x).min().unwrap();
    let max_x = positions.iter().map(|p| p.x).max().unwrap();
    let min_y = positions.iter().map(|p| p.y).min().unwrap();
    let max_y = positions.iter().map(|p| p.y).max().unwrap();
    let delta_x = max_x - min_x + 1;
    let delta_y = max_y - min_y + 1;
    delta_y * delta_x - positions.len() as i32
}

fn parse_input() -> Vec<Pos> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    let mut row = 0;

    lines.map(|line| {
        let line = line.unwrap();
        let res = line.char_indices().filter_map(|(i, c)| {
            match c {
                '#' => Some(Pos::new(i as i32, row)),
                _ => None,
            }
        }).collect::<Vec<_>>();
        row += 1;

        res
    }).flatten().collect()
}

fn main() {
    // Part 1
    let initial_positions = parse_input();
    let final_positions = make_rounds(10, initial_positions);
    let free_positions = compute_free_positions(&final_positions);
    println!("Number of free positions: {}", free_positions);

    // Part 2
    let initial_positions = parse_input();
    let num_iterations = iterate_till_convergence(initial_positions);
    println!("Converge after {} iterations", num_iterations);
}
