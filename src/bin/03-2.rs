use regex::Regex;
use std::io::{read_to_string, stdin};

fn main() {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let haystack = read_to_string(stdin()).unwrap();
    let mut r#do = true;
    let mut result = 0;
    for capture in regex.captures_iter(&haystack) {
        match &capture[0] {
            "do()" => r#do = true,
            "don't()" => r#do = false,
            _ => {
                if r#do {
                    result +=
                        capture[1].parse::<usize>().unwrap() * capture[2].parse::<usize>().unwrap()
                }
            }
        }
    }
    println!("{result}")
}
