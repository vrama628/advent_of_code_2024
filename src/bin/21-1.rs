use std::{io::stdin, iter::once};

use itertools::Itertools;

#[derive(Clone, Copy)]
enum DirectionPad {
    Right,
    Up,
    Down,
    Left,
    A,
}

impl DirectionPad {
    fn char(&self) -> char {
        match self {
            Self::Right => '>',
            Self::Up => '^',
            Self::Down => 'v',
            Self::Left => '<',
            Self::A => 'A',
        }
    }

    fn position(&self) -> Position {
        match self {
            Self::Up => (0, 1),
            Self::A => (0, 2),
            Self::Left => (1, 0),
            Self::Down => (1, 1),
            Self::Right => (1, 2),
        }
    }
}

trait Instruct {
    fn bot(&self) -> Vec<DirectionPad>;
}

type Position = (usize, usize);

fn numeric_position(&c: &char) -> Position {
    match c {
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '0' => (3, 1),
        'A' => (3, 2),
        _ => panic!(),
    }
}

fn instructions(
    &(from_row, from_col): &Position,
    &(to_row, to_col): &Position,
) -> Vec<DirectionPad> {
    let mut result = Vec::with_capacity(6);
    if from_col < to_col {
        result.extend(vec![DirectionPad::Right; to_col - from_col]);
    }
    if to_row < from_row {
        result.extend(vec![DirectionPad::Up; from_row - to_row]);
    }
    if from_row < to_row {
        result.extend(vec![DirectionPad::Down; to_row - from_row]);
    }
    if to_col < from_col {
        result.extend(vec![DirectionPad::Left; from_col - to_col]);
    }
    result.push(DirectionPad::A);
    result
}

// for numeric pad
impl Instruct for Vec<char> {
    fn bot(&self) -> Vec<DirectionPad> {
        once(&'A')
            .chain(self.iter())
            .map(numeric_position)
            .tuple_windows()
            .flat_map(|(from, to)| instructions(&from, &to))
            .collect()
    }
}

impl Instruct for Vec<DirectionPad> {
    fn bot(&self) -> Vec<DirectionPad> {
        once(&DirectionPad::A)
            .chain(self.iter())
            .map(DirectionPad::position)
            .tuple_windows()
            .flat_map(|(from, to)| instructions(&from, &to))
            .collect()
    }
}

fn main() {
    let result = stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            println!("{line} LINE line");
            let bot = line.chars().collect_vec().bot();
            println!(
                "bot: {}",
                bot.iter().map(DirectionPad::char).collect::<String>()
            );
            let bot = bot.bot();
            println!(
                "bot: {}",
                bot.iter().map(DirectionPad::char).collect::<String>()
            );
            let bot = bot.bot();
            println!(
                "bot: {}",
                bot.iter().map(DirectionPad::char).collect::<String>()
            );
            let length = bot.len();
            let num = line[..line.len() - 1].parse::<usize>().unwrap();
            println!("{length}*{num}={}", length * num);
            length * num
        })
        .sum::<usize>();
    println!("{result}");
}
