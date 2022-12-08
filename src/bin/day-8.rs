use std::cmp;

fn main() {
    let input = get_input();
    println!("Got {} characters of input.", input.len());

    let grid = get_grid(&input);
    println!("Grid: {:?}", grid);

    let high_score = get_high_score(&grid);
    println!("High score: {:?}", high_score);
}

fn get_high_score(grid :&Vec<Vec<u32>>) -> u32{
    let mut high_score = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            let current_score = get_score(&x, &y, &grid);
            high_score = cmp::max(high_score, current_score);
        }
    }
    return high_score;
}

fn get_score(x :&usize, y :&usize, grid :&Vec<Vec<u32>>) -> u32{
    let mut total_score = 1;
    total_score *= get_score_in_direction(x, y, grid, 0, -1);
    total_score *= get_score_in_direction(x, y, grid, 0, 1);
    total_score *= get_score_in_direction(x, y, grid, -1, 0);
    total_score *= get_score_in_direction(x, y, grid, 1, 0);
    println!("Score [{},{}]: {}", x, y, total_score);
    return total_score;
}

fn get_score_in_direction(x :&usize, y :&usize, grid :&Vec<Vec<u32>>, x_offset :i8, y_offset :i8) -> u32{
    let tree = grid[*x][*y];
    let max_x = grid.len();
    let max_y = grid[0].len();
    let mut current_x = adjust(&(x+1), &x_offset);
    let mut current_y = adjust(&(y+1), &y_offset);
    let mut score = 0;

    while current_x > usize::MIN && current_x <= max_x && current_y > usize::MIN && current_y <= max_y {
        score += 1;
        let blocking_tree = grid[current_x-1][current_y-1];
        if blocking_tree >= tree {
            return score;
        }
        current_x = adjust(&current_x, &x_offset);
        current_y = adjust(&current_y, &y_offset);
    }
    return score;
}

fn adjust(index :&usize, offset :&i8) -> usize {
    let abs_offset = offset.unsigned_abs();
    if offset >= &0 {
        return index + usize::from(abs_offset);
    }else{
        if index == &usize::MIN {
            return 0;
        }
        return index - usize::from(abs_offset);
    }
}

fn get_grid(input :&String) -> Vec<Vec<u32>>{
    let height = input.split('\n').count();
    let width = input.find("\n").unwrap();

    let mut grid = vec![vec![0; width]; height];
    let mut row_index = 0;

    for row in input.split('\n') {
        for column_index in 0..width {
            let tree = row.chars().nth(column_index).unwrap().to_digit(10).unwrap();
            grid[column_index][row_index] = tree;
        }
        row_index += 1;
    }

    return grid;
}

fn get_input() -> String {
    let mut input = String::new();
    input.push_str("30373
25512
65332
33549
35390");
    return input;
}