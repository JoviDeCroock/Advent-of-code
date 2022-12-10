use std::fs;

pub fn main() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should have been able to read the file");
    let lines = contents.lines().collect::<Vec<_>>();
    let mut count = 0;
    let mut max_score = 0;

    for r in 1..lines.len() - 1 {
        let row = lines[r];
        for c in 1..row.len() - 1 {
            let height = row[c..=c].parse().unwrap();

            let mut is_visible = 4;
            let mut score = 1;

            score *= if let Some(sc_score) = (0..c)
                .rev()
                .map(|cc| (c - cc, row[cc..=cc].parse::<usize>().unwrap()))
                .find(|(_sc_score, other_height)| other_height >= &height)
            {
                is_visible -= 1;
                sc_score.0
            } else {
                c
            };

            score *= if let Some(sc_score) = (c + 1..row.len())
                .map(|cc| (cc - c, row[cc..=cc].parse::<usize>().unwrap()))
                .find(|(_sc_score, other_height)| other_height >= &height)
            {
                is_visible -= 1;
                sc_score.0
            } else {
                row.len() - c - 1
            };

            score *= if let Some(sc_score) = (0..r)
                .rev()
                .map(|rr| (r - rr, lines[rr][c..=c].parse::<usize>().unwrap()))
                .find(|(_sc_score, other_height)| other_height >= &height)
            {
                is_visible -= 1;
                sc_score.0
            } else {
                r
            };

            score *= if let Some(sc_score) = lines
                .iter()
                .enumerate()
                .skip(r + 1)
                .map(|(rr, l)| (rr - r, l[c..=c].parse::<usize>().unwrap()))
                .find(|(_sc_score, other_height)| other_height >= &height)
            {
                is_visible -= 1;
                sc_score.0
            } else {
                lines.len() - r - 1
            };

            max_score = max_score.max(score);

            if is_visible > 0 {
                count += 1;
            }
        }
    }

    count += 2 * lines.len() + 2 * (lines[0].len() - 2);

    println!("Day 8, Part 01: {}", count);
    println!("Day 8, Part 02: {}", max_score);
}
