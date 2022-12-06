use std::io;

fn main() {
    let mut max: i32 = 0;
    let mut this: i32 = 0;

    let lines = io::stdin().lines();

    for line in lines {
        let string = line.unwrap();
        if string.is_empty() {
            max = if max < this { this } else { max };
            this = 0;
            continue;
        }

        let num = string.parse::<i32>().unwrap();
        this += num;
    }

    println!("max: {}", max);
}
