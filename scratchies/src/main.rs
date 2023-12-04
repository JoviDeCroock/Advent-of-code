fn main() {
    let input = include_str!("../input.txt").lines();

    let mut total = 0;
    for line in input {
        let (winning, actual) = line.split_once('|').unwrap();
        let score = get_score(winning, actual);
        total += score;
    }

    println!("P1: {}", total);

    let input = include_str!("../input.txt").lines().collect::<Vec<&str>>();

    let mut card_counts = vec![1; input.len()];
    for (i, line) in input.iter().enumerate() {
        let (winning, actual) = line.split_once('|').unwrap();

        let matches = get_matches(winning, actual);
        let start = i + 1;
        for j in start.. (start + matches as usize) {
            card_counts[j] += card_counts[i] 
        }
    }

    println!("P2: {}", card_counts.iter().sum::<i32>());
}

fn get_matches(winning: &str, actual: &str) -> i32 {
    let winning_numbers = winning.trim().split(' ').collect::<Vec<&str>>();
    actual.trim().split(' ').fold(0, |acc, actual_number| {
        let parsed = actual_number.parse::<i32>();
        if parsed.is_ok() && winning_numbers.contains(&actual_number) {
            acc + 1
        } else {
            acc
        }
    })
}

fn get_score(winning: &str, actual: &str) -> i32 {
    let winning_numbers = winning.trim().split(' ').collect::<Vec<&str>>();
    actual.trim().split(' ').fold(0, |acc, actual_number| {
        let parsed = actual_number.parse::<i32>();
        if parsed.is_ok() && winning_numbers.contains(&actual_number) {
            if acc == 0 {
                acc + 1
            } else {
                acc * 2
            }
        } else {
            acc
        }
    })
}
