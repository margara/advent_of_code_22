use std::fs::File;
use std::io::Read;

const COLS: u8 = 7;

const ALLOC_SIZE: usize = 800;

#[derive(Debug)]
enum Dir { L, R }

// Columns are indexed from 1 to 7
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

struct Shape {
    // row, column
    pixels: Vec<(usize, u8)>,
    left_pixels: Vec<(usize, u8)>,
    right_pixels: Vec<(usize, u8)>,
    bottom_pixels: Vec<(usize, u8)>,
    coord: (usize, u8),
    width: u8,
}

impl Shape {
    fn new(shape: usize) -> Self {
        let pixels = match shape {
            0 => Vec::from([(0, 0), (0, 1), (0, 2), (0, 3)]),
            1 => Vec::from([(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]),
            2 => Vec::from([(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)]),
            3 => Vec::from([(0, 0), (1, 0), (2, 0), (3, 0)]),
            4 => Vec::from([(0, 0), (0, 1), (1, 0), (1, 1)]),
            _ => panic!("Unknown shape")
        };
        let left_pixels = match shape {
            0 => Vec::from([(0, 0)]),
            1 => Vec::from([(0, 1), (1, 0), (2, 1)]),
            2 => Vec::from([(0, 0), (1, 2), (2, 2)]),
            3 => Vec::from([(0, 0), (1, 0), (2, 0), (3, 0)]),
            4 => Vec::from([(0, 0), (1, 0)]),
            _ => panic!("Unknown shape")
        };
        let right_pixels = match shape {
            0 => Vec::from([(0, 3)]),
            1 => Vec::from([(0, 1), (1, 2), (2, 1)]),
            2 => Vec::from([(0, 2), (1, 2), (2, 2)]),
            3 => Vec::from([(0, 0), (1, 0), (2, 0), (3, 0)]),
            4 => Vec::from([(0, 1), (1, 1)]),
            _ => panic!("Unknown shape")
        };
        let bottom_pixels = match shape {
            0 => Vec::from([(0, 0), (0, 1), (0, 2), (0, 3)]),
            1 => Vec::from([(0, 1), (1, 0), (1, 2)]),
            2 => Vec::from([(0, 0), (0, 1), (0, 2)]),
            3 => Vec::from([(0, 0)]),
            4 => Vec::from([(0, 0), (0, 1)]),
            _ => panic!("Unknown shape")
        };
        let width = match shape {
            0 => 4,
            1 => 3,
            2 => 3,
            3 => 1,
            4 => 2,
            _ => panic!("Unknown shape")
        };
        let coord = (0, 0);

        Shape { pixels, left_pixels, right_pixels, bottom_pixels, coord, width }
    }

    fn move_to_initial_position(&mut self, board: &Board) {
        self.coord.0 = board.height + 4;
        self.coord.1 = 3;
    }

    fn can_move_left(&self, board: &Board) -> bool {
        self.coord.1 > 1 &&
            (self.coord.0 > board.height ||
                self.left_pixels.iter().all(|(r, c)| board.is_empty(self.coord.0 + *r, self.coord.1 + *c-1)))
    }

    fn can_move_right(&self, board: &Board) -> bool {
        self.coord.1 + self.width <= COLS &&
            (self.coord.0 > board.height ||
                self.right_pixels.iter().all(|(r, c)| board.is_empty(self.coord.0 + *r, self.coord.1 + *c+1)))
    }

    fn can_move_down(&self, board: &Board) -> bool {
        self.coord.0 > board.height + 1 ||
            self.bottom_pixels.iter().all(|(r, c)| board.is_empty(self.coord.0 + *r-1, self.coord.1 + *c))
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
        let mut row = None;
        self.pixels.iter().for_each(|(r, c)| {
            if board.set(self.coord.0 + *r, self.coord.1 + *c) {
                row = Some(self.coord.0 + *r);
            }
        });
        if let Some(row) = row {
            board.resize(row);
        }
    }
}

impl Board {
    fn new() -> Self {
        let mut board = [0u8; ALLOC_SIZE];
        board[0] = 255;

        Self {
            board,
            height: 0,
            base: 0,
        }
    }

    fn is_empty(&self, row: usize, col: u8) -> bool {
        self.board[row] & 1u8 << col == 0
    }

    // Returns true if the row is filled
    fn set(&mut self, row: usize, col: u8) -> bool {
        self.board[row] |= 1u8 << col;
        if self.height < row {
            self.height = row;
        }
        self.board[row] == 254
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
    (0..num_rocks).for_each(|i| {
        if i % 1000000 == 0 {
            println!("Rock {}M", i/1000000);
        }
        let shape = shape_factory.get_shape(i % 5);
        shape.move_to_initial_position(&board);
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
    });
    let height = board.height() + board.base();
    height
}

fn main() {
    let input = parse_input();

    // Part 1
    let height = run_simulation(&input, 2022);
    println!("Height after 2022 rocks: {}", height);

    // Part 2
    let height = run_simulation(&input, 1000000000000);
    println!("Height after 1000000000000 rocks: {}", height);
}
