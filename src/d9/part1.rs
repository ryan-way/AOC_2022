use std::io;
use std::collections::HashSet;

fn parse_line(line: &String) -> (&str, u32){
    let mut it = line.split(' ');
    let s = it.next().unwrap();
    let c = it.next().unwrap().parse::<u32>().unwrap();
    (s, c)
}

fn update_tail(head: &(i32, i32), tail: &mut (i32, i32)) {
    if (tail.0 - head.0).abs() <= 1 && (tail.1 - head.1).abs() <= 1 {
        return;
    }

    if tail.1 < head.1 {
        tail.1 += 1;
    } else if tail.1 > head.1 {
        tail.1 -= 1;
    }

    if tail.0 < head.0 {
        tail.0 += 1;
    } else if tail.0 > head.0 {
        tail.0 -= 1;
    }
}

fn main () {
    let lines = io::stdin().lines();

    let mut head = (0, 0);
    let mut tail = (0, 0);

    let mut set: HashSet<(i32, i32)> = HashSet::new();

    for line in lines {
        let text = line.unwrap();
        let (dir, count) = parse_line(&text);

        for _ in 0..count {
            match dir {
                "U" => head.0 += 1,
                "D" => head.0 -= 1,
                "R" => head.1 += 1,
                "L" => head.1 -= 1,
                val => println!("Unsupported move: {}", val),
            };
            update_tail(&head, &mut tail);
            set.insert(tail.clone());
        }
    }

    println!("Unique tail positions: {}", set.len());
}
