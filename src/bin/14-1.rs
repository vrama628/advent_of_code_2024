use std::{cmp::Ordering, collections::HashMap, io::stdin, num::ParseIntError, str::FromStr};

use once_cell::sync::Lazy;
use regex::Regex;

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

struct Robot {
    px: i64,
    py: i64,
    vx: i64,
    vy: i64,
}

impl FromStr for Robot {
    type Err = ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        static REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap());
        let captures = REGEX.captures(&line).unwrap();
        let px = captures[1].parse::<i64>()?;
        let py = captures[2].parse::<i64>()?;
        let vx = captures[3].parse::<i64>()?;
        let vy = captures[4].parse::<i64>()?;
        Ok(Self { px, py, vx, vy })
    }
}

type Quadrant = (Ordering, Ordering);

impl Robot {
    fn quadrant_after(&self, seconds: i64) -> Quadrant {
        let x = (self.px + self.vx * seconds).rem_euclid(WIDTH);
        let y = (self.py + self.vy * seconds).rem_euclid(HEIGHT);
        (x.cmp(&(WIDTH / 2)), y.cmp(&(HEIGHT / 2)))
    }
}

fn main() {
    let mut quadrants: HashMap<Quadrant, usize> = HashMap::new();
    for line in stdin().lines().map(Result::unwrap) {
        let quadrant = line.parse::<Robot>().unwrap().quadrant_after(100);
        *quadrants.entry(quadrant).or_default() += 1;
    }
    let result = quadrants[&(Ordering::Less, Ordering::Less)]
        * quadrants[&(Ordering::Less, Ordering::Greater)]
        * quadrants[&(Ordering::Greater, Ordering::Less)]
        * quadrants[&(Ordering::Greater, Ordering::Greater)];
    println!("{result}");
}
