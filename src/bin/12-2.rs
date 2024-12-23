use std::{
    collections::BTreeSet,
    io::stdin,
    ops::{Add, Index},
};

use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    /// the plots surrounding a corner in counterclockwise order of bottom left, bottom right, top right, top left
    fn surrounding(&self) -> [Self; 4] {
        let Self { row, col } = *self;
        [
            Self {
                row,
                col: col.wrapping_sub(1),
            },
            Self { row, col },
            Self {
                row: row.wrapping_sub(1),
                col,
            },
            Self {
                row: row.wrapping_sub(1),
                col: col.wrapping_sub(1),
            },
        ]
    }

    /// the plots adjacent to a plot in counterclockwise order of down, right, up, left
    fn adjacent(&self) -> [Self; 4] {
        let Self { row, col } = *self;
        [
            Self { row: row + 1, col },
            Self { row, col: col + 1 },
            Self {
                row: row.wrapping_sub(1),
                col,
            },
            Self {
                row,
                col: col.wrapping_sub(1),
            },
        ]
    }

    fn incr(&self) -> Self {
        Self {
            row: self.row + 1,
            col: self.col + 1,
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, Self { row, col }: Self) -> Self {
        Self {
            row: self.row + row,
            col: self.col + col,
        }
    }
}

struct Input {
    rows: usize,
    cols: usize,
    map: Vec<Vec<char>>,
}

impl Index<Position> for Input {
    type Output = char;

    fn index(&self, Position { row, col }: Position) -> &Self::Output {
        &self.map[row][col]
    }
}

impl Input {
    fn parse() -> Self {
        let map = stdin()
            .lines()
            .map(|line| line.unwrap().chars().collect_vec())
            .collect_vec();
        let rows = map.len();
        let cols = map.iter().map(|row| row.len()).all_equal_value().unwrap();
        Self { rows, cols, map }
    }

    fn contains(&self, &Position { row, col }: &Position) -> bool {
        row < self.rows && col < self.cols
    }

    fn trace(
        &self,
        start: Position,
        remaining_corners: &mut BTreeSet<Position>,
        corner: Position,
        entered_from: usize,
    ) -> usize {
        let exit = corner
            .surrounding()
            .into_iter()
            .map(|plot: Position| self.contains(&plot) && self[plot] == self[start])
            .circular_tuple_windows::<(_, _)>()
            .enumerate()
            .cycle()
            .skip(entered_from)
            .find(|&(_, edge)| edge == (true, false))
            .unwrap()
            .0;
        let enter_next_from = (exit + 2) % 4;
        let turned = (entered_from != enter_next_from) as usize;
        let next = corner.adjacent()[exit];
        if next != start {
            remaining_corners.remove(&next);
            turned + self.trace(start, remaining_corners, next, enter_next_from)
        } else {
            turned
        }
    }

    fn flood(
        &self,
        remaining_plots: &mut BTreeSet<Position>,
        encountered_corners: &mut BTreeSet<Position>,
        position: Position,
    ) -> usize {
        let mut area = 1;
        for (i, adjacent) in position.adjacent().into_iter().enumerate() {
            if self.contains(&adjacent) && self[adjacent] == self[position] {
                if remaining_plots.remove(&adjacent) {
                    area += self.flood(remaining_plots, encountered_corners, adjacent);
                }
            } else {
                let (a, b) = position
                    .incr()
                    .surrounding()
                    .into_iter()
                    .circular_tuple_windows::<(_, _)>()
                    .nth(i)
                    .unwrap();
                encountered_corners.insert(a);
                encountered_corners.insert(b);
            }
        }
        area
    }

    fn solve(&self) -> usize {
        let mut remaining_plots: BTreeSet<Position> = (0..self.rows)
            .flat_map(|row| (0..self.cols).map(move |col| Position { row, col }))
            .collect();
        let mut result = 0;
        while let Some(position) = remaining_plots.pop_first() {
            let mut encountered_corners = BTreeSet::new();
            let flood = self.flood(&mut remaining_plots, &mut encountered_corners, position);
            let mut trace = 0;
            while let Some(position) = encountered_corners.pop_first() {
                trace += self.trace(position, &mut encountered_corners, position, 0);
            }
            result += flood * trace;
        }
        result
    }
}

fn main() {
    let input = Input::parse();
    let result = input.solve();
    println!("{result}");
}
