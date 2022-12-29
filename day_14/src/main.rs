use std::cmp::{max, min};
use std::fs::File;
use std::io::{self, BufRead};
use adjacent_pair_iterator::AdjacentPairIterator;
use array2d::Array2D;

fn read_file() -> (Vec<Vec<(usize, usize)>>, usize, usize) {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    let rocks: Vec<Vec<_>> = lines.map(|line| {
        let line = line.unwrap();
        let line = line.replace("->", "");
        line.split_whitespace()
            .map(|pair| pair.split_once(",").unwrap())
            .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
            .collect()
    }).collect();

    let xmax = *rocks.iter()
        .map(|v| {
            v.iter().map(|(x, _y)| x).max().unwrap()
        }).max().unwrap();

    let ymax = *rocks.iter()
        .map(|v| {
            v.iter().map(|(_x, y)| y).max().unwrap()
        }).max().unwrap();

    (rocks, xmax, ymax)
}

fn place_rocks(rocks: Vec<Vec<(usize, usize)>>, map: &mut Array2D<bool>) {
    rocks.iter().for_each(|line| {
        line.adjacent_pairs()
            .for_each(|((x1, y1), (x2, y2))| {
                if x1 == x2 {
                    let x = *x1;
                    let min = *min(y1, y2);
                    let max = *max(y1, y2);
                    for y in min..max + 1 {
                        map[(y, x)] = true;
                    }
                } else if y1 == y2 {
                    let y = *y1;
                    let min = *min(x1, x2);
                    let max = *max(x1, x2);
                    for x in min..max + 1 {
                        map[(y, x)] = true;
                    }
                } else {
                    panic!("Parse error: points not on the same line")
                }
            })
    });
}

fn parse_input() -> Array2D<bool> {
    let (rocks, xmax, ymax) = read_file();
    let mut map = Array2D::filled_with(false, ymax+1, xmax+1);
    place_rocks(rocks, &mut map);

    map
}

fn parse_input2() -> Array2D<bool> {
    let (rocks, xmax, ymax) = read_file();
    let ymax = ymax + 2;
    let xmax = xmax + ymax;
    let mut map = Array2D::filled_with(false, ymax+1, xmax);
    place_rocks(rocks, &mut map);
    for x in 0..xmax {
        map[(ymax, x)] = true
    }

    map
}

// Returns true if the sand falls forever
fn fall_from(map: &mut Array2D<bool>, pos: (usize, usize)) -> bool {
    let (y, x) = pos;
    if y >= map.num_rows()-1 {
        true
    } else if !map[(y+1, x)] {
        fall_from(map, (y+1, x))
    } else if !map[(y+1, x-1)] {
        fall_from(map, (y+1, x-1))
    } else if !map[(y+1, x+1)] {
        fall_from(map, (y+1, x+1))
    } else {
        map[(y, x)] = true;
        false
    }
}

fn fall_from2(map: &mut Array2D<bool>, pos: (usize, usize)) {
    let (y, x) = pos;
    if !map[(y+1, x)] {
        fall_from2(map, (y+1, x));
    } else if !map[(y+1, x-1)] {
        fall_from2(map, (y+1, x-1));
    } else if !map[(y+1, x+1)] {
        fall_from2(map, (y+1, x+1));
    } else {
        map[(y, x)] = true;
    }
}

fn main() {
    // Part 1
    let mut map = parse_input();
    for i in 1.. {
        let start = (0, 500);
        if fall_from(&mut map, start) {
            println!("Units that come to rest: {}", i-1);
            break;
        }
    }

    // Part 2
    let mut map = parse_input2();
    for i in 1.. {
        let start = (0, 500);
        fall_from2(&mut map, start);
        if map[(0, 500)] {
            println!("Units that come to rest: {}", i);
            break;
        }
    }
}
