fn main() {
    let input = get_input();
    println!("Got {} characters of input.", input.len());

    process_rps_scores(input);
}

fn process_rps_scores(input :String) {
    let input_array = input.split('\n');
    let mut total_score = 0;
    
    for round in input_array {
        total_score += calculate_score(round.to_string());
    }
    println!("Total Score: {}", total_score);
}

fn calculate_score(round :String) -> i32 {
    let opponent = round.chars().nth(0).unwrap();
    let outcome = round.chars().nth(2).unwrap();
    let mut score = 0;
    
    let choice = decode_guide(outcome, opponent);
    
    if choice == 'X' {
        score += 1;
        if opponent == 'A' { score += 3; }
        if opponent == 'B' { score += 0; }
        if opponent == 'C' { score += 6; }
    }
    if choice == 'Y' {
        score += 2;
        if opponent == 'A' { score += 6; }
        if opponent == 'B' { score += 3; }
        if opponent == 'C' { score += 0; }
    }
    if choice == 'Z' {
        score += 3;
        if opponent == 'A' { score += 0; }
        if opponent == 'B' { score += 6; }
        if opponent == 'C' { score += 3; }
    }
    
    return score;
}
 
fn decode_guide(outcome :char, opponent :char) -> char {
    if outcome == 'X' { // Lose
        if opponent == 'A' { return 'Z'; }
        if opponent == 'B' { return 'X'; }
        return 'Y';
    }
    if outcome == 'Y' { // Draw
        if opponent == 'A' { return 'X'; }
        if opponent == 'B' { return 'Y'; }
        return 'Z';
    }
    // Win
    if opponent == 'A' { return 'Y'; }
    if opponent == 'B' { return 'Z'; }
    return 'X';
}

fn get_input() -> String {
    let mut input = String::new();
    input.push_str("A Y
B X
C Z");
    return input;
}
