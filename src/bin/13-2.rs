use std::io::{self, stdin, Stdin};

use once_cell::sync::Lazy;
use regex::Regex;

struct Button {
    x: i64,
    y: i64,
}

impl Button {
    fn parse(stdin: &Stdin) -> io::Result<Self> {
        static REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap());
        let mut buf = String::new();
        stdin.read_line(&mut buf)?;
        let captures = REGEX.captures(&buf).unwrap();
        let x = captures[1].parse().unwrap();
        let y = captures[2].parse().unwrap();
        Ok(Self { x, y })
    }
}

struct Claw {
    a: Button,
    b: Button,
    x: i64,
    y: i64,
}

impl Claw {
    fn parse(stdin: &Stdin) -> io::Result<Self> {
        static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap());
        let a = Button::parse(stdin)?;
        let b = Button::parse(stdin)?;
        let mut buf = String::new();
        stdin.read_line(&mut buf)?;
        let captures = REGEX.captures(&buf).unwrap();
        let x = captures[1].parse::<i64>().unwrap() + 10000000000000;
        let y = captures[2].parse::<i64>().unwrap() + 10000000000000;
        Ok(Self { a, b, x, y })
    }

    fn solve(&self) -> Option<i64> {
        let b_numerator = self.y * self.a.x - self.a.y * self.x;
        let b_denominator = self.b.y * self.a.x - self.a.y * self.b.x;
        (b_numerator % b_denominator == 0).then_some(())?;
        let b = b_numerator / b_denominator;
        let a_numerator = self.x - self.b.x * b;
        let a_denominator = self.a.x;
        (a_numerator % a_denominator == 0).then_some(())?;
        let a = a_numerator / a_denominator;
        Some(3 * a + b)
    }
}

fn main() {
    let stdin = stdin();
    let mut result = 0;
    loop {
        if let Some(tokens) = Claw::parse(&stdin).unwrap().solve() {
            result += tokens;
        }
        if stdin.read_line(&mut String::new()).unwrap() == 0 {
            break;
        }
    }
    println!("{result}");
}
