use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

static X_MAX: i32 = 150;
static Y_MAX: i32 = 20;

#[derive(Copy, Clone, Debug)]
enum Dir {
    Horizontal, Vertical
}

#[derive(Copy, Clone, Debug)]
struct Blizzard {
    dir: Dir,
    val: i32,
    pos: (i32, i32),
}

impl Blizzard {
    pub fn new(dir: Dir, val: i32, pos: (i32, i32)) -> Self {
        Self { dir, val, pos }
    }

    fn move_one_minute(&mut self) {
        match self.dir {
            Dir::Horizontal => {
                self.pos.0 += self.val;
                if self.pos.0 < 0 {
                    self.pos.0 = X_MAX - 1;
                } else if self.pos.0 >= X_MAX {
                    self.pos.0 = 0;
                }
            },
            Dir::Vertical => {
                self.pos.1 += self.val;
                if self.pos.1 < 0 {
                    self.pos.1 = Y_MAX - 1;
                } else if self.pos.1 >= Y_MAX {
                    self.pos.1 = 0;
                }
            }
        }
    }
}

#[derive(Debug)]
struct BlizzardMap {
    blizzards: Vec<Blizzard>,
    occupied_positions: HashSet<(i32, i32)>,
}

impl BlizzardMap {
    pub fn new() -> Self {
        Self {
            blizzards: Vec::new(),
            occupied_positions: HashSet::new(),
        }
    }

    fn add(&mut self, blizzard: Blizzard) {
        self.blizzards.push(blizzard);
        self.occupied_positions.insert(blizzard.pos);
    }

    fn move_one_minute(&mut self) {
        self.blizzards.iter_mut().for_each(|blizzard| blizzard.move_one_minute());
        self.occupied_positions = self.blizzards.iter()
            .map(|blizzard| blizzard.pos)
            .collect();
    }

    fn is_position_free(&self, pos: &(i32, i32)) -> bool {
        !self.occupied_positions.contains(pos)
    }

    fn possible_moves(&self, current_pos: (i32, i32)) -> HashSet<(i32, i32)> {
        let candidates = HashSet::from([
            (current_pos.0, current_pos.1),
            (current_pos.0 + 1, current_pos.1),
            (current_pos.0, current_pos.1 + 1),
            (current_pos.0 - 1, current_pos.1),
            (current_pos.0, current_pos.1 - 1),
        ]);

        let candidates = candidates.into_iter().filter(|pos| {
            (pos.0 == 0 && pos.1 == -1) || (pos.0 == X_MAX-1 && pos.1 == Y_MAX) || // Start and final positions
                (pos.0 >= 0 && pos.1 >= 0 && pos.0 <= X_MAX - 1 && pos.1 <= Y_MAX - 1) &&
            self.is_position_free(pos)
        }).collect::<HashSet<_>>();

        candidates
    }
}

fn shortest_path(blizzard_map: &mut BlizzardMap, initial_position: (i32, i32), final_position: (i32, i32), minute: i32) -> i32 {
    let mut current_positions = HashSet::from([initial_position]);
    let mut minute = minute;

    loop {
        blizzard_map.move_one_minute();
        minute += 1;
        let next_positions = current_positions.iter().map(|&pos| {
            blizzard_map.possible_moves(pos)
        }).flatten().collect::<HashSet<_>>();
        if next_positions.contains(&final_position) {
            break;
        } else {
            current_positions = next_positions;
        }
    }

    minute
}

fn parse_input() -> BlizzardMap {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();

    let mut row = 0;
    let mut blizzard_map = BlizzardMap::new();
    lines.for_each(|line| {
        let line = line.unwrap();
        // Skip first and last
        if !line.contains("###") {
            let line = line.replace("#", "");
            line.char_indices().filter_map(|(i, c)| {
                let col = i as i32;
                match c {
                    '^' => Some(Blizzard::new(Dir::Vertical, -1, (col, row))),
                    'v' => Some(Blizzard::new(Dir::Vertical, 1, (col, row))),
                    '<' => Some(Blizzard::new(Dir::Horizontal, -1, (col, row))),
                    '>' => Some(Blizzard::new(Dir::Horizontal, 1, (col, row))),
                    _ => None,
                }
            }).for_each(|blizzard| blizzard_map.add(blizzard));
            row += 1;
        }
    });

    blizzard_map
}

fn main() {
    let mut blizzard_map = parse_input();

    // Part 1
    let initial_position = (0, -1);
    let final_position = (X_MAX-1, Y_MAX);

    let minute = 0;
    let minute = shortest_path(&mut blizzard_map, initial_position, final_position, minute);
    println!("Minimum time: {}", minute);

    // Part 2
    let initial_position = (X_MAX-1, Y_MAX);
    let final_position = (0, -1);
    let minute = shortest_path(&mut blizzard_map, initial_position, final_position, minute);
    println!("Minimum time to go back: {}", minute);

    let initial_position = (0, -1);
    let final_position = (X_MAX-1, Y_MAX);
    let minute = shortest_path(&mut blizzard_map, initial_position, final_position, minute);
    println!("Minimum time to go again: {}", minute);
}
