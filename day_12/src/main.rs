use std::fs::File;
use std::io::{self, BufRead};
use crate::Dir::{DOWN, LEFT, RIGHT, UP};

#[derive(Clone, Copy)]
enum Dir {
    UP, DOWN, LEFT, RIGHT
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn left(&self) -> Self {
        Pos::new(self.row,self.col-1)
    }

    fn right(&self) -> Self {
        Pos::new(self.row, self.col+1)
    }

    fn up(&self) -> Self {
        Pos::new(self.row-1, self.col)
    }

    fn down(&self) -> Self {
        Pos::new(self.row+1, self.col)
    }
}

struct Path {
    pos: Pos,
    len: usize,
}

impl Path {
    fn new(row: usize, col: usize) -> Self {
        Self {
            pos: Pos::new(row, col),
            len: 0,
        }
    }

    fn new_from(head: &Path, dir: Dir) -> Self {
        let pos = match dir {
            UP => head.pos.up(),
            DOWN => head.pos.down(),
            LEFT => head.pos.left(),
            RIGHT => head.pos.right()
        };
        let len = head.len + 1;

        Self {
            pos,
            len,
        }
    }

    fn do_one_step(&self, map: &Vec<Vec<u8>>, dist: &mut Vec<Vec<usize>>, num_rows: usize, num_cols: usize) -> Vec<Path> {
        fn check_add(path: &Path, dir: Dir, map: &Vec<Vec<u8>>, dist: &mut Vec<Vec<usize>>, res: &mut Vec<Path>) {
            let new_path = Path::new_from(path, dir);
            let new_pos = new_path.pos;
            if map[new_pos.row][new_pos.col] <= map[path.pos.row][path.pos.col] + 1 && dist[new_pos.row][new_pos.col] > new_path.len {
                dist[new_pos.row][new_pos.col] = new_path.len;
                res.push(Path::new_from(path, dir));
            }
        }

        let mut res = Vec::new();
        if self.pos.row < num_rows - 1 {
            check_add(self, DOWN, map, dist, &mut res);
        }
        if self.pos.row > 0 {
            check_add(self, UP, map, dist, &mut res);
        }
        if self.pos.col < num_cols - 1 {
            check_add(self, RIGHT, map, dist, &mut res);
        }
        if self.pos.col > 0 {
            check_add(self, LEFT, map, dist, &mut res);
        }
        res
    }

}

fn parse_input() -> (Vec<Vec<u8>>, Pos, Pos) {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    let lines = (0..).zip(lines);
    let mut start = None;
    let mut end = None;
    let map = lines.map(|(row, line)| {
        let line = line.unwrap();
        line.char_indices().map(|(col, c)| {
            let c = if c == 'S' {
                start = Some(Pos::new(row, col));
                'a'
            } else if c == 'E' {
                end = Some(Pos::new(row, col));
                'z'
            } else {
                c
            };
            c as u8 - 'a' as u8
        }).collect()
    }).collect();
    (map, start.unwrap(), end.unwrap())
}

fn compute_distance(map: &Vec<Vec<u8>>, start: Pos, end: Pos, max_iters: usize) -> usize {
    let num_rows = map.len();
    let num_cols = map[0].len();

    let mut dist: Vec<Vec<usize>> = (0..num_rows).map(|_i| {
        (0..num_cols).map(|_j| usize::MAX).collect()
    }).collect();
    dist[start.row][start.col] = 0;

    let mut current_paths = Vec::from([Path::new(start.row, start.col)]);
    let res;

    let mut it = 0;
    loop {
        it += 1;
        let new_paths: Vec<Path> = current_paths.into_iter()
            .flat_map(|path| path.do_one_step(&map, &mut dist, num_rows, num_cols).into_iter())
            .collect();
        let dist_to_end = dist[end.row][end.col];
        if dist_to_end < usize::MAX || it > max_iters {
            res = dist_to_end;
            break;
        } else {
            current_paths = new_paths;
        }
    };

    res
}

fn compute_distance_any_start(map: &Vec<Vec<u8>>, end: Pos, max_iters: usize) -> usize {
    let num_rows = map.len();
    let num_cols = map[0].len();

    let mut max_iters = max_iters;
    for row in 0..num_rows {
        for col in 0..num_cols {
            if map[row][col] == 0 {
                let start = Pos::new(row, col);
                let len = compute_distance(map, start, end, max_iters);
                if len < max_iters {
                    max_iters = len;
                }
            }
        }
    }

    max_iters
}

fn main() {
    let (map, start, end) = parse_input();

    // Part 1
    let min_path = compute_distance(&map, start, end, usize::MAX);
    println!("Min path from start: {}", min_path);

    // Part 2
    let min_path = compute_distance_any_start(&map, end, min_path);
    println!("Min path from any pos: {}", min_path);
}
