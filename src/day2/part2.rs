use std::io;

struct Opponent;

impl Opponent {
    pub const ROCK: char = 'A';
    pub const PAPER: char = 'B';
    pub const SCISSORS: char = 'C';
}

struct Outcome;

impl Outcome {
    pub const LOSE: char = 'X';
    pub const DRAW: char = 'Y';
    pub const WIN: char = 'Z';
}

struct Score;

impl Score {
    pub const ROCK: i32 = 1;
    pub const PAPER: i32 = 2;
    pub const SCISSORS: i32 = 3;
    pub const WIN: i32 = 6;
    pub const LOSE: i32 = 0;
    pub const DRAW: i32 = 3;
}

fn main () {
    let lines = io::stdin().lines();
    let mut score = 0;

    for line in lines {
        let string = line.unwrap();

        score += match string.chars().nth(2).unwrap() {
            Outcome::LOSE => Score::LOSE,
            Outcome::DRAW => Score::DRAW,
            Outcome::WIN => Score::WIN,
            _ => 0,
        };

        score += match string.chars().nth(2).unwrap() {
            Outcome::LOSE => match string.chars().nth(0).unwrap() {
                Opponent::ROCK => Score::SCISSORS,
                Opponent::PAPER => Score::ROCK,
                Opponent::SCISSORS => Score::PAPER,
                _ => 0,
            },
            Outcome::DRAW => match string.chars().nth(0).unwrap() {
                Opponent::ROCK => Score::ROCK,
                Opponent::PAPER => Score::PAPER,
                Opponent::SCISSORS => Score::SCISSORS,
                _ => 0,
            },
            Outcome::WIN => match string.chars().nth(0).unwrap() {
                Opponent::ROCK => Score::PAPER,
                Opponent::PAPER => Score::SCISSORS,
                Opponent::SCISSORS => Score::ROCK,
                _ => 0,
            },
            _ => 0,
        };

    }

    println!("Score: {}", score);
}
