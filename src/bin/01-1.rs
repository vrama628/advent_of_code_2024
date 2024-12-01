use itertools::Itertools;
use std::io::stdin;

fn main() {
    let (mut list1, mut list2): (Vec<usize>, Vec<usize>) = stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip();
    list1.sort();
    list2.sort();
    let result: usize = list1
        .into_iter()
        .zip(list2)
        .map(|(a, b)| a.abs_diff(b))
        .sum();
    println!("{result}");
}
