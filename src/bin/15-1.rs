use std::{
    io::stdin,
    ops::{Index, IndexMut},
};

use itertools::Itertools;

#[derive(PartialEq, Eq)]
enum Cell {
    Robot,
    Wall,
    Box,
}

impl Cell {
    fn parse(c: char) -> Option<Self> {
        match c {
            '#' => Some(Self::Wall),
            'O' => Some(Self::Box),
            '@' => Some(Self::Robot),
            '.' => None,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn gps(&self) -> usize {
        self.row * 100 + self.col
    }
}

enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn parse(c: char) -> Self {
        match c {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!(),
        }
    }

    fn apply(&self, &Position { row, col }: &Position) -> Position {
        match self {
            Self::Up => Position { row: row - 1, col },
            Self::Down => Position { row: row + 1, col },
            Self::Left => Position { row, col: col - 1 },
            Self::Right => Position { row, col: col + 1 },
        }
    }
}

struct Map(Vec<Vec<Option<Cell>>>);

impl Index<&Position> for Map {
    type Output = Option<Cell>;

    fn index(&self, &Position { row, col }: &Position) -> &Self::Output {
        &self.0[row][col]
    }
}

impl IndexMut<&Position> for Map {
    fn index_mut(&mut self, &Position { row, col }: &Position) -> &mut Self::Output {
        &mut self.0[row][col]
    }
}

impl Map {
    fn parse() -> Self {
        Self(
            stdin()
                .lines()
                .map(Result::unwrap)
                .take_while(|line| !line.is_empty())
                .map(|line| line.chars().map(Cell::parse).collect())
                .collect(),
        )
    }

    fn robot(&self) -> Position {
        self.0
            .iter()
            .enumerate()
            .find_map(|(row, map_row)| {
                map_row.iter().enumerate().find_map(|(col, cell)| {
                    (cell == &Some(Cell::Robot)).then(|| Position { row, col })
                })
            })
            .unwrap()
    }

    /// returns Some iff failed to push
    fn push(&mut self, item: Cell, direction: &Move, position: Position) -> Option<Cell> {
        self[&position]
            .replace(item)
            .map(|c| match c {
                Cell::Wall => self[&position].replace(c),
                Cell::Box => self
                    .push(c, direction, direction.apply(&position))
                    .map(|c| self[&position].replace(c).unwrap()),
                Cell::Robot => panic!(),
            })
            .flatten()
    }

    fn gps(self) -> usize {
        self.0
            .into_iter()
            .enumerate()
            .flat_map(|(row, map_row)| {
                map_row
                    .into_iter()
                    .enumerate()
                    .filter_map(move |(col, cell)| {
                        (cell == Some(Cell::Box)).then(|| Position { row, col }.gps())
                    })
            })
            .sum()
    }
}

struct Input {
    map: Map,
    moves: Vec<Move>,
}

impl Input {
    fn parse() -> Self {
        let map = Map::parse();
        let moves = stdin()
            .lines()
            .flat_map(|line| line.unwrap().chars().map(Move::parse).collect_vec())
            .collect();
        Self { map, moves }
    }

    fn solve(self) -> usize {
        let Self { mut map, moves } = self;
        let mut robot = map.robot();
        for m in moves {
            let item = map[&robot].take().unwrap();
            debug_assert!(item == Cell::Robot);
            if let Some(item) = map.push(item, &m, m.apply(&robot)) {
                let replaced = map[&robot].replace(item);
                debug_assert!(replaced.is_none())
            } else {
                robot = m.apply(&robot);
            }
        }
        map.gps()
    }
}

fn main() {
    let result = Input::parse().solve();
    println!("{result}");
}
