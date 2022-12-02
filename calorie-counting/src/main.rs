use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    let individual_calories_by_elf = contents.split("\n\n").collect::<Vec<&str>>();
    let mut total_calories_by_elf = individual_calories_by_elf.iter().map(|element| {
        element.split("\n").map(|s| s.parse().unwrap()).collect::<Vec<i32>>().iter().fold(0,|a, &b| a + b)
    }).collect::<Vec<i32>>();

    total_calories_by_elf.sort_by(|a, b| b.cmp(a));
    // let highest_calories = total_calories_by_elf.iter().max_by_key(|f| f.abs()).unwrap();
    println!("Highest calories carried by an elf: {:?}", total_calories_by_elf.get(0).unwrap());
    println!("Total of top three elves {:?}", total_calories_by_elf.get(0).unwrap() + total_calories_by_elf.get(1).unwrap() + total_calories_by_elf.get(2).unwrap());
}
