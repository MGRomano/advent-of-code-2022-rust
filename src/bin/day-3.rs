use std::collections::HashSet;

fn main() {
    let input = get_input();
    println!("Got {} characters of input.", input.len());

    let rucksacks = get_rucksacks(input);
    println!("Got {} rucksacks.", rucksacks.len());

    let all_matching_items = get_all_matching_items(rucksacks);
    println!("All matching items: {:?}", all_matching_items);

    let total_score = score_items(all_matching_items);
    println!("Total score: {}", total_score);
}

fn score_items(all_matching_items :Vec<char>) -> i32 {
    let mut total = 0;
    for item in all_matching_items {
        total += score_item(item);
    }
    return total;
}

fn score_item(item :char) -> i32{
    let value = item as i32;
    if item.is_lowercase() {
        return value - 96;
    } else {
        return value - 64 + 26;
    }
}

fn get_all_matching_items(rucksacks :Vec<String>) -> Vec<char> {
    let mut all_matching_items = Vec::new();

    for rucksack in rucksacks {
        let sides = get_item_sides(rucksack);

        let matching_items = get_matching_items(sides);
        for item in matching_items {
            all_matching_items.push(item);
        }
    }

    return all_matching_items;
}

fn get_matching_items(sides :Vec<HashSet<char>>) -> HashSet<char> {
    let mut matches = HashSet::new();
    let left = sides.get(0).unwrap();
    let right = sides.get(1).unwrap();

    for item in left {
        if right.contains(item){
            matches.insert(*item);
        }
    }
    return matches;
}

fn get_item_sides(rucksack :String) -> Vec<HashSet<char>> {
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

    let mut sides = Vec::new();
    sides.push(left);
    sides.push(right);
    return sides;
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
