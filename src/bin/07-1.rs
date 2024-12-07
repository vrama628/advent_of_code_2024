use std::io::stdin;

fn is_possible_tail(test_value: usize, acc: usize, numbers: &[usize]) -> bool {
    if let [head, tail @ ..] = numbers {
        is_possible_tail(test_value, acc + head, tail)
            || is_possible_tail(test_value, acc * head, tail)
    } else {
        acc == test_value
    }
}

fn is_possible(test_value: usize, numbers: Vec<usize>) -> bool {
    is_possible_tail(test_value, numbers[0], &numbers[1..])
}

fn main() {
    let mut result = 0;
    for line in stdin().lines().map(Result::unwrap) {
        let (test_value, numbers) = line.split_once(": ").unwrap();
        let test_value = test_value.parse().unwrap();
        let numbers = numbers.split(" ").map(|n| n.parse().unwrap()).collect();
        if is_possible(test_value, numbers) {
            result += test_value;
        }
    }
    println!("{result}");
}
