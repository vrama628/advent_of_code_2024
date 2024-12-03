use regex::Regex;
use std::io::{read_to_string, stdin};

fn main() {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let haystack = read_to_string(stdin()).unwrap();
    let result: usize = regex
        .captures_iter(&haystack)
        .map(|capture| capture[1].parse::<usize>().unwrap() * capture[2].parse::<usize>().unwrap())
        .sum();
    println!("{result}")
}
