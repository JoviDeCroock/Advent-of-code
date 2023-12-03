use std::collections::HashMap;

fn is_qualified_symbol(c: char) -> bool {
    !c.is_numeric() && c != '.'
}

fn is_gear(c: char) -> bool {
    c == '*'
}

fn consider_number_neighbors(board: &Vec<String>, gear_nums: &mut HashMap<(usize, usize), Vec<i32>>, start_y: usize, start_x: usize, end_y: usize, end_x: usize, num: i32) -> bool {
    for y in start_y..=end_y {
        for x in start_x..=end_x {
            if y < board.len() && x < board[y].len() {
                let symb = board[y].chars().nth(x).unwrap_or('.');
                if is_qualified_symbol(symb) {
                    if is_gear(symb) {
                        gear_nums.entry((y, x)).or_insert(vec![]).push(num);
                    }

                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    let mut total = 0;
    let mut board = Vec::new();
    let mut gear_nums = HashMap::new();

    let num_pattern = regex::Regex::new(r"\d+").unwrap();

    let input = include_str!("../input.txt");

    for line in input.lines() {
        board.push(line.trim().to_string());
    }

    for (row_num, line) in board.iter().enumerate() {
        for mat in num_pattern.find_iter(line) {
            if consider_number_neighbors(&board, &mut gear_nums, row_num.saturating_sub(1), mat.start().saturating_sub(1), row_num + 1, mat.end(), mat.as_str().parse().unwrap()) {
                total += mat.as_str().parse::<i32>().unwrap();
            }
        }
    }

    println!("{}", total);

    let mut rat_total = 0;
    for (_, v) in gear_nums.iter() {
        if v.len() == 2 {
            rat_total += v[0] * v[1];
        }
    }

    println!("{}", rat_total);
}