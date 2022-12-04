use std::{fs};

fn main() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    let pairs = contents.lines().collect::<Vec<&str>>();

    let result = pairs.iter().fold(0, |acc, pair| {
        let sections = pair.split(',').collect::<Vec<&str>>();

        let first_section_set = sections.get(0).unwrap().split("-").collect::<Vec<&str>>();
        let second_section_set = sections.get(1).unwrap().split("-").collect::<Vec<&str>>();

        let first_range = std::ops::Range { start: first_section_set.get(0).unwrap().parse::<i32>().unwrap(), end: first_section_set.get(1).unwrap().parse::<i32>().unwrap() + 1 };
        let second_range = std::ops::Range { start: second_section_set.get(0).unwrap().parse::<i32>().unwrap(), end: second_section_set.get(1).unwrap().parse::<i32>().unwrap() + 1 };
        
        if first_range.len() > second_range.len() {
            let all_matching = second_range.into_iter().all(|f| {
                first_range.contains(&f)
            });

            if all_matching {
                acc + 1
            } else {
                acc
            }
        } else {
            let all_matching = first_range.into_iter().all(|f| {
                second_range.contains(&f)
            });

            if all_matching {
                acc + 1
            } else {
                acc
            }
        }        
    });

    println!("Result: {}", result);

    let result = pairs.iter().fold(0, |acc, pair| {
        let sections = pair.split(',').collect::<Vec<&str>>();

        let first_section_set = sections.get(0).unwrap().split("-").collect::<Vec<&str>>();
        let second_section_set = sections.get(1).unwrap().split("-").collect::<Vec<&str>>();

        let first_range = std::ops::Range { start: first_section_set.get(0).unwrap().parse::<i32>().unwrap(), end: first_section_set.get(1).unwrap().parse::<i32>().unwrap() + 1 };
        let second_range = std::ops::Range { start: second_section_set.get(0).unwrap().parse::<i32>().unwrap(), end: second_section_set.get(1).unwrap().parse::<i32>().unwrap() + 1 };
        
        if first_range.len() > second_range.len() {
            let all_matching = second_range.into_iter().any(|f| {
                first_range.contains(&f)
            });

            if all_matching {
                acc + 1
            } else {
                acc
            }
        } else {
            let all_matching = first_range.into_iter().any(|f| {
                second_range.contains(&f)
            });

            if all_matching {
                acc + 1
            } else {
                acc
            }
        }        
    });

    println!("Result: {}", result);
}
