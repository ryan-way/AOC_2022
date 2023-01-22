use std::io;
use std::cmp;

fn visibility_from_max(grid: &Vec<Vec<i32>>, maxes: &Vec<Vec<i32>>) -> Vec<Vec<bool>> {
    grid.iter().zip(maxes.iter())
        .map(|(x, y)|
            x.iter().zip(y.iter())
            .map(|(a, b)| a > b)
            .collect()
        ).collect()
}

fn top_visibility(grid: &Vec<Vec<i32>>) -> Vec<Vec<bool>> {
    let len = grid.first().unwrap().len();
    let maxes = grid.iter()
        .scan(vec![-1; len], |x, y| {
            let max: Vec<i32> = x.iter().zip(y.iter())
                .map(|(&a, &b)| cmp::max(a, b))
                .collect();
            let ret = x.clone();
            x.clone_from(&max);
            Some(ret)
        }).collect();

    visibility_from_max(grid, &maxes)
}

fn bottom_visibility(grid: &Vec<Vec<i32>>) -> Vec<Vec<bool>> {
    let len = grid.first().unwrap().len();
    let mut maxes: Vec<Vec<i32>> = grid.iter().rev()
        .scan(vec![-1; len], |x, y| {
            let max: Vec<i32> = x.iter().zip(y.iter())
                .map(|(&a, &b)| cmp::max(a, b))
                .collect();
            let ret = x.clone();
            x.clone_from(&max);
            Some(ret)
        }).collect();
    maxes.reverse();

    visibility_from_max(grid, &maxes)
}

fn right_visibility(grid: &Vec<Vec<i32>>) -> Vec<Vec<bool>> {
    let len = grid.first().unwrap().len();
    let maxes = grid.iter()
        .map(|row|
            row.iter().scan(-1, |state, val| {
                let ret = state.clone();
                state.clone_from(&cmp::max(*val, *state));
                Some(ret)
            }).collect()
        ).collect();

    visibility_from_max(grid, &maxes)
}

fn left_visibility(grid: &Vec<Vec<i32>>) -> Vec<Vec<bool>> {
    let len = grid.first().unwrap().len();
    let maxes = grid.iter()
        .map(|row| {
            let mut new_row: Vec<i32> = row.iter().rev().scan(-1, |state, val| {
                let ret = state.clone();
                state.clone_from(&cmp::max(*val, *state));
                Some(ret)
            }).collect();
            new_row.reverse();
            new_row
        }).collect();

    visibility_from_max(grid, &maxes)
}

fn merge_visibility(one: &Vec<Vec<bool>>, two: &Vec<Vec<bool>>) -> Vec<Vec<bool>> { one.iter()
        .zip(two.iter())
        .map(|(row1, row2)| row1.iter().zip(row2.iter())
            .map(|(val1, val2)| *val1 || *val2)
            .collect()
        )
        .collect()
}

fn main () {
    let lines = io::stdin().lines();

    let grid: Vec<Vec<i32>> = lines
        .filter_map(|x| x.ok())
        .map(|x| x.split("")
            .filter_map(|y| y.to_string().parse::<i32>().ok())
            .collect()
        )
        .collect();

    let visibility = merge_visibility(
        &merge_visibility(&right_visibility(&grid), &left_visibility(&grid)),
        &merge_visibility(&bottom_visibility(&grid), &top_visibility(&grid))
    );

    let count: i32 = visibility.iter()
        .flatten()
        .map(|x| if *x { 1 } else { 0 })
        .sum();

    println!("Count: {}", count);

}
