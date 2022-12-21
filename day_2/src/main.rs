use std::fs::File;
use std::io::{self, BufRead};
use crate::GameMove::{Rock, Paper, Scissors};
use crate::GameResult::{Win, Lose, Draw};

#[derive(Clone, Copy, Debug)]
enum GameMove {
    Rock,
    Paper,
    Scissors
}

impl GameMove {
    fn score(self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

impl From<&str> for GameMove {
    fn from(value: &str) -> Self {
        let value = value.chars().next().unwrap();
        if value == 'A' || value == 'X' { Rock }
        else if value == 'B' || value == 'Y' { Paper }
        else if value == 'C' || value == 'Z' { Scissors }
        else { panic!("Illegal input"); }
    }
}

#[derive(Debug)]
enum GameResult {
    Win,
    Lose,
    Draw
}

impl GameResult {
    fn score(self) -> u32 {
        match self {
            Win => 6,
            Lose => 0,
            Draw => 3,
        }
    }
}

#[derive(Debug)]
enum ExpectedResult {
    Win,
    Lose,
    Draw
}

impl From<&str> for ExpectedResult {
    fn from(value: &str) -> Self {
        let value = value.chars().next().unwrap();
        if value == 'X' { ExpectedResult::Lose }
        else if value == 'Y' { ExpectedResult::Draw }
        else if value == 'Z' { ExpectedResult::Win }
        else { panic!("Illegal input"); }
    }
}

fn compute_move(other: GameMove, expected: ExpectedResult) -> GameMove {
    match expected {
        ExpectedResult::Draw => { other },
        ExpectedResult::Win => {
            match other {
                Rock => Paper,
                Paper => Scissors,
                Scissors => Rock,
            }
        },
        ExpectedResult::Lose => {
            match other {
                Rock => Scissors,
                Paper => Rock,
                Scissors => Paper,
            }
        },
    }
}

impl From<(GameMove, GameMove)> for GameResult {
    fn from(value: (GameMove, GameMove)) -> Self {
        let p1 = value.0.score();
        let p2 = value.1.score();
        if p2 == p1 { Draw }
        else if p2 == (p1%3) + 1 { Win }
        else { Lose }
    }
}

fn main() {

    // Part 1
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    let tot: u32 = lines
        .map(|line| {
            let line = line.unwrap();
            let game_moves: Vec<&str> = line.split(' ').collect();
            let other_move: GameMove = game_moves[0].into();
            let my_move: GameMove = game_moves[1].into();
            let game_result: GameResult = (other_move, my_move).into();
            my_move.score() + game_result.score()
        }).sum();

    println!("Part 1: {tot}");

    // Part 2
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    let tot: u32 = lines
        .map(|line| {
            let line = line.unwrap();
            let game_moves: Vec<&str> = line.split(' ').collect();
            let other_move: GameMove = game_moves[0].into();
            let expected_result: ExpectedResult = game_moves[1].into();
            let my_move = compute_move(other_move, expected_result);
            let game_result: GameResult = (other_move, my_move).into();
            my_move.score() + game_result.score()
        }).sum();

    println!("Part 2: {tot}");
}
