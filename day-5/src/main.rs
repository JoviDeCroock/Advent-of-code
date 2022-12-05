use std::{fs, collections::VecDeque};

fn main() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    println!("{}", contents);
    let (arrangement, instructions) = contents.split_once("\n\n").unwrap();

    let mut start = arrangement.lines().rev();
    let mut stacks: Vec<VecDeque<char>> = start
        .next()
        .unwrap()
        .split_whitespace()
        .map(|_| VecDeque::<char>::new())
        .collect();

    start
        .map(|l| {
            l.chars()
                .skip(1)
                .enumerate()
                .filter_map(|(i, c)| if i % 4 == 0 { Some((i / 4, c)) } else { None })
                .filter(|(_, c)| *c != ' ')
                .collect::<Vec<(usize, char)>>()
        })
        .flatten()
        .for_each(|(i, c)| stacks[i].push_front(c));

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
                .map(|_| stacks[from].pop_front().unwrap())
                .collect();
            m.iter().for_each(|c| stacks[to].push_front(*c));
        });

    let out: String = stacks.iter().map(|s| s[0]).collect();
    println!("{}", out);
}
