use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should have been able to read the file");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut values: Vec<i32> = vec![1];

    for line in lines {
        let parts = line.split(' ').collect::<Vec<&str>>();

        let command = parts.get(0).unwrap();
        if command == &"noop" {
            values.push(values[values.len() - 1]);
            continue;
        } else {
            let amount = parts.get(1).unwrap();
            let amount = amount.parse::<i32>().unwrap();
            // we add the previous instruction another time to symbolize the
            // value is still constant at a cycle
            values.push(values[values.len() - 1]);
            // we add the next value for the next cycle
            values.push(values[values.len() - 1] + amount);
        }
    }

    let mut results = Vec::new();
    for i in [20, 60, 100, 140, 180, 220].iter() {
        results.push(values[i - 1] * *i as i32);
    }

    let answer: i32 = results.iter().sum();
    println!("Result for part 1: {:?}", answer);

    // 40 by 6 resolution
    let pixels: Vec<i32> = (0..240).collect();

    let display = pixels
        .iter()
        .map(|cycle_num| {
            let val = values[*cycle_num as usize];
            (val - (cycle_num % 40)).abs() <= 1
        })
        .map(|one| if one { '#' } else { '.' })
        .collect::<String>();

    println!("Answer pt 2:");
    println!("{}", &display[..40]);
    println!("{}", &display[40..80]);
    println!("{}", &display[80..120]);
    println!("{}", &display[120..160]);
    println!("{}", &display[160..200]);
    println!("{}", &display[200..]);
}
