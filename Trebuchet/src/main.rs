fn main() {
    let input = include_str!("../input.txt");
    let part_1 = part_1(input);
    println!("Part 1: {}", part_1);
    let part_2 = part_2(input);
    println!("Part 2: {}", part_2);
}

fn part_1(input: &str) -> i32 {
    input.split('\n').into_iter().fold(0, |acc, line| {
        let numbers = line.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>();
        if numbers.len() == 1 {
            let combined = format!("{}{}", numbers.get(0).unwrap(), numbers.get(0).unwrap()).parse::<i32>().unwrap();
            return acc + combined;
        } else if numbers.len() > 1 {
            let combined = format!("{}{}", numbers.get(0).unwrap(), numbers.get(numbers.len() - 1).unwrap()).parse::<i32>().unwrap();
            return acc + combined;
        } else {
            return acc;
        }
    })
}

fn part_2(text: &str) -> i32 {
    text.split('\n').into_iter().fold(0, |acc, line| {
        let line = line
            .replacen("one", "on1e", 100)
            .replacen("two", "tw2o", 100)
            .replacen("three", "thr3ee", 100)
            .replacen("four", "fo4ur", 100)
            .replacen("five", "fi5ve", 100)
            .replacen("six", "si6x", 100)
            .replacen("seven", "sev7en", 100)
            .replacen("eight", "eig8th", 100)
            .replacen("nine", "ni9ne", 100);
        let numbers = line.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>();
        if numbers.len() == 1 {
            let combined = format!("{}{}", numbers.get(0).unwrap(), numbers.get(0).unwrap()).parse::<i32>().unwrap();
            return acc + combined;
        } else if numbers.len() > 1 {
            let combined = format!("{}{}", numbers.get(0).unwrap(), numbers.get(numbers.len() - 1).unwrap()).parse::<i32>().unwrap();
            return acc + combined;
        } else {
            return acc;
        }
    })
}
