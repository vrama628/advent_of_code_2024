use std::{
    collections::{HashMap, HashSet},
    io::stdin,
};

use itertools::Itertools;

struct Input {
    rules: HashMap<usize, HashSet<usize>>,
    updates: Vec<Vec<usize>>,
}

impl Input {
    fn parse() -> Self {
        let mut lines = stdin().lines().map(Result::unwrap);
        let mut rules: HashMap<usize, HashSet<usize>> = HashMap::new();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let (a, b) = line.split_once("|").unwrap();
            let (a, b) = (a.parse().unwrap(), b.parse().unwrap());
            rules.entry(a).or_default().insert(b);
        }
        let updates = lines
            .map(|line| line.split(",").map(|page| page.parse().unwrap()).collect())
            .collect();
        Self { rules, updates }
    }

    fn dfs(&self, update: &[usize], sorted: &mut Vec<usize>, page: usize) {
        if sorted.contains(&page) {
            return;
        }
        for &neighbor in update.iter().filter(|&neighbor| {
            self.rules
                .get(&page)
                .is_some_and(|adj| adj.contains(neighbor))
        }) {
            self.dfs(update, sorted, neighbor)
        }
        sorted.push(page)
    }

    fn sort(&self, update: &[usize]) -> usize {
        let mut sorted = Vec::with_capacity(update.len());
        for &page in update {
            self.dfs(update, &mut sorted, page);
        }
        sorted[sorted.len() / 2]
    }

    fn solve(&self) -> usize {
        self.updates
            .iter()
            .filter(|update| {
                update
                    .iter()
                    .tuple_combinations()
                    .any(|(a, b)| self.rules.get(b).is_some_and(|adj| adj.contains(a)))
            })
            .map(|update| self.sort(update))
            .sum()
    }
}

fn main() {
    let input = Input::parse();
    let result = input.solve();
    println!("{result}");
}
