use std::{collections::HashMap, hash::Hash, io::stdin, iter::repeat_n};

use itertools::Itertools;

trait FoldingMapExt {
    fn folding_map<B, F>(self, b: B, f: F) -> FoldingMap<Self, B, F>
    where
        Self: Sized,
        B: Copy;
}

impl<T: Iterator> FoldingMapExt for T {
    fn folding_map<B, F>(self, b: B, f: F) -> FoldingMap<Self, B, F>
    where
        Self: Sized,
        B: Copy,
    {
        FoldingMap { i: self, b, f }
    }
}

struct FoldingMap<I, B, F> {
    i: I,
    b: B,
    f: F,
}

impl<I, B, F> Iterator for FoldingMap<I, B, F>
where
    I: Iterator,
    F: FnMut(I::Item, B) -> B,
    B: Copy,
{
    type Item = B;

    fn next(&mut self) -> Option<B> {
        self.i.next().map(|x| {
            self.b = (self.f)(x, self.b);
            self.b
        })
    }
}

type Position = (usize, usize);

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum DirectionPad {
    Right,
    Up,
    Down,
    Left,
    A,
}

impl DirectionPad {
    fn go(&self, (r, c): Position) -> Position {
        match self {
            DirectionPad::Right => (r, c + 1),
            DirectionPad::Up => (r - 1, c),
            DirectionPad::Down => (r + 1, c),
            DirectionPad::Left => (r, c - 1),
            DirectionPad::A => panic!(),
        }
    }
}

trait Positioned {
    const PANIC: Position;
    fn position(&self) -> Position;
}

impl Positioned for DirectionPad {
    const PANIC: Position = (0, 0);

    fn position(&self) -> Position {
        match self {
            Self::Up => (0, 1),
            Self::A => (0, 2),
            Self::Left => (1, 0),
            Self::Down => (1, 1),
            Self::Right => (1, 2),
        }
    }
}

impl Positioned for char {
    const PANIC: Position = (3, 0);

    fn position(&self) -> Position {
        match self {
            '7' => (0, 0),
            '8' => (0, 1),
            '9' => (0, 2),
            '4' => (1, 0),
            '5' => (1, 1),
            '6' => (1, 2),
            '1' => (2, 0),
            '2' => (2, 1),
            '3' => (2, 2),
            '0' => (3, 1),
            'A' => (3, 2),
            _ => panic!(),
        }
    }
}

trait Length {
    type Item;
    fn eval(&mut self, sequence: &[Self::Item]) -> usize;
}

struct Memo<D, I> {
    next: D,
    memo: HashMap<(I, I), usize>,
}

impl<D: Length<Item = DirectionPad>, I: Positioned + Copy + Hash + Eq> Memo<D, I> {
    fn new(next: D) -> Self {
        Self {
            next,
            memo: HashMap::new(),
        }
    }

    fn pair(&mut self, a: I, b: I) -> usize {
        *self.memo.entry((a, b)).or_insert_with(|| {
            let (from_row, from_col) = a.position();
            let (to_row, to_col) = b.position();
            let vertical = if from_row < to_row {
                DirectionPad::Down
            } else {
                DirectionPad::Up
            };
            let horizontal = if from_col < to_col {
                DirectionPad::Right
            } else {
                DirectionPad::Left
            };
            let rows = from_row.abs_diff(to_row);
            let cols = from_col.abs_diff(to_col);
            repeat_n(vertical, rows)
                .chain(repeat_n(horizontal, cols))
                .permutations(rows + cols)
                .filter(|x| {
                    !x.iter()
                        .folding_map((from_row, from_col), DirectionPad::go)
                        .contains(&I::PANIC)
                })
                .map(|mut perm| {
                    perm.push(DirectionPad::A);
                    self.next.eval(&perm)
                })
                .min()
                .unwrap()
        })
    }
}

impl<D: Length<Item = DirectionPad>, I: Positioned + Copy + Hash + Eq> Length for Memo<D, I> {
    type Item = I;

    fn eval(&mut self, sequence: &[I]) -> usize {
        sequence
            .iter()
            .circular_tuple_windows()
            .map(|(&a, &b)| self.pair(a, b))
            .sum()
    }
}

struct One;

impl Length for One {
    type Item = DirectionPad;
    fn eval(&mut self, sequence: &[DirectionPad]) -> usize {
        sequence.len()
    }
}

fn main() {
    let mut memo = Memo::new(Memo::new(Memo::new(Memo::new(Memo::new(Memo::new(
        Memo::new(Memo::new(Memo::new(Memo::new(Memo::new(Memo::new(
            Memo::new(Memo::new(Memo::new(Memo::new(Memo::new(Memo::new(
                Memo::new(Memo::new(Memo::new(Memo::new(Memo::new(Memo::new(
                    Memo::new(Memo::new(One)),
                )))))),
            )))))),
        )))))),
    ))))));
    let result = stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let length = memo.eval(&line.chars().collect_vec());
            let num = line[..line.len() - 1].parse::<usize>().unwrap();
            length * num
        })
        .sum::<usize>();
    println!("{result}");
}
