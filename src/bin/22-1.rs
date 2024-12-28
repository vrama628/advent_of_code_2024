use std::io::stdin;

fn next(mut x: usize) -> usize {
    x ^= x * 64;
    x %= 16777216;
    x ^= x / 32;
    x %= 16777216;
    x ^= x * 2048;
    x %= 16777216;
    x
}

fn next_2000(x: usize) -> usize {
    (0..2000).fold(x, |x, _| next(x))
}

fn main() {
    let result: usize = stdin()
        .lines()
        .map(|line| next_2000(line.unwrap().parse().unwrap()))
        .sum();
    println!("{result}")
}
