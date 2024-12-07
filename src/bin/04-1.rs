use std::io::stdin;

use itertools::Itertools;

struct Input(Vec<Vec<char>>);

fn is_xmas(xmas: &(char, char, char, char)) -> bool {
    *xmas == ('X', 'M', 'A', 'S') || *xmas == ('S', 'A', 'M', 'X')
}

impl Input {
    fn parse() -> Self {
        Self(
            stdin()
                .lines()
                .map(|line| line.unwrap().chars().collect())
                .collect(),
        )
    }

    fn length(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn is_valid(&self) -> bool {
        self.0.iter().all(|line| line.len() == self.width())
            && self
                .0
                .iter()
                .flatten()
                .all(|c| ['X', 'M', 'A', 'S'].contains(&c))
    }

    fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = char> + '_> + '_ {
        self.0.iter().map(|line| line.iter().copied())
    }

    fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = char> + '_> + '_ {
        (0..self.width()).map(|i| self.0.iter().map(move |line| line[i]))
    }

    fn backward_diagonals(&self) -> impl Iterator<Item = impl Iterator<Item = char> + '_> + '_ {
        (-(self.length() as i64)..self.width() as i64).map(|offset| {
            self.0.iter().enumerate().filter_map(move |(i, line)| {
                line.get(usize::try_from(offset + i as i64).ok()?).copied()
            })
        })
    }

    fn forward_diagonals(&self) -> impl Iterator<Item = impl Iterator<Item = char> + '_> + '_ {
        (0..self.width() + self.length()).map(|offset| {
            self.0
                .iter()
                .enumerate()
                .filter_map(move |(i, line)| line.get(offset.checked_sub(i)?).copied())
        })
    }

    fn solve(&self) -> usize {
        self.rows()
            .flat_map(Itertools::tuple_windows)
            .chain(self.columns().flat_map(Itertools::tuple_windows))
            .chain(self.backward_diagonals().flat_map(Itertools::tuple_windows))
            .chain(self.forward_diagonals().flat_map(Itertools::tuple_windows))
            .filter(is_xmas)
            .count()
    }
}

fn main() {
    let input = Input::parse();
    debug_assert!(input.is_valid());
    let result = input.solve();
    println!("{}", result);
}
