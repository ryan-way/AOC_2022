use std::io;

#[derive(PartialEq)]
enum Op {
    Addx,
    Noop,
}

fn parse_input(input: &String) -> (Op, i32) {
    if input.starts_with("addx") {
        let count = input.split(' ').last().unwrap().parse::<i32>().unwrap();

        (Op::Addx, count)
    } else {
        (Op::Noop, 0)
    }
}

fn process_cycle(op: Op, param: i32, value: &mut i32, cycle: &mut i32, crt: &mut Vec<char>) {
    let cycles_to_process = if op == Op::Addx { 2 } else { 1 };

    for _ in 0..cycles_to_process {
        *cycle += 1;

        if ((*cycle % 40) <= *value+2) && ((*cycle % 40) >= *value) {
            crt.push('#');
        } else {
            crt.push('.');
        }
    }

    *value += param;
}

fn main () {
    let lines = io::stdin().lines();

    let mut value = 1;
    let mut cycle = 0;
    let mut crt: Vec<char> = vec![];

    for line in lines {
        let input = line.unwrap();

        let (op, param) = parse_input(&input);

        process_cycle(op, param, &mut value, &mut cycle, &mut crt);
    }

    for (idx, c) in crt.iter().enumerate() {
        if idx % 40 == 0 {
            println!();
        }
        print!("{}", c);
    }

}
