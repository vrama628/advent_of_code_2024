use std::{
    collections::HashMap,
    io::{read_to_string, stdin},
};

use itertools::Itertools;

type Stone = usize;
type Stones = Vec<Stone>;

fn change(stone: usize) -> Stones {
    if stone == 0 {
        vec![1]
    } else {
        let n_digits = stone.ilog10() + 1;
        if n_digits % 2 == 0 {
            let half = 10usize.pow(n_digits / 2);
            vec![stone / half, stone % half]
        } else {
            vec![stone * 2024]
        }
    }
}

#[derive(Default)]
struct Memo(HashMap<usize, Vec<usize>>);

impl Memo {
    fn get(&mut self, stone: Stone) -> &mut Vec<usize> {
        self.0.entry(stone).or_insert_with(|| vec![1])
    }

    /// postcondition: self.get(stone).len() == steps
    fn compute(&mut self, stone: Stone, steps: usize) {
        if self.get(stone).len() <= steps {
            let nexts = change(stone);
            for &next in &nexts {
                self.compute(next, steps - 1)
            }
            for i in self.get(stone).len()..=steps {
                let res = nexts.iter().map(|&next| self.get(next)[i - 1]).sum();
                self.get(stone).push(res)
            }
        }
    }
}

fn main() {
    let stones = read_to_string(stdin())
        .unwrap()
        .split_ascii_whitespace()
        .map(|stone| stone.parse::<usize>().unwrap())
        .collect_vec();
    let mut memo = Memo::default();
    let mut result = 0;
    for &stone in &stones {
        memo.compute(stone, 75);
        result += memo.get(stone)[75];
    }
    println!("{result}");
}
