use std::{fmt::Display, io::stdin, iter::empty, rc::Rc};

use itertools::Itertools;
use regex::Regex;

#[derive(Clone)]
struct Program<O> {
    a: O,
    b: O,
    c: O,
    program: Vec<u8>,
}

enum Operand {
    A,
    Const(usize),
    ShiftRight(Op, Op),
    XOr(Op, Op),
    Mod8(Op),
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::A => write!(f, "A"),
            Operand::Const(c) => write!(f, "{c}"),
            Operand::ShiftRight(a, b) => write!(f, "({a} >> {b})"),
            Operand::XOr(a, b) => write!(f, "({a} ^ {b})"),
            Operand::Mod8(x) => write!(f, "({x} % 8)"),
        }
    }
}

type Op = Rc<Operand>;

enum Constraint {
    Eq(Op, u8),
    NotZero(Op),
}

impl Constraint {
    fn and(self, cs: Cs) -> Cs {
        Rc::new(Constraints::And(self, cs))
    }
}

impl Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Constraint::Eq(operand, c) => write!(f, "{operand} == {c}"),
            Constraint::NotZero(operand) => write!(f, "{operand} != 0"),
        }
    }
}

enum Constraints {
    T,
    And(Constraint, Cs),
}

impl Constraints {
    fn t() -> Cs {
        Rc::new(Self::T)
    }
}

impl Display for Constraints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Constraints::T => Ok(()),
            Constraints::And(constraint, constraints) => write!(f, "{constraints}{constraint}\n"),
        }
    }
}

type Cs = Rc<Constraints>;

impl Program<usize> {
    fn parse() -> Self {
        let mut lines = stdin().lines();
        let register_regex = Regex::new(r"Register [ABC]: (\d+)").unwrap();
        let (a, b, c) = lines
            .by_ref()
            .map(|line| {
                register_regex.captures(&line.unwrap()).unwrap()[1]
                    .parse()
                    .unwrap()
            })
            .next_tuple()
            .unwrap();
        lines.next().unwrap().unwrap();
        let program_regex = regex::Regex::new(r"Program: ((?:\d,)*\d)").unwrap();
        let program = program_regex
            .captures(&lines.next().unwrap().unwrap())
            .unwrap()[1]
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        Self { a, b, c, program }
    }

    fn corrupt(self) -> Program<Op> {
        Program {
            a: Rc::new(Operand::A),
            b: Rc::new(Operand::Const(self.b)),
            c: Rc::new(Operand::Const(self.c)),
            program: self.program,
        }
    }
}

impl Program<Op> {
    fn combo(&self, operand: u8) -> Op {
        match operand {
            0 | 1 | 2 | 3 => Rc::new(Operand::Const(operand as usize)),
            4 => self.a.clone(),
            5 => self.b.clone(),
            6 => self.c.clone(),
            _ => panic!(),
        }
    }

    fn run(mut self, i: usize, constraints: Cs, output: usize) -> Box<dyn Iterator<Item = Cs>> {
        if output > self.program.len() {
            return Box::new(empty());
        }
        if i >= self.program.len() {
            return if output == self.program.len() {
                Box::new(std::iter::once(constraints))
            } else {
                Box::new(empty())
            };
        }
        match self.program[i] {
            0 => {
                self.a = Rc::new(Operand::ShiftRight(
                    self.a.clone(),
                    self.combo(self.program[i + 1]),
                ))
            }
            1 => {
                self.b = Rc::new(Operand::XOr(
                    self.b.clone(),
                    Rc::new(Operand::Const(self.program[i + 1] as usize)),
                ))
            }
            2 => self.b = Rc::new(Operand::Mod8(self.combo(self.program[i + 1]))),
            3 => {
                let a = self.a.clone();
                let jump_to = self.program[i + 1] as usize;
                return Box::new(
                    self.clone()
                        .run(
                            i + 2,
                            Constraint::Eq(self.a.clone(), 0).and(constraints.clone()),
                            output,
                        )
                        .chain(self.run(jump_to, Constraint::NotZero(a).and(constraints), output)),
                );
            }
            4 => self.b = Rc::new(Operand::XOr(self.b.clone(), self.c.clone())),
            5 => {
                return if output >= self.program.len() {
                    Box::new(empty())
                } else {
                    let actual = Rc::new(Operand::Mod8(self.combo(self.program[i + 1])));
                    let expected = self.program[output];
                    self.run(
                        i + 2,
                        Constraint::Eq(actual, expected).and(constraints),
                        output + 1,
                    )
                };
            }
            6 => {
                self.b = Rc::new(Operand::ShiftRight(
                    self.a.clone(),
                    self.combo(self.program[i + 1]),
                ))
            }
            7 => {
                self.c = Rc::new(Operand::ShiftRight(
                    self.a.clone(),
                    self.combo(self.program[i + 1]),
                ))
            }
            _ => panic!(),
        }
        self.run(i + 2, constraints, output)
    }
}

fn main() {
    let program = Program::parse().corrupt();
    for constraints in program.run(0, Constraints::t(), 0) {
        println!("CONSTRAINTS:\n{constraints}");
    }
}
