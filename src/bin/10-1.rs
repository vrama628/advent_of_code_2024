use std::io::stdin;

use im::OrdSet;
use itertools::Itertools;

struct Input {
    rows: usize,
    cols: usize,
    map: Vec<Vec<usize>>,
    scores: Vec<Vec<Option<OrdSet<(usize, usize)>>>>,
}

impl Input {
    fn parse() -> Self {
        let map = stdin()
            .lines()
            .map(|line| {
                line.unwrap()
                    .chars()
                    .map(|c| (c as usize) - ('0' as usize))
                    .collect_vec()
            })
            .collect_vec();
        let rows = map.len();
        let cols = map.iter().map(|row| row.len()).all_equal_value().unwrap();
        let scores = vec![vec![None; cols]; rows];
        Self {
            rows,
            cols,
            map,
            scores,
        }
    }

    fn score(&mut self, row: usize, col: usize) -> OrdSet<(usize, usize)> {
        if self.scores[row][col].is_none() {
            self.scores[row][col] = Some(if self.map[row][col] == 9 {
                OrdSet::unit((row, col))
            } else {
                OrdSet::unions(
                    [
                        (row.wrapping_sub(1), col),
                        (row, col + 1),
                        (row + 1, col),
                        (row, col.wrapping_sub(1)),
                    ]
                    .into_iter()
                    .filter_map(|(r, c)| {
                        (r < self.rows && c < self.cols && self.map[r][c] == self.map[row][col] + 1)
                            .then(|| self.score(r, c))
                    }),
                )
            });
        }
        self.scores[row][col].clone().unwrap()
    }

    fn solve(&mut self) -> usize {
        let mut result = 0;
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.map[r][c] == 0 {
                    result += self.score(r, c).len();
                }
            }
        }
        result
    }
}

fn main() {
    let mut input = Input::parse();
    let result = input.solve();
    println!("{result}");
}
