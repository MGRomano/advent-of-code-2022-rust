use std::collections::HashSet;

fn main() {
    let input = get_input();
    println!("Got {} characters of input.", input.len());

    let rucksacks = get_rucksacks(input);
    println!("Got {} rucksacks.", rucksacks.len());

    let all_matching_badges = all_matching_badges(rucksacks);
    println!("All matching items: {:?}", all_matching_badges);

    let total_score = score_items(all_matching_badges);
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

struct Troop{
    first: Vec<HashSet<char>>,
    second: Vec<HashSet<char>>,
    third: Vec<HashSet<char>>
}

fn all_matching_badges(rucksacks :Vec<String>) -> Vec<char> {
    let all_matching_badges = Vec::new();

    let troops = get_troops(rucksacks);
    for troop in troops{
        let mut only_matches = HashSet::new();
        only_matches = get_only_matches(troop.first, only_matches);
        only_matches = get_only_matches(troop.second, only_matches);
        only_matches = get_only_matches(troop.third, only_matches);
    }

    return all_matching_badges;
}

fn get_only_matches(elf_pack :Vec<HashSet<char>>, previous_matches :HashSet<char>) -> HashSet<char> {
    let fill_all = previous_matches.is_empty();
    let mut current_matches = HashSet::new();

    for side in elf_pack {
        for item in side {
            if fill_all || previous_matches.contains(&item) {
                current_matches.insert(item);
            }
        }
    }

    return current_matches;
}

fn get_troops(rucksacks :Vec<String>) -> Vec<Troop> {
    let mut troops = Vec::new();

    for index in (0..rucksacks.len()).step_by(3) {
        let troop = Troop {
            first: get_item_sides(rucksacks.get(index).unwrap().to_string()),
            second: get_item_sides(rucksacks.get(index+1).unwrap().to_string()),
            third: get_item_sides(rucksacks.get(index+2).unwrap().to_string())
        };
        troops.push(troop);
    }

    return troops;
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
