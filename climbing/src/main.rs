use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

struct HeightMap {
    data: Vec<u8>,
    width: usize,
    lowest_points: Vec<(usize, usize)>,
}
impl HeightMap {
    fn parse(str: &str) -> (Self, (usize, usize), (usize, usize)) {
        let mut data = Vec::with_capacity(str.len());
        let mut width = 0;

        let mut start = (0, 0);
        let mut goal = (0, 0);
        let mut lowest_points = Vec::new();

        for (y, line) in str.lines().enumerate() {
            width = line.len();

            for (x, mut byte) in line.bytes().enumerate() {
                if byte == b'S' {
                    start = (x, y);
                    byte = b'a';
                }

                if byte == b'E' {
                    goal = (x, y);
                    byte = b'z';
                }

                if byte == b'a' {
                    lowest_points.push((x, y));
                }

                data.push(byte - b'a');
            }
        }

        (
            Self {
                data,
                width,
                lowest_points,
            },
            start,
            goal,
        )
    }

    fn height(&self) -> usize {
        self.data.len() / self.width
    }

    fn val_at(&self, x: usize, y: usize) -> Option<u8> {
        if x > self.width {
            return None;
        }
        if y > self.height() {
            return None;
        }

        let idx = y * self.width + x;
        self.data.get(idx).copied()
    }

    fn traversable(&self, cur_height: u8, dest_x: usize, dest_y: usize) -> bool {
        if let Some(dest_height) = self.val_at(dest_x, dest_y) {
            dest_height <= cur_height || dest_height == (cur_height + 1)
        } else {
            false
        }
    }

    fn possible_moves(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let cur_height = self.val_at(x, y);
        if cur_height.is_none() {
            return vec![];
        }

        let cur_height = cur_height.unwrap();

        let mut result = Vec::new();

        // up
        if self.traversable(cur_height, x, y.checked_add(1).unwrap_or(usize::MAX)) {
            result.push((x, y + 1));
        }

        // down
        if self.traversable(cur_height, x, y.checked_sub(1).unwrap_or(usize::MAX)) {
            result.push((x, y - 1));
        }

        // left
        if self.traversable(cur_height, x.checked_sub(1).unwrap_or(usize::MAX), y) {
            result.push((x - 1, y));
        }

        // right
        if self.traversable(cur_height, x.checked_add(1).unwrap_or(usize::MAX), y) {
            result.push((x + 1, y));
        }

        result
    }
}

#[derive(Debug, Eq)]
struct Visit {
    pos: (usize, usize),
    distance: usize,
}
impl PartialEq for Visit {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}
impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Visit {
    fn cmp(&self, other: &Self) -> Ordering {
        // reversed so that the max-heap std::collections::BinaryHeap will act as a min-heap
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

fn len_of_shortest_path(start: (usize, usize), goal: (usize, usize), map: &HeightMap) -> usize {
    let mut queue = BinaryHeap::new();
    let mut distances = HashMap::new();
    let mut visited = HashSet::new();

    for y in 0..map.height() {
        for x in 0..map.width {
            distances.insert((x, y), usize::MAX);
        }
    }
    distances.insert(start, 0);
    queue.push(Visit {
        pos: start,
        distance: 0,
    });

    while let Some(Visit { pos, distance }) = queue.pop() {
        if !visited.insert(pos) {
            continue;
        }

        for neighbor in map.possible_moves(pos.0, pos.1) {
            let new_distance = distance + 1;
            if distances.get(&neighbor).copied().unwrap_or(usize::MAX) > new_distance {
                distances.insert(neighbor, new_distance);
                queue.push(Visit {
                    pos: neighbor,
                    distance: new_distance,
                });
            }
        }
    }

    // for y in 0..map.height() {
    //     let mut row = String::new();
    //     for x in 0..map.width {
    //         row.push_str(&format!("{:03} ", distances.get(&(x, y)).unwrap_or(&999)));
    //     }
    //     println!("{}", row);
    // }

    *distances.get(&goal).unwrap()
}

fn main() {
    let input = std::fs::read_to_string("./src/input.txt").unwrap();

    let (map, start, goal) = HeightMap::parse(&input);
    println!("{}", len_of_shortest_path(start, goal, &map));

    let possible_starts = map.lowest_points.clone();
    let shortest_possible_climb = possible_starts
        .into_iter()
        .map(|point| len_of_shortest_path(point, goal, &map))
        .min()
        .unwrap();
    println!("{}", shortest_possible_climb);
}
