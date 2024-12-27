use std::{
    cell::OnceCell,
    collections::{BTreeMap, VecDeque},
    io::stdin,
    ops::Index,
};

const CHEAT_LENGTH: usize = 20;
const MINIMUM_IMPROVEMENT: usize = 100;

use itertools::Itertools;

type Position = (usize, usize);

fn manhattan(&(from_row, from_col): &Position, &(to_row, to_col): &Position) -> usize {
    from_row.abs_diff(to_row) + from_col.abs_diff(to_col)
}

struct Racetrack {
    map: Vec<Vec<bool>>,
    rows: usize,
    cols: usize,
    start: Position,
    end: Position,
}

impl Index<&Position> for Racetrack {
    type Output = bool;

    fn index(&self, &(row, col): &Position) -> &Self::Output {
        &self.map[row][col]
    }
}

impl Racetrack {
    fn parse() -> Self {
        let start = OnceCell::new();
        let end = OnceCell::new();
        let map = stdin()
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.unwrap()
                    .char_indices()
                    .map(|(col, c)| match c {
                        '#' => false,
                        '.' => true,
                        'S' => {
                            start.set((row, col)).unwrap();
                            true
                        }
                        'E' => {
                            end.set((row, col)).unwrap();
                            true
                        }
                        _ => panic!(),
                    })
                    .collect_vec()
            })
            .collect_vec();
        let rows = map.len();
        let cols = map.iter().map(|row| row.len()).all_equal_value().unwrap();
        Self {
            map,
            rows,
            cols,
            start: start.into_inner().unwrap(),
            end: end.into_inner().unwrap(),
        }
    }

    fn adjacent(&self, &(row, col): &Position) -> impl Iterator<Item = Position> + '_ {
        [
            (row.wrapping_sub(1), col),
            (row, col.wrapping_sub(1)),
            (row, col + 1),
            (row + 1, col),
        ]
        .into_iter()
        .filter(|&(row, col)| row < self.rows && col < self.cols)
    }

    fn shortest_without_cheat(&self, start: &Position) -> BTreeMap<Position, usize> {
        let mut queue = VecDeque::from([(start.clone(), 0)]);
        let mut visited = BTreeMap::from([(start.clone(), 0)]);
        while let Some((position, picoseconds)) = queue.pop_front() {
            for adjacent in self.adjacent(&position) {
                if self[&adjacent] && !visited.contains_key(&adjacent) {
                    visited.insert(adjacent.clone(), picoseconds + 1);
                    queue.push_back((adjacent, picoseconds + 1));
                }
            }
        }
        return visited;
    }

    fn cheatable(&self, &from: &Position) -> impl Iterator<Item = Position> + '_ {
        let (row, col) = from;
        (row.saturating_sub(CHEAT_LENGTH)..=(row + CHEAT_LENGTH).min(self.rows - 1)).flat_map(
            move |r| {
                let radius = CHEAT_LENGTH - row.abs_diff(r);
                (col.saturating_sub(radius)..=(col + radius).min(self.cols - 1))
                    .map(move |c| (r, c))
                    .filter(|to| self[to])
            },
        )
    }

    fn solve(self) -> usize {
        let distances: BTreeMap<Position, BTreeMap<Position, usize>> = (0..self.rows)
            .cartesian_product(0..self.cols)
            .filter(|position| self[&position])
            .map(|position| (position, self.shortest_without_cheat(&position)))
            .collect();
        let without_cheat = distances[&self.start][&self.end];
        distances
            .keys()
            .flat_map(|from| {
                self.cheatable(from).filter(|to| {
                    distances[&self.start][from] + manhattan(from, to) + distances[to][&self.end]
                        <= without_cheat - MINIMUM_IMPROVEMENT
                })
            })
            .count()
    }
}

fn main() {
    let result = Racetrack::parse().solve();
    println!("{result}");
}
