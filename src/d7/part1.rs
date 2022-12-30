use std::io;
use std::collections::HashMap;
use std::collections::HashSet;

struct ChangeDirectory {
    new_directory: String,
}

fn parse_change_directory(input: &String) -> ChangeDirectory {
    let new_directory = input.split(' ').last().unwrap().to_string();

    ChangeDirectory {
        new_directory
    }
}

struct Directory {
    name: String,
}

fn parse_directory(input: &String) -> Directory {
    let name = input.split(' ').last().unwrap().to_string();

    Directory {
        name
    }
}

struct File {
    name: String,
    size: u32,
}

fn parse_file(input: &String) -> File {
    let mut split = input.split(' ');
    let size = split.next().unwrap().parse::<u32>().unwrap();
    let name = split.next().unwrap().to_string();

    File {
        name,
        size,
    }
}

fn change_directory(directory: &mut Vec<String>, new_directory: &String) {
    let parent_directory: String = "..".to_string();
    if new_directory == &parent_directory {
        directory.pop();
    } else {
        directory.push(new_directory.to_string());
    }
}

fn main () {
    let lines = io::stdin().lines();

    let mut directory: Vec<String> = vec![];
    let mut files: HashMap<String, u32> = HashMap::new();
    let mut directories: HashSet<String> = HashSet::new();
    directories.insert("/".to_string());

    for line in lines.into_iter() {
        let input = line.unwrap();
        let split: Vec<&str> = input.split(' ').collect();

        if split[1] == "cd" {
            let new_directory = parse_change_directory(&input).new_directory;
            change_directory(&mut directory, &new_directory);
        } else if split[0] == "dir" {
            let name = parse_directory(&input).name;
            let dir_name = directory.join("/") + "/" + &name;
            directories.insert(dir_name);
        } else if split[1] == "ls" {
            continue;
        } else {
            let File { name, size } = parse_file(&input);
            let dir_name = directory.join("/") + "/" + &name;
            files.insert(dir_name, size);
        }
    }

    let size = directories
        .iter()
        .map(|x| files
            .iter()
            .filter(|y| y.0.starts_with(x))
            .map(|y| y.1)
            .sum::<u32>()
        )
        .filter(|x| x <= &100000)
        .sum::<u32>();

    println!("Calulated size: {}", size);
}
