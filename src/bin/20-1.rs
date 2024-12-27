use std::{
    cell::OnceCell,
    collections::{btree_map::Entry, BTreeMap, VecDeque},
    io::stdin,
    ops::Index,
};

use itertools::Itertools;

type Position = (usize, usize);

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

    fn solve(self) -> usize {
        let mut queue = VecDeque::from([(self.start.clone(), None, 0)]);
        let mut visited = BTreeMap::from([(self.start.clone(), BTreeMap::from([(None, 0)]))]);
        while let Some((position, cheat, picoseconds)) = queue.pop_front() {
            if position == self.end && cheat.is_none() {
                return visited[&self.end]
                    .iter()
                    .filter(|&(_, &p)| p + 100 <= picoseconds)
                    .count();
            }
            for adjacent in self.adjacent(&position) {
                if let Some(cheat) = if self[&adjacent] {
                    Some(cheat)
                } else if cheat.is_none() {
                    Some(Some(adjacent.clone()))
                } else {
                    None
                } {
                    if let Entry::Vacant(vacant) = visited
                        .entry(adjacent.clone())
                        .or_default()
                        .entry(cheat.clone())
                    {
                        vacant.insert(picoseconds + 1);
                        queue.push_back((adjacent, cheat, picoseconds + 1));
                    }
                }
            }
        }
        panic!()
    }
}

fn main() {
    let result = Racetrack::parse().solve();
    println!("{result}");
}
