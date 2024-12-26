use std::io::stdin;

use itertools::Itertools;

fn ways(towels: &[&str], design: &str) -> usize {
    let mut memo = vec![0; design.len() + 1];
    memo[design.len()] = 1;
    for i in (0..design.len()).rev() {
        memo[i] = towels
            .iter()
            .map(|&towel| {
                design[i..]
                    .starts_with(towel)
                    .then(|| memo[i + towel.len()])
                    .unwrap_or_default()
            })
            .sum();
    }
    memo[0]
}

fn main() {
    let mut lines = stdin().lines().map(Result::unwrap);
    let towels = lines.next().unwrap();
    let towels = towels.split(", ").collect_vec();
    lines.next().unwrap();
    let result = lines.map(|design| ways(&towels, &design)).sum::<usize>();
    println!("{result}");
}
