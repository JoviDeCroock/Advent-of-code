fn main() {
    let input = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    let raw_times = input.get(0).unwrap().split(":").collect::<Vec<&str>>();
    let raw_distances = input.get(1).unwrap().split(":").collect::<Vec<&str>>();

    let times = raw_times.get(1).unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let distances = raw_distances.get(1).unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut track_record = vec![];
    for i in 0..times.len() {
        let time = times.get(i).unwrap();
        let recorded_distance = distances.get(i).unwrap();
        let mut count = 0;
        for j in 1..*time {
            let time_left = *time - j;
            let distance_covered = j * time_left;
            if distance_covered > *recorded_distance {
                count += 1;
            }
        }
        track_record.push(count);
    }

    println!("Part 1: {:?}", track_record.iter().fold(1, |acc, x| acc * x));

    let time = raw_times.get(1).unwrap().split_whitespace().collect::<Vec<&str>>().join("").parse::<i64>().unwrap();
    let recorded_distance = raw_distances.get(1).unwrap().split_whitespace().collect::<Vec<&str>>().join("").parse::<i64>().unwrap();
    let mut count = 0;
    for j in 1..time {
        let time_left = time - j;
        let distance_covered = j * time_left;
        if distance_covered > recorded_distance {
            count += 1;
        }
    }

    println!("Part 2: {:?}", count);
}
