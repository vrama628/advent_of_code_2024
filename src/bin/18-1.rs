use std::{
    collections::{BTreeSet, VecDeque},
    io::stdin,
};

const SIZE: usize = 71;
const KILO: usize = 1024;

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

fn main() {
    let mut memory = vec![vec![true; SIZE]; SIZE];
    for line in stdin().lines().map(Result::unwrap).take(KILO) {
        let (x, y) = line.split_once(",").unwrap();
        let (x, y) = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());
        memory[y][x] = false;
    }

    let start = (0, 0);
    let end = (SIZE - 1, SIZE - 1);

    let mut queue = VecDeque::from([(start.clone(), 0)]);
    let mut visited = BTreeSet::from([start]);
    while let Some((point, steps)) = queue.pop_front() {
        if point == end {
            println!("{steps}");
            break;
        }

        for (x, y) in adjacent(&point) {
            if within(&(x, y)) && memory[y][x] && visited.insert((x, y)) {
                queue.push_back(((x, y), steps + 1));
            }
        }
    }
}
