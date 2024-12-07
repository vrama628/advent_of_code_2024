use std::io::stdin;

use itertools::{izip, Itertools};

struct Input(Vec<Vec<char>>);

type Window = ((char, char, char), (char, char, char), (char, char, char));

fn is_mas(a: char, b: char, c: char) -> bool {
    a == 'M' && b == 'A' && c == 'S' || a == 'S' && b == 'A' && c == 'M'
}

fn is_xmas(((a, _, b), (_, c, _), (d, _, e)): &Window) -> bool {
    is_mas(*a, *c, *e) && is_mas(*b, *c, *d)
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

    fn windows(&self) -> impl Iterator<Item = Window> + '_ {
        self.0.iter().tuple_windows().flat_map(|(a, b, c)| {
            izip!(
                a.iter().copied().tuple_windows(),
                b.iter().copied().tuple_windows(),
                c.iter().copied().tuple_windows()
            )
        })
    }

    fn solve(&self) -> usize {
        self.windows().filter(is_xmas).count()
    }
}

fn main() {
    let input = Input::parse();
    debug_assert!(input.is_valid());
    let result = input.solve();
    println!("{}", result);
}
