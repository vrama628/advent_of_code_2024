use itertools::Itertools;
use std::io::stdin;

fn is_safe_increase((a, b): (&usize, &usize)) -> bool {
    b.checked_sub(*a).is_some_and(|diff| 1 <= diff && diff <= 3)
}

fn is_safe_decrease((a, b): (&usize, &usize)) -> bool {
    is_safe_increase((b, a))
}

fn is_safe(report: &[usize]) -> bool {
    report.iter().tuple_windows().all(is_safe_increase)
        || report.iter().tuple_windows().all(is_safe_decrease)
}

fn without(report: &Vec<usize>, i: usize) -> Vec<usize> {
    let mut clone = report.clone();
    clone.remove(i);
    clone
}

fn is_safe_dampened(report: &Vec<usize>) -> bool {
    is_safe(report) || (0..report.len()).any(|i| is_safe(&without(report, i)))
}

fn main() {
    let reports = stdin().lines().map(|line| {
        line.unwrap()
            .split_whitespace()
            .map(|level| level.parse::<usize>().unwrap())
            .collect_vec()
    });
    let result = reports.filter(is_safe_dampened).count();
    println!("{result}")
}
