use std::{collections::HashSet, io::stdin};

type Position = (usize, usize);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn r#move(&self, (r, c): Position) -> Position {
        match self {
            Direction::Up => (r.wrapping_sub(1), c),
            Direction::Down => (r + 1, c),
            Direction::Left => (r, c.wrapping_sub(1)),
            Direction::Right => (r, c + 1),
        }
    }

    fn turn(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

struct Input {
    rows: usize,
    cols: usize,
    obstructions: HashSet<Position>,
    start: Position,
}

impl Input {
    fn parse() -> Self {
        let mut rows = 0;
        let mut cols = None;
        let mut obstructions = HashSet::new();
        let mut start = None;
        for (r, line) in stdin().lines().map(Result::unwrap).enumerate() {
            rows += 1;
            match cols {
                None => cols = Some(line.len()),
                Some(cols) => debug_assert_eq!(cols, line.len()),
            }
            for (c, char) in line.char_indices() {
                match char {
                    '#' => {
                        obstructions.insert((r, c));
                    }
                    '^' => {
                        debug_assert!(start.is_none());
                        start = Some((r, c));
                    }
                    _ => {}
                }
            }
        }
        Input {
            rows,
            cols: cols.unwrap(),
            obstructions,
            start: start.unwrap(),
        }
    }

    fn is_within(&self, (r, c): Position) -> bool {
        r < self.rows && c < self.cols
    }

    fn solve(&self) -> usize {
        let mut direction = Direction::Up;
        let mut position = self.start;
        let mut visited = HashSet::new();

        while self.is_within(position) {
            visited.insert(position);
            let next = direction.r#move(position);
            if self.obstructions.contains(&next) {
                direction = direction.turn();
            } else {
                position = next
            }
        }
        visited.len()
    }
}

fn main() {
    let input = Input::parse();
    let result = input.solve();
    println!("{result}");
}
