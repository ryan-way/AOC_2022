use std::io;

fn main () {
    let lines = io::stdin().lines();

    let count = lines
        .map(|x| x.unwrap()
            .split(',')
            .map(|x|
                x.split('-')
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect())
            .collect()
        )
        .filter(|assignments: &Vec<Vec<u32>>|
                (assignments[0][0] <= assignments[1][0] && assignments[1][1] <= assignments[0][1])
                || (assignments[1][0] <= assignments[0][0] && assignments[0][1] <= assignments[1][1]))
        .count();


    println!("Count: {}", count);
}
