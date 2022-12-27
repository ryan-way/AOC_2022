use std::io;

struct Container {
    label: char,
}

impl Container {
    pub fn can_parse(line: &str) -> bool {
        return line.chars()
            .filter(|c: &char| c.is_alphabetic())
            .count() > 0 &&
            line.chars()
            .filter(|c: &char| c == &'[')
            .count() > 0;
    }

    pub fn parse(line: &str) -> Vec<(u32, Container)> {
        return line.char_indices()
            .filter(|tup: &(usize, char)| tup.1.is_alphabetic())
            .map(|tup: (usize, char)| (tup.0 as u32, Container { label: tup.1 }))
            .collect();
    }
}

struct Stack {
    label: u32,
    containers: Vec<Container>, 
}

struct Ship {
    stacks: Vec<Stack>,
}

impl Ship {
    pub fn get_container_index(pos: u32) -> u32 {
        return (pos -1 )/4;
    }

    pub fn print(&self) {

        println!("Ship status: ");
        for stack in &self.stacks {
            print!("Stack {}:", stack.label);
            for container in &stack.containers {
                print!("{}", container.label);
            }
            println!();
        }
    }
}

#[derive(Debug)]
struct Instruction {
    count: i32,
    src: i32,
    dest: i32,
}

impl Instruction {
    pub fn can_parse(line: &str) -> bool {
        return line.starts_with("move");
    }

    pub fn parse(line: &str) -> Instruction {
        let tokens: Vec<&str> = line.split(' ').collect();

        if tokens.len() != 6 {
            panic!("Expected 6 tokens, but found {} in {}", tokens.len(), line);
        }

        if tokens[0] != "move" {
            panic!("Expected 'move', found {}", tokens[0]);
        };

        if tokens[2] != "from" {
            panic!("Expected 'from', found {}", tokens[0]);
        }

        if tokens[4] != "to" {
            panic!("Expected 'to', found {}", tokens[0]);
        }

        Instruction {
            count: tokens[1].parse::<i32>().unwrap(),
            src: tokens[3].parse::<i32>().unwrap(),
            dest: tokens[5].parse::<i32>().unwrap()
        }
    }

    pub fn run(&self, ship: &mut Ship) {
        for _ in 0..self.count {
            let src_stack = &mut ship.stacks[(self.src-1) as usize];
            let container = src_stack.containers.pop().unwrap();

            let dest_stack = &mut ship.stacks[(self.dest-1) as usize];
            dest_stack.containers.push(container);
        }
    }
}

struct InstructionRunner {
    instructions: Vec<Instruction>,
    ship: Ship,
}

impl InstructionRunner {
    pub fn run(&mut self) {
        println!();
        for instruction in &self.instructions {
            instruction.run(&mut self.ship);
        }
    }

    pub fn print_result(&self) {
        let results: Vec<char> = self.ship.stacks.iter()
            .map(|stack| stack
                 .containers
                 .last()
                 .unwrap()
                 .label)
            .collect();

        print!("Results: ");
        for result in results {
            print!("{}", result);
        }
        println!();
    }
}

fn parseInput(lines: &Vec<String>) -> (Ship, Vec<Instruction>){
    let mut ship = Ship {
        stacks: vec![],
    };

    let mut instructions = vec![];

    for line in lines {
        if !Container::can_parse(line) {
            continue;
        }

        let containers = Container::parse(line);

        for (pos, container) in containers {
            let stack_idx = Ship::get_container_index(pos);
            while (ship.stacks.len() as u32) <= stack_idx {
                ship.stacks.push(Stack {
                    containers: vec![],
                    label: ship.stacks.len() as u32,
                });
            }
            ship.stacks[stack_idx as usize].containers.insert(0, container);
        }
    }

    for line in lines {
        if !Instruction::can_parse(line) {
            continue;
        }

        instructions.push(Instruction::parse(line));
    };

    (ship, instructions)
}

fn main () {
    let lines: Vec<String> = io::stdin().lines()
        .map(|x| x.unwrap())
        .collect();
    let (ship, instructions) = parseInput(&lines);

    let mut runner = InstructionRunner {
        ship: ship,
        instructions: instructions
    };

    runner.run();
    runner.print_result();
}
