
fn main() {
    let input = get_input();
    println!("Got {} characters of input.", input.len());

    let elves = consolidate_elf_foods(input);
    println!("Got {} elves.", elves.len());
    
    calculate_food_sum(elves);
}

fn consolidate_elf_foods(input :String) -> Vec<i32> {
    let mut elves = Vec::new();
    let mut total = 0;
    let input_array = input.split('\n');
    
    println!("Got {} lines of input.", input.len());
    
    for input in input_array {
        if input.is_empty() {
            elves.push(total);
            total = 0;
        }else{
            let current = input.parse::<i32>().unwrap();
            total += current;
        }
    } 
    elves.push(total);
    elves.sort();
    
    return elves;
}

fn calculate_food_sum(elves :Vec<i32>) {
    let mut sum = 0;
    for i in 1..=3 {
        let index = elves.len() - i;
        println!("Elf {}: {}", index+1, elves[index]);
        sum += elves[index];
    }
    println!("Total: {}", sum);
}

fn get_input() -> String {
    let mut input = String::new();
    input.push_str("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000");
    return input;
}
