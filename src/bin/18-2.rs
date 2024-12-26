use std::{
    collections::{BTreeSet, VecDeque},
    io::stdin,
};

use itertools::Itertools;

const SIZE: usize = 71;

type Point = (usize, usize);

fn adjacent(&(x, y): &Point) -> [Point; 4] {
    [
        (x.wrapping_sub(1), y),
        (x + 1, y),
        (x, y.wrapping_sub(1)),
        (x, y + 1),
    ]
}

fn within(&(x, y): &Point) -> bool {
    x < SIZE && y < SIZE
}

const START: Point = (0, 0);
const END: Point = (SIZE - 1, SIZE - 1);

fn reachable(blocked: &BTreeSet<Point>) -> bool {
    let mut queue = VecDeque::from([START]);
    let mut visited = BTreeSet::from([START]);
    while let Some(point) = queue.pop_front() {
        if point == END {
            return true;
        }
        for adj in adjacent(&point) {
            if within(&adj) && !blocked.contains(&adj) && visited.insert(adj.clone()) {
                queue.push_back(adj);
            }
        }
    }
    false
}

fn main() {
    let blocks = stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<Point>>();

    let mut lo = 0;
    let mut hi = blocks.len();
    while lo != hi {
        let mid = (lo + hi) >> 1;
        if reachable(&blocks[..mid].iter().cloned().collect()) {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    let (x, y) = blocks[lo - 1];
    println!("{x},{y}");
}
