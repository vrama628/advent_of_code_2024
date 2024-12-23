use std::{
    io::stdin,
    ops::{Index, IndexMut},
};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Robot,
    Wall,
    BoxLeft,
    BoxRight,
}

impl Cell {
    fn parse(c: char) -> [Option<Self>; 2] {
        match c {
            '#' => [Some(Self::Wall), Some(Self::Wall)],
            'O' => [Some(Self::BoxLeft), Some(Self::BoxRight)],
            '@' => [Some(Self::Robot), None],
            '.' => [None, None],
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
    fn go(&self, direction: &Move) -> Self {
        let &Self { row, col } = self;
        match direction {
            Move::Up => Position { row: row - 1, col },
            Move::Down => Position { row: row + 1, col },
            Move::Left => Position { row, col: col - 1 },
            Move::Right => Position { row, col: col + 1 },
        }
    }
    fn gps(&self) -> usize {
        self.row * 100 + self.col
    }
}

#[derive(Debug, PartialEq, Eq)]
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

    fn is_vertical(&self) -> bool {
        matches!(self, Self::Up | Self::Down)
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
                .map(|line| line.chars().flat_map(Cell::parse).collect())
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

    fn can_push(&self, direction: &Move, position: Position) -> bool {
        match self[&position] {
            Some(Cell::Wall) => false,
            Some(Cell::BoxLeft) => {
                if direction.is_vertical() {
                    self.can_push(direction, position.go(direction))
                        && self.can_push(direction, position.go(&Move::Right).go(direction))
                } else {
                    debug_assert_eq!(direction, &Move::Right);
                    self.can_push(direction, position.go(direction).go(direction))
                }
            }
            Some(Cell::BoxRight) => {
                if direction.is_vertical() {
                    self.can_push(direction, position.go(direction))
                        && self.can_push(direction, position.go(&Move::Left).go(direction))
                } else {
                    debug_assert_eq!(direction, &Move::Left);
                    self.can_push(direction, position.go(direction).go(direction))
                }
            }
            Some(Cell::Robot) => panic!(),
            None => true,
        }
    }

    // precondition: self.can_push(direction, position)
    fn push(&mut self, direction: &Move, position: Position, item: Cell) {
        match self[&position].replace(item) {
            Some(left @ Cell::BoxLeft) => {
                self.push(direction, position.go(direction), left);
                if direction.is_vertical() {
                    let right_pos = position.go(&Move::Right);
                    let right = self[&right_pos].take().unwrap();
                    debug_assert_eq!(right, Cell::BoxRight);
                    self.push(direction, right_pos.go(&direction), right)
                }
            }
            Some(right @ Cell::BoxRight) => {
                self.push(direction, position.go(direction), right);
                if direction.is_vertical() {
                    let left_pos = position.go(&Move::Left);
                    let left = self[&left_pos].take().unwrap();
                    debug_assert_eq!(left, Cell::BoxLeft);
                    self.push(direction, left_pos.go(&direction), left)
                }
            }
            Some(Cell::Wall | Cell::Robot) => panic!(),
            None => (),
        }
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
                        (cell == Some(Cell::BoxLeft)).then(|| Position { row, col }.gps())
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
            if map.can_push(&m, robot.go(&m)) {
                let item = map[&robot].take().unwrap();
                debug_assert_eq!(item, Cell::Robot);
                robot = robot.go(&m);
                map.push(&m, robot.clone(), item)
            }
        }
        map.gps()
    }
}

fn main() {
    let result = Input::parse().solve();
    println!("{result}");
}
