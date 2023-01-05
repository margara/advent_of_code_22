use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

const COLS: u8 = 7;
const MAX_SHAPE_ROWS: usize = 4;
const ALLOC_SIZE: usize = 800;

#[derive(Debug)]
enum Dir { L, R }

// Columns are indexed from 0 to 6
// Columns are represented using the bits of a byte, where 1 means full
// Row 0 is the floor
struct Board {
    board: [u8; ALLOC_SIZE],
    height: usize,
    base: usize,
}

struct ShapeFactory {
    shapes: Vec<Shape>,
}

impl ShapeFactory {
    pub fn new() -> Self {
        let shapes = (0..5).map(|i| Shape::new(i)).collect();
        Self { shapes }
    }

    pub fn get_shape(&mut self, shape: usize) -> &mut Shape {
        self.shapes.get_mut(shape).expect("Unknown shape")
    }
}

// Rows are identified by a bit mask
struct Shape {
    rows_masks: [u8; MAX_SHAPE_ROWS],
    num_rows: usize,
    width: u8,
    coord: (usize, u8),
}

impl Shape {
    fn new(shape: usize) -> Self {
        let mut rows_masks = [0; MAX_SHAPE_ROWS];
        let num_rows;
        let width;
        match shape {
            0 => {
                rows_masks[0] = 1u8 + (1u8 << 1) + (1u8 << 2) + (1u8 << 3);
                num_rows = 1;
                width = 4;
            },
            1 => {
                rows_masks[0] = 1u8 << 1;
                rows_masks[1] = 1u8 + (1u8 << 1) + (1u8 << 2);
                rows_masks[2] = 1u8 << 1;
                num_rows = 3;
                width = 3;
            },
            2 => {
                rows_masks[0] = 1u8 + (1u8 << 1) + (1u8 << 2);
                rows_masks[1] = 1u8 << 2;
                rows_masks[2] = 1u8 << 2;
                num_rows = 3;
                width = 3;
            },
            3 => {
                rows_masks[0] = 1u8;
                rows_masks[1] = 1u8;
                rows_masks[2] = 1u8;
                rows_masks[3] = 1u8;
                num_rows = 4;
                width = 1;
            },
            4 => {
                rows_masks[0] = 1u8 + (1u8 << 1);
                rows_masks[1] = 1u8 + (1u8 << 1);
                num_rows = 2;
                width = 2;
            },
            _ => panic!("Unknown shape")
        };
        let coord = (0, 0);

        Shape { rows_masks, num_rows, width, coord }
    }

    fn move_to_initial_position(&mut self, board: &Board) {
        self.coord.0 = board.height + 1;
        self.coord.1 = 2;
    }

    fn can_move_left(&self, board: &Board) -> bool {
        if self.coord.1 < 1 {
            return false;
        }
        if self.coord.0 > board.height {
            return true;
        }
        for i in 0..self.num_rows {
            let row = self.coord.0 + i;
            let mask = self.rows_masks[i] << (self.coord.1 - 1);
            if !board.is_empty(row, mask) {
                return false;
            }
        }
        true
    }

    fn can_move_right(&self, board: &Board) -> bool {
        if self.coord.1 + self.width >= COLS {
            return false;
        }
        if self.coord.0 > board.height {
            return true;
        }
        for i in 0..self.num_rows {
            let row = self.coord.0 + i;
            let mask = self.rows_masks[i] << (self.coord.1 + 1);
            if !board.is_empty(row, mask) {
                return false;
            }
        }
        true
    }

    fn can_move_down(&self, board: &Board) -> bool {
        if self.coord.0 > board.height + 1 {
            return true;
        }
        for i in 0..self.num_rows {
            let row = self.coord.0 + i - 1;
            let mask = self.rows_masks[i] << self.coord.1;
            if !board.is_empty(row, mask) {
                return false;
            }
        }
        true
    }

    fn move_left(&mut self) {
        self.coord.1 -= 1;
    }

    fn move_right(&mut self) {
        self.coord.1 += 1;
    }

    fn move_down(&mut self) {
        self.coord.0 -= 1;
    }

    fn stop(&self, board: &mut Board) {
        let mut full_row = None;
        for i in 0..self.num_rows {
            let row = self.coord.0 + i;
            let mask = self.rows_masks[i] << self.coord.1;
            if board.set(row, mask) {
                full_row = Some(row);
            }
        }
        if let Some(full_row) = full_row {
            board.resize(full_row);
        }
    }
}

impl Board {
    fn new() -> Self {
        let mut board = [0u8; ALLOC_SIZE];
        board[0] = 127;

        Self {
            board,
            height: 0,
            base: 0,
        }
    }

    fn is_empty(&self, row: usize, mask: u8) -> bool {
        self.board[row] & mask == 0
    }

    // Returns true if the row is filled
    fn set(&mut self, row: usize, mask: u8) -> bool {
        self.board[row] |= mask;
        if self.height < row {
            self.height = row;
        }
        self.board[row] == 127
    }

    fn height(&self) -> usize {
        self.height
    }

    fn base(&self) -> usize {
        self.base
    }

