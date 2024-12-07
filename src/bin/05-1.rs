use std::{collections::HashSet, io::stdin};

use itertools::Itertools;

struct Input {
    rules: HashSet<(usize, usize)>,
    updates: Vec<Vec<usize>>,
}

impl Input {
    fn parse() -> Self {
        let mut lines = stdin().lines().map(Result::unwrap);
        let rules = lines
            .by_ref()
            .take_while(|x| !x.is_empty())
            .map(|line| {
                let (a, b) = line.split_once("|").unwrap();
                (a.parse().unwrap(), b.parse().unwrap())
            })
            .collect();
        let updates = lines
            .map(|line| line.split(",").map(|page| page.parse().unwrap()).collect())
            .collect();
        Self { rules, updates }
    }

    fn solve(&self) -> usize {
        self.updates
            .iter()
            .filter(|update| {
                update
                    .iter()
                    .tuple_combinations()
                    .all(|(a, b)| !self.rules.contains(&(*b, *a)))
            })
            .map(|update| update[update.len() / 2])
            .sum()
    }
}

fn main() {
    let input = Input::parse();
    let result = input.solve();
    println!("{result}");
}
