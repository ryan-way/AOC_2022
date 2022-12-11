use std::io;
use std::collections::HashSet;

mod score;

fn main () {
    let lines = io::stdin().lines();
    let mut priority = 0;

    let mut group = vec![];

    for line in lines {
        group.push(line.unwrap());

        if group.len() < 3 {
            continue;
        }

        let group_sets: Vec<HashSet<char>> = group.iter_mut()
            .map(|x| HashSet::from_iter(x.chars()))
            .collect();

        let badge = group.first()
            .unwrap()
            .chars()
            .filter(
                |x| group_sets.iter()
                    .all(|y| y.contains(x))
            )
            .next()
            .unwrap();

        priority += score::char_score(badge);

        group.clear();
    }

    println!("Priority: {}", priority);
}