    // Resize such that row becomes the new base
    fn resize(&mut self, row: usize) {
        let old_height = self.height;
        let new_height = old_height - row;
        self.board.as_mut_slice().copy_within(row..old_height+1, 0);
        self.board.as_mut_slice()[new_height+1..old_height+1].fill(0);

        self.base += row;
        self.height = new_height;
    }

}

fn parse_input() -> Vec<Dir> {
    let mut f = File::open("input/input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s.trim().chars().map(|c| {
        match c {
            '>' => Dir::R,
            '<' => Dir::L,
            _ => panic!("Parse error"),
        }
    }).collect()
}

fn run_simulation(input: &Vec<Dir>, num_rocks: usize) -> usize {
    let mut board = Board::new();

    let mut input_iter = input.iter().cycle();
    let mut shape_factory = ShapeFactory::new();
    for i in 0..num_rocks {
        if i % 1000000 == 0 {
            println!("Rock {}M", i/1000000);
        }
        let shape = shape_factory.get_shape(i % 5);
        shape.move_to_initial_position(&board);
        // Start from max height + 1 to avoid three moves down
        for _x in 0..3 {
            let dir = input_iter.next().unwrap();
            match dir {
                Dir::L => if shape.can_move_left(&board) { shape.move_left(); },
                Dir::R => if shape.can_move_right(&board) { shape.move_right(); },
            }
        }
        loop {
            let dir = input_iter.next().unwrap();
            match dir {
                Dir::L => if shape.can_move_left(&board) { shape.move_left(); },
                Dir::R => if shape.can_move_right(&board) { shape.move_right(); },
            }
            if shape.can_move_down(&board) {
                shape.move_down();
            } else {
                shape.stop(&mut board);
                break;
            }
        }
    }
    let height = board.height() + board.base();
    height
}

#[derive(Hash, Eq, PartialEq)]
struct State {
    shape: usize,
    height: usize,
    input: usize,
    board: Vec<u8>,
}

impl State {
    pub fn new(shape: usize, input: usize, b: &Board) -> Self {
        let height = b.height();
        let board = (0..height+1).map(|i| {
            b.board[i]
        }).collect();

        Self { shape, height, input, board }
    }
}

// Result = (first id, first_height, repetition step, height delta)
// Where first indicates the first state for which we
fn find_repetitions(input: &Vec<Dir>, num_rocks: usize) -> (usize, usize, usize, usize) {
    let mut board = Board::new();

    let mut input_iter = input.iter().cycle();
    let mut shape_factory = ShapeFactory::new();
    let mut state_map = HashMap::new();

    let mut first = None;
    let mut res = (0, 0, 0, 0);
    for i in 0..num_rocks {
        if i % 1000000 == 0 {
            println!("Rock {}M", i/1000000);
        }
        let shape = shape_factory.get_shape(i % 5);
        shape.move_to_initial_position(&board);
        // Start from max height + 1 to avoid three moves down
        for _x in 0..3 {
            let dir = input_iter.next().unwrap();
            match dir {
                Dir::L => if shape.can_move_left(&board) { shape.move_left(); },
                Dir::R => if shape.can_move_right(&board) { shape.move_right(); },
            }
        }
        loop {
            let dir = input_iter.next().unwrap();
            match dir {
                Dir::L => if shape.can_move_left(&board) { shape.move_left(); },
                Dir::R => if shape.can_move_right(&board) { shape.move_right(); },
            }
            if shape.can_move_down(&board) {
                shape.move_down();
            } else {
                shape.stop(&mut board);
                break;
            }
        }
        let state = State::new(i % 5, i % input.len(), &board);
        let height = board.height() + board.base();
        if let Some((old_id, old_height)) = state_map.get(&state) {
            if let Some((first_id, first_height)) = first {
                res = (first_id, first_height, *old_id-first_id, *old_height-first_height);
                break;
            } else {
                first = Some((*old_id, *old_height));
            }
        } else {
            state_map.insert(state, (i, height));
        }
    }

    res
}

fn main() {
    let input = parse_input();

    // Part 1
    let height = run_simulation(&input, 2022);
    println!("Height after 2022 rocks: {}", height);

    // Part 2 (searching for patterns)
    let (first_it, first_height, it_step, height_step) = find_repetitions(&input, 1000000000000);
    println!("Found repetition. From state {} with height {}, height increases by {} every {}", first_it, first_height, height_step, it_step);

    let it_step = it_step * 5 * input.len();
    let height_step = height_step * 5 * input.len();
    println!("So, starting from {} the same state appears every {} iterations, with a height increment of {}", first_it, it_step, height_step);

    let mut it = 0;
    let mut height = 0;
    while it + it_step < 1000000000000 {
        it += it_step;
        height += height_step;
    }
    println!("The last occurrence before 1000000000000 is at iteration {} with height {}", it, height);

    let equivalent_to = 1000000000000 - it;
    println!("So state 1000000000000 is equivalent to state {} with an addition of {}", equivalent_to, height);

    let height = run_simulation(&input, equivalent_to) + height;
    println!("Height after 1000000000000 rocks: {}", height);

    // Part 2 (brute force)
    let height = run_simulation(&input, 1000000000000);
    println!("Height after 1000000000000 rocks: {}", height);
}
