fn update_knot(leader: (isize, isize), follower: &mut (isize, isize)) {
    let (x_dist, y_dist) = (leader.0 - follower.0, leader.1 - follower.1);
    if x_dist.abs() > 1 || y_dist.abs() > 1 {
        follower.0 += 1 * x_dist.signum();
        follower.1 += 1 * y_dist.signum();
    }
}

fn run_with_n_knots(lines: std::str::Lines, num_knots: usize) -> usize {
    let mut knot_positions: Vec<(isize, isize)> = Vec::with_capacity(num_knots);
    knot_positions.resize(num_knots, (0, 0));
    let mut tail_positions: std::collections::HashSet<(isize, isize)> = 
std::collections::HashSet::new();

    for line in lines {
        let (dir, step) = line.split_once(' ').unwrap();
        let step = step.parse::<u32>().unwrap();
        for _ in 0..step {
            match dir {
                "U" => knot_positions[0].1 += 1,
                "D" => knot_positions[0].1 -= 1,
                "R" => knot_positions[0].0 += 1,
                "L" => knot_positions[0].0 -= 1,
                _ => unreachable!(),
            }
            for l in 0..num_knots - 1 {
                update_knot(knot_positions[l as usize], &mut knot_positions[l + 1 as usize]);
            }
            tail_positions.insert(knot_positions[num_knots - 1]);
            
        }           
    }
    tail_positions.len()
}

fn main() {
    let now = std::time::Instant::now();
    let input = std::fs::read_to_string("./src/input.txt").unwrap();
    println!("Part 1: {}, Part 2: {}, took {:#?}", 
    run_with_n_knots(input.lines(), 2), run_with_n_knots(input.lines(), 10), now.elapsed());
}