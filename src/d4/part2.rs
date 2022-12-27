use std::io;

fn contains_overlaps(assignments: &Vec<Vec<u32>>) -> bool {
    let first = assignments.first().unwrap();
    let last = assignments.last().unwrap();
    return last.iter().any(|&x| first[0] <= x && x <= first[1]) ||
        first.iter().any(|&x| last[0] <= x && x <= last[1]);
}

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
        .filter(|assignments: &Vec<Vec<u32>>| contains_overlaps(assignments))
        .count();


    println!("Count: {}", count);
}
