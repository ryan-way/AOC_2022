use std::io;
use std::collections::HashSet;

mod score;

fn main () {
    let lines = io::stdin().lines();
    let mut priority = 0;

    for line in lines {
        let rucksack = match line {
            Ok(result) => result,
            Err(err) => panic!("Error getting line: {}", err)
        };

        let compartment1 = &rucksack[0..rucksack.len()/2];
        let compartment2 = &rucksack[rucksack.len()/2..];

        let set1: HashSet<char> = HashSet::from_iter(compartment1.chars());
        let set2: HashSet<char> = HashSet::from_iter(compartment2.chars());

        let common: &char = set1.intersection(&set2)
            .next().unwrap();

        priority += score::char_score(*common);
    }

    println!("Priority: {}", priority);
}
