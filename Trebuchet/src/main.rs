fn main() {
    let input = include_str!("../input.txt");
    let sum = input.split('\n').into_iter().fold(0, |acc, line| {
        // I need to find the first and last numerical characters and combine them into a single number
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
    });

    println!("Sum: {}", sum)
}
