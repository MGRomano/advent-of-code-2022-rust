fn main() {
    let input = get_input();
    println!("Got {} characters of input.", input.len());

    let pairs = parse_pairs(input);
    println!("Got {} pairs.", pairs.len());

    let overlap_count = count_overlaps(pairs);
    println!("Got {} overlaps.", overlap_count);
}

fn count_overlaps(pairs :Vec<Pair>) -> i32 {
    let mut overlap_count = 0;

    for pair in pairs {
        if is_overlapping(pair) {
            overlap_count += 1;
        }
    }

    return overlap_count;
}

fn is_overlapping(pair :Pair) -> bool {
    return pair.first.min <= pair.second.min && pair.first.max >= pair.second.max ||
           pair.first.min >= pair.second.min && pair.first.max <= pair.second.max;
}

fn parse_pairs(input :String) -> Vec<Pair> {
    let mut pairs = Vec::new();

    for pair_input in input.split('\n'){
        let mut assignments = pair_input.split(',');
        let pair = Pair{
            first: parse_assignment(assignments.next().unwrap()),
            second: parse_assignment(assignments.next().unwrap())
        };
        pairs.push(pair);
    }

    return pairs;
}

fn parse_assignment(assignment_input :&str) -> Assignment {
    let mut ranges = assignment_input.split('-');
    return Assignment {
        min: ranges.next().unwrap().parse::<i32>().unwrap(),
        max: ranges.next().unwrap().parse::<i32>().unwrap()
    };
}

struct Pair {
    first: Assignment,
    second: Assignment
}

struct Assignment {
    min: i32,
    max: i32
}

fn get_input() -> String {
    let mut input = String::new();
    input.push_str("2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8");
    return input;
}