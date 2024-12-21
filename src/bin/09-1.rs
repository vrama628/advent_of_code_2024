use std::{
    io::{read_to_string, stdin},
    iter::repeat_n,
};

use itertools::Itertools;

fn main() {
    let mut disk = read_to_string(stdin())
        .unwrap()
        .trim_ascii_end()
        .chars()
        .map(|c| (c as usize) - ('0' as usize))
        .chunks(2)
        .into_iter()
        .enumerate()
        .flat_map(|(i, chunk)| {
            let (file, free) = match chunk.exactly_one() {
                Ok(file) => (file, 0),
                Err(chunk) => chunk.collect_tuple().unwrap(),
            };
            repeat_n(Some(i), file).chain(repeat_n(None, free))
        })
        .collect_vec();
    debug_assert!(disk[disk.len() - 1].is_some());
    let mut i = 0;
    while i < disk.len() {
        while disk.ends_with(&[None]) {
            disk.pop();
        }
        if disk[i].is_none() {
            disk.swap_remove(i);
        }
        i += 1;
    }
    let result: usize = disk
        .into_iter()
        .map(Option::unwrap)
        .enumerate()
        .map(|(i, f)| i * f)
        .sum();
    println!("{result}");
}
