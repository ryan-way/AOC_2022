use std::io;

fn main () {
    let lines = io::stdin().lines();
    let mut score = 0;

    for line in lines {
        let string = line.unwrap();

        score += match string.chars().nth(2).unwrap() {
            'Y' => 2,
            'X' => 1,
            'Z' => 3,
            _ => 0,
        };

        score += match string.chars().nth(2).unwrap() {
            'Y' => match string.chars().nth(0).unwrap() {
                'A' => 6,
                'B' => 3,
                'C' => 0,
                _ => 0,
            },
            'X' => match string.chars().nth(0).unwrap() {
                'A' => 3,
                'B' => 0,
                'C' => 6,
                _ => 0,
            },
            'Z' => match string.chars().nth(0).unwrap() {
                'A' => 0,
                'B' => 6,
                'C' => 3,
                _ => 0,
            },
            _ => 0,
        };

    }

    println!("Score: {}", score);
}
