use std::io;

fn main() {
    let mut first: i32 = 0;
    let mut second: i32 = 0;
    let mut third: i32 = 0;
    let mut this: i32 = 0;

    let lines = io::stdin().lines();

    for line in lines {
        let string = line.unwrap();
        if string.is_empty() {
            if this > first {
                third = second;
                second = first;
                first = this;
            } else if this > second {
                third = second;
                second = this;
            } else if this > third {
                third = this;
            }
            this = 0;
            continue;
        }

        let num = string.parse::<i32>().unwrap();
        this += num;

    }

    if this > first {
        third = second;
        second = first;
        first = this;
    } else if this > second {
        third = second;
        second = this;
    } else if this > third {
        third = this;
    }

    println!("max: {}", first + second + third);
}
