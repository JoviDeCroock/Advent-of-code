use std::{fs};

fn main() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    let characters = contents.chars().collect::<Vec<char>>();
    let length = characters.len() - 4;

    for i in 0..length {
        let mut found_token = true;

        // We go over elements 1-4
        for j in 0..3 {
            // We want to compare them to the ones behind them in
            // our range of 4
            for k in (j + 1)..4 {
                // When one doesn't match we bail
                if characters[i + j] == characters[i + k] {
                    found_token = false;
                    break;
                }
            }

            if !found_token {
                break;
            }
        }

        if found_token {
            println!("Found {}", i + 4);
            break;
        }
    }

    let length = characters.len() - 14;
    for i in 0..length {
        let mut found_token = true;
        for j in 0..13 {
            for k in (j + 1)..14 {
                if characters[i + j] == characters[i + k] {
                    found_token = false;
                    break;
                }
            }

            if !found_token {
                break;
            }
        }

        if found_token {
            println!("Found {}", i + 14);
            break;
        }
    }
}
