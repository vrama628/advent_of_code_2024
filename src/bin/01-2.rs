use itertools::Itertools;
use std::{collections::HashMap, io::stdin};

fn frequencies(list: Vec<usize>) -> HashMap<usize, usize> {
    let mut result: HashMap<usize, usize> = HashMap::new();
    for n in list {
        *result.entry(n).or_insert(0) += 1;
    }
    result
}

fn main() {
    let (list1, list2): (Vec<usize>, Vec<usize>) = stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip();
    let frequencies1 = frequencies(list1);
    let frequencies2 = frequencies(list2);
    let mut result = 0;
    for (k, v) in frequencies1.into_iter() {
        result += v * k * frequencies2.get(&k).unwrap_or(&0);
    }
    println!("{}", result);
}
