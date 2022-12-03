use std::collections::HashSet;

fn main() {
    let input = get_input();
    println!("Got {} characters of input.", input.len());

    let rucksacks = get_rucksacks(input);
    println!("Got {} rucksacks.", rucksacks.len());

    let total_score = get_matching_items_total_score(rucksacks);
    println!("Total Score: {}", total_score);
}

fn get_matching_items_total_score(rucksacks :Vec<String>) -> i32 {
    for rucksack in rucksacks {
        let mut left = HashSet::new();
        let mut right = HashSet::new();

        for index in 0..rucksack.len() {
            let item = rucksack.chars().nth(index).unwrap();
            if index < rucksack.len() / 2 {
                left.insert(item);
            }else{
                right.insert(item);
            }
        }

        let matching_items = get_matching_items(left, right);
        println!("Matching Items: {:?}", matching_items);
    }

    return 0;
}

fn get_matching_items(left :HashSet<char>, right :HashSet<char>) -> HashSet<char> {
    let mut matches = HashSet::new();
    for item in left {
        if right.contains(&item){
            matches.insert(item);
        }
    }
    return matches;
}

fn get_rucksacks(input :String) -> Vec<String> {
    let mut rucksacks = Vec::new();
    for rucksack in input.split('\n') {
        rucksacks.push(rucksack.to_string());
    }
    return rucksacks;
}

fn get_input() -> String {
    let mut input = String::new();
    input.push_str("vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw");
    return input;
}
