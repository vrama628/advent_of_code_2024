use std::io::stdin;

use itertools::Itertools;

fn is_possible(towels: &[&str], design: &str) -> bool {
    let mut memo = vec![false; design.len() + 1];
    memo[design.len()] = true;
    for i in (0..design.len()).rev() {
        memo[i] = towels
            .iter()
            .any(|&towel| design[i..].starts_with(towel) && memo[i + towel.len()])
    }
    memo[0]
}

fn main() {
    let mut lines = stdin().lines().map(Result::unwrap);
    let towels = lines.next().unwrap();
    let towels = towels.split(", ").collect_vec();
    lines.next().unwrap();
    let result = lines.filter(|design| is_possible(&towels, design)).count();
    println!("{result}");
}
