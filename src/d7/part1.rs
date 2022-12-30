use std::io;

struct File {
    name: String,
    size: u32,
}

impl File {
    fn parse(input: &String) -> Result<File, String> {
        let split: Vec<&str> = input.split(' ').collect();
        let mut iter = split.iter();
        let size = iter.next().unwrap().parse::<u32>().unwrap();
        let name = String::from(*iter.next().unwrap());

        Ok(File { name, size })
    }
}

impl FileSystemItem for File {
    fn size(&self) -> u32 {
        return self.size
    }

    fn is_directory(&self) -> bool {
        return false;
    }

    fn name(&self) -> &str {
        return self.name.as_str();
    }
}

struct Directory {
    files: Vec<File>,
    directories: Vec<Directory>,
    name: String,
}

impl Directory {
    fn parse(input: &String) -> Result<Directory, String> {
        let split: Vec<&str> = input.split(' ').collect();
        let mut iter = split.iter();
        let dir = iter.next().unwrap();
        let name = String::from(*iter.next().unwrap());

        if dir != &"dir" {
            Err::<Directory, String>(format!("'dir' keyword not found in input: {}", input));
        }

        Ok(Directory {
            name,
            files: vec![],
            directories: vec![]
        })
    }
}

impl FileSystemItem for Directory {
    fn size(&self) -> u32 {
        return self.files
            .iter()
            .map(|x| x.size())
            .sum::<u32>() +
            self.directories
            .iter()
            .map(|x| x.size())
            .sum::<u32>();
    }

    fn is_directory(&self) -> bool {
        return true;
    }

    fn name(&self) -> &str {
        return self.name.as_str();
    }
}

trait FileSystemItem {
    fn size(&self) -> u32;
    fn is_directory(&self) -> bool;
    fn name(&self) -> &str;
}

struct FileSystem<'a> {
    root: Directory,
    current_directory: &'a mut Directory,
}

struct CommandProcessor<'a> {
    file_system: &'a mut FileSystem<'a>,
    commands: &'a Vec<Box<dyn ShellCommand>>,
}

impl CommandProcessor<'_> {
    fn process(&self) {

    }
}

struct ListCommand {
    output: Vec<String>,
}

enum FileSystemItemType {
    Directory,
    File
}

impl ListCommand {
    fn parse(input: &String) -> Result<ListCommand, String> {
        Ok(ListCommand { output: vec![] })
    }

    fn get_file_system_item_type(input: &String) -> FileSystemItemType {
        if input.contains("dir") {
            FileSystemItemType::Directory
        } else {
            FileSystemItemType::File
        }
    }
}

struct ChangeDirectoryCommand {
    directory: String,
}

impl ChangeDirectoryCommand {
    fn parse(input: &String) -> Result<ChangeDirectoryCommand, String> {
        let split: Vec<&str> = input.split(' ').collect();
        let mut iter = split.iter();
        let dollar = iter.next().unwrap();
        let cd = iter.next().unwrap();
        let directory = String::from(*iter.next().unwrap());

        if dollar != &"$" {
            Err::<ChangeDirectoryCommand, String>(format!("Expected shell prompt symbol: {}", input));
        }

        if cd != &"cd" {
            Err::<ChangeDirectoryCommand, String>(format!("Expected cd command: {}", input));
        }

        Ok(ChangeDirectoryCommand { directory })
    }
}

trait ShellCommand {
    fn add_output(&mut self, line: String);
    fn print(&self);
    fn run_command<'a>(&self, file_system: &'a mut FileSystem);
}

impl ShellCommand for ChangeDirectoryCommand {
    fn print(&self) {
        println!("Change Directory Command");
        println!("directory: {}", self.directory);
        println!();
    }

    fn add_output(&mut self, line: String) {}

    fn run_command<'a>(&self, file_system: &'a mut FileSystem) {
        if self.directory == "/" {
            return;
        }

        file_system.current_directory = file_system.current_directory
            .directories
            .iter_mut()
            .find(|x| x.name() == self.directory)
            .unwrap();

    }
}

impl ShellCommand for ListCommand {
    fn add_output(&mut self, line: String) {
        self.output.push(line);
    }

    fn print(&self) {
        println!("List Command");
        println!("Output: ");
        for output in &self.output {
            println!("    {}", output);
        }
        println!();
    }

    fn run_command<'a>(&self, file_system: &'a mut FileSystem) {
        for output in &self.output {
            match ListCommand::get_file_system_item_type(output) {
                FileSystemItemType::File => {
                    let file = File::parse(output).unwrap();
                    file_system.current_directory.files.push(file);
                },
                FileSystemItemType::Directory => {
                    let directory = Directory::parse(output).unwrap();
                    file_system.current_directory.directories.push(directory);
                },
                _ => println!("Err: couldn't match file system type for: {}", output),
            }
        }
    }
}

enum InputType {
    Output,
    List,
    ChangeDirectory,
}

struct InputParser;
impl InputParser {
    fn get_input_type(input: &str) -> Result<InputType, String> {
        if !input.contains('$') {
            return Ok(InputType::Output);
        }
        else if input.contains("ls") {
            return Ok(InputType::List);
        }
        else if input.contains("cd") {
            return Ok(InputType::ChangeDirectory);
        }

        Err(format!("Unexpected input type: {}", input))
    }
}

fn main () {
    let lines = io::stdin().lines();
    let mut commands: Vec<Box<dyn ShellCommand>> = vec![];

    for line in lines.into_iter() {
        let input = line.unwrap();

        match InputParser::get_input_type(&input) {
            Ok(InputType::List) => {
                let list = ListCommand::parse(&input).unwrap();
                commands.push(Box::new(list));
            },
            Ok(InputType::ChangeDirectory) => {
                let cd = ChangeDirectoryCommand::parse(&input).unwrap();
                commands.push(Box::new(cd));
            },
            Ok(InputType::Output) => {
                commands.last_mut().unwrap().add_output(input)
            },
            Err(err) => panic!("Input Parsing Error Occured: {}", err),
        }
    }

    for command in commands {
        command.print();
    }
}
