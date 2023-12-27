fn main() {
    let input = include_str!("../input.txt").lines().collect::<Vec<&str>>();

    let mut appends = vec![];
    let mut prepends = vec![];
    for line in input {
        let new_numbers = get_differences(line);
        let next_number = append_sequence(&new_numbers);
        appends.push(next_number);
        let next_number = prepend_sequence(&new_numbers);
        prepends.push(next_number);
    }

    println!("Part 1 {:?}", appends.iter().sum::<i32>());
    println!("Part 2 {:?}", prepends.iter().sum::<i32>());
}

fn get_differences(line: &str) -> Vec<Vec<i32>> {
    let numbers = line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut differences = vec![numbers];

    while differences.last().unwrap().iter().any(|x| x != &0) {
        differences.push(get_differences_from_numbers(differences.last().unwrap()));
    }

    differences
}

fn get_differences_from_numbers(numbers: &[i32]) -> Vec<i32> {
    let mut new_numbers = vec![];
    for i in 0..numbers.len() - 1 {
        new_numbers.push(numbers[i + 1] - numbers[i]);
    }
    new_numbers
}

fn append_sequence(sequences: &Vec<Vec<i32>>) -> i32 {
    let mut reversed_sequences = sequences.clone();
    reversed_sequences.reverse();
    let mut last_number = 0;
    for (i, _sequence) in reversed_sequences.iter().enumerate() {
        if i == 0 {
            last_number = 0;
        } else {
            let current_sequence = reversed_sequences.get(i).unwrap();
            let last_sequence_number = current_sequence.last().unwrap();
            last_number = last_number + *last_sequence_number;
        }
    };

    last_number.to_owned()
}

fn prepend_sequence(sequences: &Vec<Vec<i32>>) -> i32 {
    let mut reversed_sequences = sequences.clone();
    reversed_sequences.reverse();
    let mut first_number = 0;
    for (i, _sequence) in reversed_sequences.iter().enumerate() {
        if i == 0 {
            first_number = 0;
        } else {
            let current_sequence = reversed_sequences.get(i).unwrap();
            let first_sequence_member = current_sequence.first().unwrap();
            first_number =  *first_sequence_member - first_number;
        }
    };

    first_number.to_owned()
}

