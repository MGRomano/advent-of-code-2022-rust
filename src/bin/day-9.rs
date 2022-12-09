use std::collections::HashSet;

fn main() {
    let input = get_input();
    println!("Got {} characters of input.", input.len());

    let moves = get_moves(&input);
    println!("Got {} moves.", moves.len());

    let mut state = initialize_game_state();
    println!("Initial state: {}", state);

    state = process_moves(state, moves);
    println!("Locations visited: {:?}", state.locations_visited);
    println!("Final unique count: {}", state.locations_visited.len());
}

struct GameState {
    head: [i32; 2],
    tail: [i32; 2],
    locations_visited: HashSet<[i32; 2]>
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Head: {:?} - Tail: {:?}", self.head, self.tail)
    }
}

fn process_moves(mut state :GameState, moves :Vec<String>) -> GameState {
    for next_move in moves {
        let direction = next_move.chars().nth(0).unwrap();
        let count = next_move.split(' ').nth(1).unwrap().parse::<i32>().unwrap();
        for _i in 0..count {
            state = process_move(state, direction);
        }
    }
    return state;
}

fn process_move(mut state :GameState, direction :char) -> GameState {
    let move_direction = get_move_direction(&direction);
    state.head = array_add(state.head, move_direction);
    state.tail = catch_up(state.head, state.tail);
    state.locations_visited.insert(state.tail);
    return state;
}

fn catch_up(head :[i32; 2], tail: [i32; 2]) -> [i32; 2] {
    let mut new_location = [tail[0], tail[1]];
    if head[0] == tail[0] || head[1] == tail[1] {
        if head[0] - tail[0] == 2 { new_location[0] += 1; }
        if head[0] - tail[0] == -2 { new_location[0] += -1; }
        if head[1] - tail[1] == 2 { new_location[1] += 1; }
        if head[1] - tail[1] == -2 { new_location[1] += -1; }
    } else if (head[0] - tail[0]).abs() > 1 || (head[1] - tail[1]).abs() > 1 {
        if head[0] > tail [0] && head[1] > tail[1] { new_location[0] += 1; new_location[1] += 1; }
        if head[0] > tail [0] && head[1] < tail[1] { new_location[0] += 1; new_location[1] -= 1; }
        if head[0] < tail [0] && head[1] > tail[1] { new_location[0] -= 1; new_location[1] += 1; }
        if head[0] < tail [0] && head[1] < tail[1] { new_location[0] -= 1; new_location[1] -= 1; }
    }
    return new_location;
}

fn array_add(left :[i32; 2], right :[i32; 2]) -> [i32; 2] {
    return [(left[0]+right[0]), (left[1]+right[1])];
}

fn get_move_direction(direction :&char) -> [i32; 2] {
    if &'U' == direction { return [0, -1]; }
    if &'D' == direction { return [0, 1]; }
    if &'L' == direction { return [-1, 0]; }
    if &'R' == direction { return [1, 0]; }
    return [0, 0];
}

fn initialize_game_state() -> GameState {
    return GameState{
        head: [0, 0],
        tail: [0, 0],
        locations_visited: HashSet::new()
    };
}

fn get_moves(input :&String) -> Vec<String> {
    let mut commands = Vec::new();
    for line in input.split('\n') {
        commands.push(line.to_string());
    }
    return commands;
}

fn get_input() -> String {
    let mut input = String::new();
    input.push_str("R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2");
    return input;
}