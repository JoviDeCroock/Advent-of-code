use std::{fs, collections::HashSet};

fn main() {
    let alphabet = vec![
        'a', 'b', 'c', 'd', 'e', 
        'f', 'g', 'h', 'i', 'j', 
        'k', 'l', 'm', 'n', 'o',
        'p', 'q', 'r', 's', 't', 
        'u', 'v', 'w', 'x', 'y', 
        'z', 'A', 'B', 'C', 'D',
        'E', 'F', 'G', 'H', 'I',
        'J', 'K', 'L', 'M', 'N',
        'O', 'P', 'Q', 'R', 'S',
        'T', 'U', 'V', 'W', 'X',
        'Y', 'Z'
    ];

    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    let items = contents.lines().collect::<Vec<&str>>();

    // Find common item in both halfs of the backpack
    let common_items = items.iter().flat_map(|item| {
        let length = item.chars().count();
        let first_half = &item[..(length / 2)];
        let second_half = &item[(length / 2)..];

        let second_half_set: HashSet<char> = second_half.chars().collect();
        let mut shared_characters = HashSet::new();
        first_half.chars().for_each(|f| {
            if second_half_set.contains(&f) {
                shared_characters.insert(f);
            }
        });

        shared_characters.into_iter().collect::<Vec<char>>()
    }).collect::<Vec<char>>();

    let score = common_items.iter().fold(0, |acc, character| {
        let index = alphabet.iter().position(|x| x == character).unwrap() + 1;
        acc + index
    });

    println!("{}", score);

    // find common character in groups of 3.
    let items_length = items.len() / 3;
    let mut groups = vec![];
    for n in 0..items_length {
        let base = n * 3;
        let gnome_1 = items.get(base).unwrap();
        let gnome_2: HashSet<char> = items.get(base + 1).unwrap().chars().collect();
        let gnome_3: HashSet<char> = items.get(base + 2).unwrap().chars().collect();

        let badge = gnome_1.chars().find(|c| {
            gnome_2.contains(c) && gnome_3.contains(c)
        }).unwrap();

        groups.push(badge)
    }

    let score = groups.iter().fold(0, |acc, character| {
        let index = alphabet.iter().position(|x| x == character).unwrap() + 1;
        acc + index
    });
    println!("{}", score);
}
