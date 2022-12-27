use std::{io, collections::HashSet};

fn main () {
    let message_size = 14;
    let mut lines = io::stdin().lines();

    for line in lines {
        let datastream = line.unwrap();

        let stream: Vec<char> = datastream
            .chars()
            .collect();

        let marker = stream
            .windows(message_size)
            .enumerate()
            .filter(|tup: &(usize, &[char])| HashSet::<&char>::from_iter(tup.1.iter()).len() == tup.1.len())
            .nth(0).unwrap();

        println!("Marker: {}", marker.0 + message_size);
    }
}
