use std::{io::stdin, ops::Add};

use im::OrdMap;
use itertools::Itertools;

fn next(mut x: usize) -> usize {
    x ^= x * 64;
    x %= 16777216;
    x ^= x / 32;
    x %= 16777216;
    x ^= x * 2048;
    x %= 16777216;
    x
}

#[derive(Clone)]
struct Prices(usize);

impl Iterator for Prices {
    type Item = i8;

    fn next(&mut self) -> Option<Self::Item> {
        let res = (self.0 % 10).try_into().unwrap();
        self.0 = next(self.0);
        Some(res)
    }
}

type Changes = (i8, i8, i8, i8);

fn changes(x: usize) -> OrdMap<Changes, usize> {
    let prices = Prices(x);
    let mut result = OrdMap::new();
    for (changes, price) in prices
        .clone()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .take(2000)
        .tuple_windows::<Changes>()
        .zip(prices.skip(4).map(|p| usize::try_from(p).unwrap()))
    {
        if !result.contains_key(&changes) {
            result.insert(changes, price);
        }
    }
    result
}

fn main() {
    let totals = OrdMap::unions_with(
        stdin()
            .lines()
            .map(|line| changes(line.unwrap().parse().unwrap())),
        Add::add,
    );
    let result = totals.values().max().unwrap();
    println!("{result}");
}
