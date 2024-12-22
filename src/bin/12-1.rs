use std::{
    collections::BTreeSet,
    io::stdin,
    ops::{AddAssign, Index},
};

use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn adjacent(&self) -> impl Iterator<Item = Self> {
        let Self { row, col } = *self;
        [
            Self {
                row: row.wrapping_sub(1),
                col,
            },
            Self { row: row + 1, col },
            Self {
                row,
                col: col.wrapping_sub(1),
            },
            Self { row, col: col + 1 },
        ]
        .into_iter()
    }
}

struct Region {
    area: usize,
    perimeter: usize,
}

impl Region {
    fn one() -> Self {
        Self {
            area: 1,
            perimeter: 0,
        }
    }

    fn price(&self) -> usize {
        self.area * self.perimeter
    }
}

impl AddAssign for Region {
    fn add_assign(&mut self, other: Self) {
        self.area += other.area;
        self.perimeter += other.perimeter;
    }
}

struct Input {
    rows: usize,
    cols: usize,
    plot: Vec<Vec<char>>,
}

impl Index<&Position> for Input {
    type Output = char;

    fn index(&self, &Position { row, col }: &Position) -> &Self::Output {
        &self.plot[row][col]
    }
}

impl Input {
    fn parse() -> Self {
        let plot = stdin()
            .lines()
            .map(|line| line.unwrap().chars().collect_vec())
            .collect_vec();
        let rows = plot.len();
        let cols = plot.iter().map(|row| row.len()).all_equal_value().unwrap();
        Self { rows, cols, plot }
    }

    fn contains(&self, &Position { row, col }: &Position) -> bool {
        row < self.rows && col < self.cols
    }

    fn flood(&self, remaining: &mut BTreeSet<Position>, position: &Position) -> Region {
        let mut region = Region::one();
        for adjacent in position.adjacent() {
            if self.contains(&adjacent) && self[&adjacent] == self[position] {
                if remaining.remove(&adjacent) {
                    region += self.flood(remaining, &adjacent);
                }
            } else {
                region.perimeter += 1;
            }
        }
        region
    }

    fn solve(&self) -> usize {
        let mut remaining: BTreeSet<Position> = (0..self.rows)
            .flat_map(|row| (0..self.cols).map(move |col| Position { row, col }))
            .collect();
        let mut result = 0;
        while let Some(position) = remaining.pop_first() {
            result += self.flood(&mut remaining, &position).price()
        }
        result
    }
}

fn main() {
    let input = Input::parse();
    let result = input.solve();
    println!("{result}");
}
