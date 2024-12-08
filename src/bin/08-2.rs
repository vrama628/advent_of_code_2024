use std::{
    collections::{HashMap, HashSet},
    io::stdin,
};

use itertools::{iterate, Itertools};

type Position = (usize, usize);
type Difference = (isize, isize);

fn difference((r1, c1): &Position, (r2, c2): &Position) -> Difference {
    ((*r1 as isize - *r2 as isize), (*c1 as isize - *c2 as isize))
}

fn add(u: usize, i: isize) -> usize {
    (u as isize + i) as usize
}

fn add_difference((r1, c1): &Position, (r2, c2): &Difference) -> Position {
    (add(*r1, *r2), add(*c1, *c2))
}

fn gcd(position: &Difference) -> Difference {
    // lol turns out gcd wasn't necessary
    *position
}

struct Input {
    rows: usize,
    cols: usize,
    antennas: HashMap<char, HashSet<Position>>,
}

impl Input {
    fn parse() -> Self {
        let mut rows = 0;
        let mut cols = None;
        let mut antennas: HashMap<char, HashSet<Position>> = HashMap::new();
        for (r, line) in stdin().lines().map(Result::unwrap).enumerate() {
            rows += 1;
            match cols {
                None => cols = Some(line.len()),
                Some(cols) => debug_assert_eq!(cols, line.len()),
            }
            for (c, char) in line.char_indices() {
                if char.is_ascii_alphanumeric() {
                    antennas.entry(char).or_default().insert((r, c));
                }
            }
        }
        Self {
            rows,
            cols: cols.unwrap(),
            antennas,
        }
    }

    fn is_within(&self, (r, c): &Position) -> bool {
        r < &self.rows && c < &self.cols
    }

    fn solve_frequency(&self, frequency: &char) -> impl Iterator<Item = Position> + '_ {
        self.antennas[frequency]
            .iter()
            .permutations(2)
            .flat_map(|perm| {
                let [a, b] = perm[..] else { unreachable!() };
                let step = gcd(&difference(b, a));
                iterate(*a, move |acc| add_difference(acc, &step))
                    .take_while(|position| self.is_within(position))
            })
    }

    fn solve(&self) -> usize {
        let antinodes: HashSet<Position> = self
            .antennas
            .keys()
            .flat_map(|frequency| self.solve_frequency(frequency))
            .collect();
        antinodes.len()
    }
}

fn main() {
    let input = Input::parse();
    let result = input.solve();
    println!("{result}");
}
