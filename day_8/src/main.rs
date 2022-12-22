use std::fmt::{Display, Formatter};
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};

#[derive(Copy, Clone)]
struct Tree {
    height: u8,
    visible: bool,
}

impl Tree {
    fn new(height: u8) -> Self {
        Self {
            height,
            visible: false
        }
    }
}

fn print_field(field: &Vec<Vec<Tree>>) {
    field.iter().for_each(|line| {
        line.iter().for_each(|t| print!("{}", t.height));
        println!();
    })
}

fn parse_line(line: &String) -> Vec<Tree> {
    line.chars()
        .map(|c| c.to_string().parse::<u8>().unwrap())
        .map(|h| Tree::new(h))
        .collect()
}

fn parse_input() -> Vec<Vec<Tree>> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    lines.map(|line| {
        let line = line.unwrap();
        parse_line(&line)
    }).collect()
}

fn compute_visibility(field: &mut Vec<Vec<Tree>>) {
    let height = field.len();
    let width = field[0].len();

    // Returns true if max becomes 9
    fn update_visibility(field: &mut Vec<Vec<Tree>>, i: usize, j: usize, max: &mut i8) -> bool {
        if field[i][j].height as i8 > *max {
            *max = field[i][j].height as i8;
            field[i][j].visible = true;
            *max == 9
        } else {
            false
        }
    }

    // From left
    for i in 0..height {
        let mut max: i8 = -1;
        for j in 0..width {
            if update_visibility(field, i, j, &mut max) { break; }
        }
    }

    // From right
    for i in 0..height {
        let mut max: i8 = -1;
        for j in (0..width).rev() {
            if update_visibility(field, i, j, &mut max) { break; }
        }
    }

    // From top
    for j in 0..width {
        let mut max: i8 = -1;
        for i in 0..height {
            if update_visibility(field, i, j, &mut max) { break; }
        }
    }

    // From bottom
    for j in 0..width {
        let mut max: i8 = -1;
        for i in (0..height).rev() {
            if update_visibility(field, i, j, &mut max) { break; }
        }
    }
}

fn compute_num_visible(field: &Vec<Vec<Tree>>) -> usize {
    field.iter().map(|line| {
        line.iter().filter(|t| t.visible).count()
    }).sum()
}

fn compute_scenic_score(field: &Vec<Vec<Tree>>) -> usize {
    fn compute_scenic_score_for(field: &Vec<Vec<Tree>>, i: usize, j: usize, height: usize, width: usize) -> usize {
        let mut top = 0;
        for k in (0..i).rev() {
            top += 1;
            if field[k][j].height >= field[i][j].height {
                break;
            }
        }

        let mut bottom = 0;
        for k in i+1..height {
            bottom += 1;
            if field[k][j].height >= field[i][j].height {
                break;
            }
        }

        let mut left = 0;
        for k in (0..j).rev() {
            left += 1;
            if field[i][k].height >= field[i][j].height {
                break;
            }
        }

        let mut right = 0;
        for k in j+1..width {
            right += 1;
            if field[i][k].height >= field[i][j].height {
                break;
            }
        }

        top * bottom * left * right
    }

    let height = field.len();
    let width = field[0].len();
    let mut max = 0;
    for i in 0..height {
        for j in 0..width {
            let scenic_score = compute_scenic_score_for(field, i, j, height, width);
            if scenic_score > max {
                max = scenic_score
            }
        }
    }
    max
}

fn main() {
    let mut field = parse_input();
    print_field(&field);

    // Part 1
    compute_visibility(&mut field);
    let num_visible = compute_num_visible(&field);
    println!("Part 1: {}", num_visible);

    // Part 2
    let max_scenic_score = compute_scenic_score(&mut field);
    println!("Part 2: {}", max_scenic_score);
}
