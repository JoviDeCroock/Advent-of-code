fn get_absolute_distance(p1: &(i32, i32), p2: &(i32, i32)) -> i32 {
    let x_diff = p1.0 - p2.0;
    let y_diff = p1.1 - p2.1;
    x_diff.abs() + y_diff.abs()
}

fn main() {
    let target_y = 2000000;
    let input = std::fs::read_to_string("./src/input.txt").expect("Why no file?");

    let mut grid: Vec<((i32, i32), (i32, i32))> = vec![];
    // Example line: Sensor at x=3890859, y=2762958: closest beacon is at x=4037927, y=2985317
    input.lines().for_each(|line| {
        let line = line.replace("Sensor at ", "")
            .replace(" closest beacon is at ", "")
            .replace(" ", "")
            .replace(":", ",")
            .replace("x=", "")
            .replace("y=", "");

        let mut ps = line.split(',').map(|x| x.parse::<i32>().unwrap());
        grid.push((
            (ps.next().unwrap(), ps.next().unwrap()),
            (ps.next().unwrap(), ps.next().unwrap()),
        ))
    });

    let mut free_points: Vec<(i32, i32)> = Vec::new();
    for (sensor, beacon) in grid.clone() {
        let b_s_dst = get_absolute_distance(&sensor, &beacon);
        let dy = sensor.1 - target_y;
        let dy = dy.abs();

        if dy <= b_s_dst {
            let dx = (b_s_dst - dy) as i32;
            free_points.push((sensor.0 - dx, sensor.0 + dx));
        }
    }

    free_points.sort_unstable();

    let mut combined_ranges: Vec<(i32, i32)> = vec![free_points[0]];

    for r in free_points.iter().skip(1) {
        let to_check = combined_ranges.last().unwrap();

        if r.0 > to_check.1 {
            combined_ranges.push(*r);
        } else {
            *combined_ranges.last_mut().unwrap() =
                (to_check.0, *[r.1, to_check.1].iter().max().unwrap());
        }
    }

    let mut total: u32 = 0;
    for r in combined_ranges {
        total += (r.1 - r.0) as u32;
    }

    println!("{}", total);

    for y in 0..4000000 {
        let mut free_xs: Vec<(i32, i32)> = Vec::new();

        for (sensor, beacon) in &grid {
            let b_s_dst = get_absolute_distance(sensor, beacon);
            let dy = sensor.1 - y;
            let dy = dy.abs();

            if dy < b_s_dst {
                let dx = (b_s_dst - dy) as i32;
                free_xs.push((
                    *[sensor.0 - dx, 0].iter().max().unwrap(),
                    *[sensor.0 + dx, 4000000].iter().min().unwrap(),
                ));
            }
        }

        free_xs.sort_unstable();
        let mut combined_ranges: Vec<(i32, i32)> = vec![free_xs[0]];

        for r in free_xs.iter().skip(1) {
            let to_check = combined_ranges.last().unwrap();

            if r.0 > to_check.1 {
                combined_ranges.push(*r);
            } else {
                *combined_ranges.last_mut().unwrap() =
                    (to_check.0, *[r.1, to_check.1].iter().max().unwrap());
            }
        }

        if combined_ranges.len() > 1 {
            println!(
                "{}",
                4000000u64 * (combined_ranges.first().unwrap().1 + 1) as u64 + y as u64
            );
        }
    }
}