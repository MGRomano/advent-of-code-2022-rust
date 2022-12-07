use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = get_input();
    println!("Got {} characters of input.", input.len());

    let commands = get_commands(&input);
    println!("Got {} commands.", commands.len());

    let terminal_state = process_commands(&commands);
    println!("Final state: {}", terminal_state);

    let directory_sizes = get_directory_sizes(&terminal_state);
    println!("Directory Sizes: {:?}", directory_sizes);

    let max_size = 100000;
    let matching_directories = find_directories_under(&directory_sizes, &max_size);
    println!("Matching Directories: {:?}", matching_directories);

    let sizes_sum :u64 = matching_directories.iter().map(|(_directory, size)| *size).sum();
    println!("Sum of matching directories: {:?}", sizes_sum);
    //1002724
}

struct TerminalState {
    current_directory: Vec<String>,
    directories: HashSet<String>,
    file_sizes: HashMap<String, u64>
}

impl std::fmt::Display for TerminalState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Current Directory: {:?}\nDirectories: {:?}\nSizes: {:?}", self.current_directory, self.directories, self.file_sizes)
    }
}

fn get_directory_sizes(state :&TerminalState) -> HashMap<String, u64>{
    let mut directory_sizes = HashMap::new();

    for directory in &state.directories {
        let mut directory_size = 0;

        for (file, file_size) in &state.file_sizes {
            let is_file_in_directory = file.starts_with(directory);
            if is_file_in_directory {
                directory_size += file_size;
            }
        }

        directory_sizes.insert(directory.to_string(), directory_size);
    }
    return directory_sizes;
}

fn find_directories_under<'a>(directory_sizes :&'a HashMap<String, u64>, max_size :&'a u64) -> HashMap<&'a String, &'a u64> {
    let mut matching_directories = HashMap::new();
    for (directory, directory_size) in directory_sizes {
        if directory_size <= max_size {
            matching_directories.insert(directory, directory_size);
        }
    }
    return matching_directories;
}

fn process_commands(commands :&Vec<String>) -> TerminalState {
    let mut state = TerminalState{
        current_directory: Vec::new(),
        directories: HashSet::new(),
        file_sizes: HashMap::new()
    };

    for command in commands {
        state = process_command(&command, state);
    }

    return state;
}

fn process_command(command :&String, mut state :TerminalState) -> TerminalState{
    if command.trim().eq_ignore_ascii_case("$ cd /") {
        state.current_directory.clear();
        state.current_directory.push("".to_string());
        println!("Root {}: {:?}", command, state.current_directory);
    }else if command.trim().eq_ignore_ascii_case("$ cd ..") {
        state.current_directory.pop();
        println!("Pop {}: {:?}", command, state.current_directory);
    }else if command.trim().starts_with("$ cd") {
        let command_start = "$ cd ";
        let new_directory = &command[command_start.len()..command.len()];
        state.current_directory.push(new_directory.to_string());
        println!("Down {}: {:?}", command, state.current_directory);
    }else if is_file_size(&command) {
        state = add_size(&command, state);
        println!("Size {}: {:?}", command, state.current_directory);
    }else{
        println!("Skip {}", command);
    }
    return state;
}

fn add_size(command :&String, mut state :TerminalState) -> TerminalState {
    let file_size = command.split(' ').nth(0).unwrap().parse::<u64>().unwrap();
    let file_name = command.split(' ').nth(1).unwrap();
    let directory_name = state.current_directory.iter().map(|d| d.to_string() + "/").collect::<String>();
    let full_name = directory_name + file_name;

    let mut current_directory = String::new();
    for folder in state.current_directory.iter() {
        current_directory += folder;
        current_directory += "/";
        state.directories.insert(current_directory.to_string());
    }

    state.file_sizes.insert(full_name, file_size);
    return state;
}

fn is_file_size(command :&String) -> bool {
    let first_character = &command[0..1];
    return "0123456789".contains(first_character);
}

fn get_commands(input :&String) -> Vec<String> {
    let mut commands = Vec::new();
    for line in input.split('\n') {
        commands.push(line.to_string());
    }
    return commands;
}

fn get_input() -> String {
    let mut input = String::new();
    input.push_str("$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
    7214296 k");
    return input;
}