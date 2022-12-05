fn main() {
    let input = get_input();
    println!("Got {} characters of input.", input.len());

    let input_section = get_input_sections(input);
    println!("Got:\n{}\nas stack setup.", input_section.stack_setup);
    println!("Got:\n{}\nas move input.", input_section.move_setup);

    let mut stacks = parse_stacks(input_section.stack_setup);
    println!("Parsed {} stacks.", stacks.len());
    print_stacks(&stacks);

    let moves = parse_moves(input_section.move_setup);
    println!("Parsed {} moves.", moves.len());
    println!("First move: {}", moves[0]);

    println!("Processing moves...");
    stacks = process_moves(stacks, moves);
    println!("Moves complete!");
    print_stacks(&stacks);
}

struct Input{
    stack_setup: String,
    move_setup: String
}

struct Stack{
    boxes :Vec<char>
}

struct MoveCommand {
    count: i32,
    from: usize,
    to: usize
}

impl std::fmt::Display for MoveCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}x {}->{}", self.count, self.from, self.to)
    }
}

fn process_moves(mut stacks :Vec<Stack>, moves :Vec<MoveCommand>) -> Vec<Stack>{
    for move_command in moves {
        for _move_count in 0..move_command.count {
            let crate_letter = stacks[move_command.from - 1].boxes.pop();
            stacks[move_command.to - 1].boxes.push(crate_letter.unwrap());
        }
    }
    return stacks;
}

fn parse_moves(move_setup :String) -> Vec<MoveCommand> {
    let mut moves = Vec::new();

    for move_line in move_setup.split("\n") {
        let move_command = parse_move(move_line.to_string());
        moves.push(move_command);
    }

    return moves;
}

fn parse_move(move_line :String) -> MoveCommand {
    let chunks = move_line.split(" ").collect::<Vec<&str>>();
    return MoveCommand {
        count: chunks[1].parse::<i32>().unwrap(),
        from: chunks[3].parse::<usize>().unwrap(),
        to: chunks[5].parse::<usize>().unwrap()
    };
}

fn print_stacks(stacks :&Vec<Stack>){
    println!("Stacks:");
    for stack in stacks {
        println!("  {:?}", stack.boxes);
    }
}

fn parse_stacks(stack_input :String) -> Vec<Stack> {
    let mut stacks = Vec::new();

    for stack_row in stack_input.split("\n") {
        for (index, character) in stack_row.chars().into_iter().enumerate() {
            if "ABCDEFGHIJKLMNOPQRSTUVWXYZ".contains(character) {
                let stack_index = (index - 1) / 4;
                while stacks.len() <= stack_index {
                    stacks.push(Stack{
                        boxes :Vec::new()
                    })
                }
                stacks[stack_index].boxes.insert(0, character);
            }
        }
    }

    return stacks;
}

fn get_input_sections(input :String) -> Input {
    return Input {
        stack_setup: input[0..input.find("\n\n").unwrap()].to_string(),
        move_setup: input[(input.find("\n\n").unwrap())+2..input.len()].to_string() }
}

fn get_input() -> String {
    let mut input = String::new();
    input.push_str("    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2");
    return input;
}