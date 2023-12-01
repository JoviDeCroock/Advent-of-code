use std::fs;

pub fn part1(input: &str) -> usize {
    solve(input, true)
}

pub fn part2(input: &str) -> usize {
    solve(input, false)
}

fn solve(input: &str, is_abyss: bool) -> usize {
    std::iter::once((parse(input), (500, 0)))
        .fold(0, |_, ((mut solid, bottom), sand)| {
            std::iter::repeat(sand).take_while(|(mut sx, mut sy)| {
                while sy != bottom + 1 {
                    match [(0, 1), (-1, 1), (1, 1)].iter().find(|(dx, dy)|
                        !solid.contains(&(sx + dx, sy + dy))
                    ) {
                        Some((dx, dy)) => {
                            sx += dx;
                            sy += dy;
                        },
                        _ => break,
                    }
                }

                if is_abyss && sy == bottom + 1 {
                    false
                } else {
                    solid.insert((sx, sy))
                }
            }).count()
        })
}

fn parse(input: &str) -> (std::collections::HashSet<(i32, i32)>, i32) {
    input.lines()
        .map(|l| l.split(" -> ")
            .flat_map(|coords| coords.split_once(',')
                .and_then(|(x, y)| Some((
                    x.parse::<i32>().ok()?,
                    y.parse::<i32>().ok()?,
                ))))
            .collect::<Vec<_>>())
        .fold((std::collections::HashSet::new(), 0), |(r, b), p|
            p.windows(2)
                .flat_map(|w| TryInto::<[_; 2]>::try_into(w).ok())
                .fold((r, b), |(mut rocks, mut bottom), [(a, b), (c, d)]| {
                    for y in b.min(d)..=b.max(d) {
                        for x in a.min(c)..=a.max(c) {
                            rocks.extend(std::iter::once((x, y)))
                        }

                        bottom = bottom.max(y)
                    }

                    (rocks, bottom)
                }))
}

pub fn main() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");
    println!("{}", part1(contents.as_str()));
    println!("{}", part2(contents.as_str()));
}
