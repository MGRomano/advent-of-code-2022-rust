use std::collections::HashSet;

fn main() {
    let input = get_input();
    println!("Got {} characters of input.", input.len());

    let messages = get_messages(&input);
    println!("Got {} messages.", messages.len());

    print_message_starts(&messages);
}

fn print_message_starts(messages :&Vec<String>){
    let mut count = 1;
    for message in messages {
        let message_start = get_message_start(&message);
        println!("Message {} starts at {}.", count, message_start);
        count += 1;
    }
}

fn get_message_start(message :&String) -> usize {
    for i in 3..message.len() {
        let mut unique_characters: HashSet<char> = HashSet::new();
        unique_characters.insert(message.chars().nth(i).unwrap());
        unique_characters.insert(message.chars().nth(i-1).unwrap());
        unique_characters.insert(message.chars().nth(i-2).unwrap());
        unique_characters.insert(message.chars().nth(i-3).unwrap());

        if unique_characters.len() == 4 {
            return i+1;
        }
        unique_characters.clear();
    }
    return 0;
}

fn get_messages(input :&String) -> Vec<String> {
    let mut messages = Vec::new();
    for line in input.split('\n') {
        messages.push(line.to_string());
    }
    return messages;
}

fn get_input() -> String {
    let mut input = String::new();
    input.push_str("mjqjpqmgbljsphdztnvjfqwrcgsmlb
bvwbjplbgvbhsrlpgdmjqwftvncz
nppdvjthqldpwncqszvftbrmjlhg
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
    return input;
}