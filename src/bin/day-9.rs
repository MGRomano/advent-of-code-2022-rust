use std::collections::HashSet;

fn main() {
    let input = get_input();
    println!("Got {} characters of input.", input.len());

    let moves = get_moves(&input);
    println!("Got {} moves.", moves.len());

    let mut state = initialize_game_state();
    println!("Initial state: {}", state);

    state = process_moves(state, moves);
    println!("Locations visited: {:?}", state.tail_locations_visited);
    println!("Final unique count: {}", state.tail_locations_visited.len());
}

struct GameState {
    knots: Vec<[i32; 2]>,
    tail_locations_visited: HashSet<[i32; 2]>
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Knots: {:?}", self.knots)
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
    state.knots[0] = array_add(state.knots[0], move_direction);
    for i in 1..state.knots.len() {
        state.knots[i] = catch_up(state.knots[i-1], state.knots[i]);
    }
    state.tail_locations_visited.insert(*state.knots.last().unwrap());
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
    let mut state = GameState{
        knots: Vec::new(),
        tail_locations_visited: HashSet::new()
    };
    for _i in 0..10 {
        state.knots.push([0,0]);
    }
    return state;
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
    input.push_str("R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20");
    return input;
}