use std::{io::stdin, str::FromStr};

use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;
const MOST_CHRISTMASY_NUMBER: usize = 72;

struct Robot {
    px: i64,
    py: i64,
    vx: i64,
    vy: i64,
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        static REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap());
        let captures = REGEX.captures(&line).ok_or(())?;
        let px = captures[1].parse::<i64>().unwrap();
        let py = captures[2].parse::<i64>().unwrap();
        let vx = captures[3].parse::<i64>().unwrap();
        let vy = captures[4].parse::<i64>().unwrap();
        Ok(Self { px, py, vx, vy })
    }
}

impl Robot {
    fn step(&mut self) {
        self.px = (self.px + self.vx).rem_euclid(WIDTH);
        self.py = (self.py + self.vy).rem_euclid(HEIGHT);
    }
}

fn main() {
    let mut robots = stdin()
        .lines()
        .map(|line| line.unwrap().parse::<Robot>().unwrap())
        .collect_vec();
    for i in 0..((WIDTH * HEIGHT) as usize) {
        let mut display = vec![vec![' ' as u8; WIDTH as usize]; HEIGHT as usize];
        for robot in &mut robots {
            display[robot.py as usize][robot.px as usize] = '#' as u8;
            robot.step();
        }
        if i % HEIGHT as usize == MOST_CHRISTMASY_NUMBER {
            println!("\n\n\n\n");
            for row in display {
                println!("{}", String::from_utf8(row).unwrap());
            }
            println!("{i}");
        }
    }
}
