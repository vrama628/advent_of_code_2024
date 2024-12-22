use std::io::{read_to_string, stdin};

fn change(stone: usize) -> Vec<usize> {
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

fn blink(stones: Vec<usize>) -> Vec<usize> {
    stones.into_iter().flat_map(change).collect()
}

fn main() {
    let stones = read_to_string(stdin())
        .unwrap()
        .split_ascii_whitespace()
        .map(|stone| stone.parse::<usize>().unwrap())
        .collect();
    let result = (0..25).fold(stones, |stones, _| blink(stones)).len();
    println!("{result}");
}
