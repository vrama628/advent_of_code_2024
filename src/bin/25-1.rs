use std::io::stdin;

use itertools::Itertools;

#[derive(Debug)]
struct Door {
    keys: Vec<[u8; 5]>,
    locks: Vec<[u8; 5]>,
}

const EMPTY: [u8; 5] = [0, 0, 0, 0, 0];
const HEIGHT: u8 = 6;

impl Door {
    fn parse() -> Self {
        let mut keys = vec![];
        let mut locks = vec![];
        let mut current = EMPTY.clone();
        let mut i = 0;
        let mut is_key = true;
        for line in stdin().lines().map(Result::unwrap) {
            if i == 0 {
                is_key = match line.as_str() {
                    "....." => true,
                    "#####" => false,
                    _ => panic!(),
                };
            } else if i == HEIGHT {
                if is_key {
                    debug_assert_eq!(line, "#####")
                } else {
                    debug_assert_eq!(line, ".....")
                }
                if is_key {
                    keys.push(current);
                } else {
                    locks.push(current);
                }
                current = EMPTY.clone();
            } else if line.is_empty() {
                i = 0;
                continue;
            } else {
                for (i, c) in line.char_indices() {
                    if c == '#' {
                        current[i] += 1;
                    }
                }
            }
            i += 1;
        }
        Self { keys, locks }
    }

    fn solve(self) -> usize {
        let Self { keys, locks } = self;
        keys.into_iter()
            .cartesian_product(locks)
            .filter(|([k1, k2, k3, k4, k5], [l1, l2, l3, l4, l5])| {
                k1 + l1 < HEIGHT
                    && k2 + l2 < HEIGHT
                    && k3 + l3 < HEIGHT
                    && k4 + l4 < HEIGHT
                    && k5 + l5 < HEIGHT
            })
            .count()
    }
}

fn main() {
    let door = Door::parse();
    let result = door.solve();
    println!("{result}");
}
