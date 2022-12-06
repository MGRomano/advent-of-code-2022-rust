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
        let message_start = get_message_start(&message, 14);
        println!("Message {} starts at {}.", count, message_start);
        count += 1;
    }
}

fn get_message_start(message :&String, length :usize) -> usize {
    for i in (length-1)..message.len() {
        let mut unique_characters: HashSet<char> = HashSet::new();
        for j in 0..length {
            unique_characters.insert(message.chars().nth(i-j).unwrap());
        }

        if unique_characters.len() == length {
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