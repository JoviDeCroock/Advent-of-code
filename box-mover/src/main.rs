use std::{fs, collections::VecDeque};

fn main() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    let (arrangement, instructions) = contents.split_once("\n\n").unwrap();

    // We split the arrangement into lines and reverse so
    // we can construct vectors to pop on and off.
    let mut arrangement = arrangement.lines().rev();
    // We remove the column identifier and construct our 2-dimensional
    // vector.
    let mut ordered_boxes: Vec<VecDeque<char>> = arrangement
        .next()
        .unwrap()
        .split_whitespace()
        .map(|_| VecDeque::<char>::new())
        .collect();

    arrangement
        .map(|l| {
            // We take lines like [x] [y] [z]
            // and transform them into (column, x)
            l.chars()
                .skip(1)
                .enumerate()
                .filter_map(|(i, c)| if i % 4 == 0 { Some((i / 4, c)) } else { None })
                .filter(|(_, c)| *c != ' ')
                .collect::<Vec<(usize, char)>>()
        })
        .flatten()
        .for_each(|(i, c)| ordered_boxes[i].push_front(c));

    // Now we go over all instructions and apply them to our vector
    // we parse it into a list that tells us (boxes_moved, from_col, to_col)
    instructions
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|c| c.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .map(|m| (m[0], m[1] - 1, m[2] - 1))
        .for_each(|(num, from, to)| {
            let m: Vec<char> = (0..num)
                .map(|_| ordered_boxes[from].pop_front().unwrap())
                .collect();

            // Solution for the second part
            // m.iter().rev().for_each(|c| ordered_boxes[to].push_front(*c));
            // Solution for the first part
            m.iter().for_each(|c| ordered_boxes[to].push_front(*c));
        });

    println!("{}", ordered_boxes.iter().map(|s| s[0]).collect::<String>());
}
