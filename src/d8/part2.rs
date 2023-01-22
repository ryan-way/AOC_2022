use std::io;
use std::cmp;

fn score(grid: &Vec<Vec<i32>>, row_idx: usize, col_idx: usize, row_max: usize, col_max: usize) -> i32 {
    let mut score = 1;

    let mut count = 0;
    for row in (0..row_idx).rev() {
        if grid[row][col_idx] >= grid[row_idx][col_idx] {
            count +=1;
            break;
        } 
        count += 1;
    }

    score *=  if count == 0 { 1 } else { count };
    count = 0;

    for row in row_idx+1..row_max {
        if grid[row][col_idx] >= grid[row_idx][col_idx] {
            count +=1;
            break;
        } 
        count += 1;
    }

    score *=  if count == 0 { 1 } else { count };
    count = 0;

    for col in (0..col_idx).rev() {
        if grid[row_idx][col] >= grid[row_idx][col_idx] {
            count +=1;
            break;
        } 
        count += 1;
    }

    score *=  if count == 0 { 1 } else { count };
    count = 0;

    for col in col_idx+1..col_max {
        if grid[row_idx][col] >= grid[row_idx][col_idx] {
            count +=1;
            break;
        } 
        count += 1;
    }

    score *=  if count == 0 { 1 } else { count };
    score
}


fn slow() {
    let lines = io::stdin().lines();

    let grid: Vec<Vec<i32>> = lines
        .filter_map(|x| x.ok())
        .map(|x| x.split("")
            .filter_map(|y| y.to_string().parse::<i32>().ok())
            .collect()
        )
        .collect();
    let row_max = grid.len();
    let col_max = grid[0].len();

    let mut totals: Vec<Vec<i32>> = grid.iter()
        .map(|row| row.iter()
            .map(|_| 1)
            .collect()
        ).collect();


    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, _) in row.iter().enumerate() {
            totals[row_idx][col_idx] = score(&grid, row_idx, col_idx, row_max, col_max);
        }
    }

    let best = totals.iter().flatten().max().unwrap();

    println!("Best {}", best);
}

fn main () {
    slow();
}
