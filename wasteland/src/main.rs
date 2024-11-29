use std::collections::HashMap;
use std::cmp::{max, min};

fn get_cycle_length(current_node: &str, nodes: &HashMap<&str, (String, String)>, directions: &Vec<char>) -> usize {
    let mut length = 0;
    let mut current_node = current_node;

    while current_node.chars().last().unwrap() != 'Z' {
        for dir in directions {
            let (left, right) = nodes.get(current_node).unwrap();
            match dir {
                'L' => {
                    current_node = &left;
                }
                'R' => {
                    current_node = &right;
                }
                _ => panic!("Invalid direction"),
            }
            length += 1;
            if current_node.chars().last().unwrap() == 'Z' {
                break;
            }
        }
    }

    length
}

fn main() {
    let input = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    let instructions = input.get(0).unwrap().chars().collect::<Vec<char>>();

    let mut items: HashMap<&str, (String, String)> = HashMap::new();
    for row in input.iter().skip(2) {
        let (key, left_right) = row.split_once('=').unwrap();
        let (left, right) = left_right.split_once(',').unwrap();
        let right = right.trim().replace(')', "");
        let left = left.trim().replace('(', "");
        items.insert(key.trim(), (left, right));
    }

    let mut steps = 0;
    let mut instruction_pointer = 0;
    let mut current_key = "AAA";
    while current_key != "ZZZ" {
        let instruction = instructions.get(instruction_pointer).unwrap();
        let item = items.get(current_key.trim()).unwrap();
        match instruction {
            'L' => {
                current_key = &item.0;
            }
            'R' => {
                current_key = &item.1;
            }
            _ => {}
        }

        instruction_pointer += 1;
        steps += 1;
        if instruction_pointer + 1 > instructions.len() {
            instruction_pointer = 0;
        }
    }

    println!("Steps: {}", steps);

    let input = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    let instructions = input.get(0).unwrap().chars().collect::<Vec<char>>();

    let mut items: HashMap<&str, (String, String)> = HashMap::new();
    let mut points = vec![];
    for row in input.iter().skip(2) {
        let (key, left_right) = row.split_once('=').unwrap();
        let key = key.trim();
        let (left, right) = left_right.split_once(',').unwrap();
        let right = right.trim().replace(')', "");
        let left = left.trim().replace('(', "");
        items.insert(key, (left, right));

        if key.ends_with('A') {
            points.push(key);
        }
    }

    let cycle_lengths = points
        .iter()
        .map(|n| get_cycle_length(n, &items, &instructions)).collect::<Vec<usize>>();

    dbg!(cycle_lengths.clone());
    let mut steps = 1;

    for item in cycle_lengths {
        println!("Steps: {}", steps);
        steps = lcm(steps, item);
        println!("Steps: {}", steps);
    }

    println!("Steps: {}", steps)
}

fn gcf(a: usize, b: usize) -> usize {
    let min_val = min(a, b);
    let max_val = max(a, b);
    if min_val == 0 {
        return max_val
    }
    gcf(min_val, max_val % min_val)
}

fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcf(a, b))
}
