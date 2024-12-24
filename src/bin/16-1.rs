use std::{
    cell::OnceCell,
    collections::{BTreeSet, BinaryHeap},
    io::stdin,
    ops::Index,
};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn go(&self, direction: &Direction) -> Self {
        let &Self { row, col } = self;
        match direction {
            Direction::Up => Self {
                row: row.wrapping_sub(1),
                col,
            },
            Direction::Down => Self { row: row + 1, col },
            Direction::Left => Self {
                row,
                col: col.wrapping_sub(1),
            },
            Direction::Right => Self { row, col: col + 1 },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&self) -> [Self; 2] {
        match self {
            Self::Up | Self::Down => [Self::Left, Self::Right],
            Self::Left | Self::Right => [Self::Up, Self::Down],
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct PositionAndDirection {
    position: Position,
    direction: Direction,
}

#[derive(PartialEq, Eq)]
struct Node {
    score: usize,
    position_and_direction: PositionAndDirection,
}

impl Ord for Node {
    /// a low score is always ordered greater than a high score
    /// to make BinaryHeap behave like a min-heap
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score).then(
            other
                .position_and_direction
                .cmp(&self.position_and_direction),
        )
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Map {
    grid: Vec<Vec<bool>>,
    rows: usize,
    cols: usize,
}

impl Index<&Position> for Map {
    type Output = bool;

    fn index(&self, &Position { row, col }: &Position) -> &Self::Output {
        &self.grid[row][col]
    }
}

impl Map {
    fn can_go(&self, position: &Position) -> bool {
        position.row < self.rows && position.col < self.cols && self[position]
    }

    fn neighbors(
        &self,
        Node {
            score,
            position_and_direction:
                PositionAndDirection {
                    position,
                    direction,
                },
        }: &Node,
    ) -> impl Iterator<Item = Node> {
        let [a, b] = direction.turn();
        let mut res = vec![
            Node {
                score: score + 1000,
                position_and_direction: PositionAndDirection {
                    position: position.clone(),
                    direction: a,
                },
            },
            Node {
                score: score + 1000,
                position_and_direction: PositionAndDirection {
                    position: position.clone(),
                    direction: b,
                },
            },
        ];
        let gone = position.go(&direction);
        if self.can_go(&gone) {
            res.push(Node {
                score: score + 1,
                position_and_direction: PositionAndDirection {
                    position: gone,
                    direction: direction.clone(),
                },
            })
        }
        res.into_iter()
    }
}

struct Input {
    map: Map,
    start: PositionAndDirection,
    end: Position,
}

impl Input {
    fn parse() -> Self {
        let start = OnceCell::new();
        let end = OnceCell::new();
        let grid = stdin()
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.unwrap()
                    .char_indices()
                    .map(|(col, c)| match c {
                        '#' => false,
                        '.' => true,
                        'S' => {
                            start
                                .set(PositionAndDirection {
                                    position: Position { row, col },
                                    direction: Direction::Right,
                                })
                                .unwrap();
                            true
                        }
                        'E' => {
                            end.set(Position { row, col }).unwrap();
                            true
                        }
                        _ => panic!(),
                    })
                    .collect_vec()
            })
            .collect_vec();
        let rows = grid.len();
        let cols = grid.iter().map(|row| row.len()).all_equal_value().unwrap();
        Self {
            map: Map { grid, rows, cols },
            start: start.into_inner().unwrap(),
            end: end.into_inner().unwrap(),
        }
    }

    fn solve(self) -> usize {
        let Input { map, start, end } = self;
        let mut visited = BTreeSet::new();
        let mut queue = BinaryHeap::from([Node {
            score: 0,
            position_and_direction: start,
        }]);
        while let Some(node) = queue.pop() {
            if !visited.insert(node.position_and_direction.clone()) {
                continue;
            }
            if node.position_and_direction.position == end {
                return node.score;
            }
            for neighbor in map.neighbors(&node) {
                queue.push(neighbor);
            }
        }
        panic!()
    }
}

fn main() {
    let result = Input::parse().solve();
    println!("{result}");
}
