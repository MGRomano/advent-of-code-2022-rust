fn main() {
    let input = get_input();
    println!("Got {} characters of input.", input.len());

    let state = initialize_system_state(&input);
    println!("Got initial state with {} commands.", state.instructions.len());

    process_commands(state);
}

struct SystemState{
    register: i32,
    instructions: Vec<String>,
    current_instruction_index: usize,
    is_processing: bool
}

fn process_commands(mut state :SystemState){
    let mut cycle = 1;
    let mut signal_sum = 0;

    while is_still_processing(&state) && cycle < 10000 {
        let current_command = get_current_command(&state);

        if "noop" == current_command {
            state.current_instruction_index += 1;
            state.is_processing = false;
        } else if state.is_processing {
            let value_change = current_command.split(' ').nth(1).unwrap().parse::<i32>().unwrap();
            state.register += value_change;
            state.is_processing = false;
            state.current_instruction_index += 1;
        } else {
            state.is_processing = true;
        }

        println!("Cycle {} [x:{}] command: {}", cycle, state.register, current_command);
        cycle += 1;

        if (cycle - 20) % 40 == 0 {
            let signal_strength = cycle * state.register;
            println!(" --> Cycle {} Signal Strength: {}", cycle, signal_strength);
            signal_sum += signal_strength;
        }
    }

    println!("Total Signal Strength: {}", signal_sum);
}

fn get_current_command(state :&SystemState) -> String {
    let current_index = state.current_instruction_index;
    return state.instructions[current_index].to_string();
}

fn is_still_processing(state :&SystemState) -> bool {
    return state.current_instruction_index < state.instructions.len();
}

fn initialize_system_state(input :&String) -> SystemState{
    let instruction_list = parse_input(input);
    let state = SystemState{
        register: 1,
        instructions: instruction_list,
        current_instruction_index: 0,
        is_processing: false
    };
    return state;
}

fn parse_input(input :&String) -> Vec<String>{
    let mut commands = Vec::new();
    for line in input.split('\n') {
        commands.push(line.to_string());
    }
    return commands;
}

fn get_input() -> String {
    let mut input = String::new();
    input.push_str("addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop");
    return input;
}