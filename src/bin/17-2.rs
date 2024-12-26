use std::{io::stdin, iter::empty};

use itertools::Itertools;
use regex::Regex;
use z3::{
    ast::{Ast, Bool, BV},
    Config, Context, Optimize, SatResult,
};

#[derive(Clone)]
struct Program<O, C> {
    a: O,
    b: O,
    c: O,
    program: Vec<u8>,
    constraints: C,
}

const SIZE: u32 = 64;

impl Program<u64, ()> {
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
        Self {
            a,
            b,
            c,
            program,
            constraints: (),
        }
    }

    fn corrupt(self, ctx: &Context) -> Program<BV<'_>, Bool<'_>> {
        Program {
            a: BV::new_const(ctx, "A", SIZE),
            b: BV::from_u64(ctx, self.b, SIZE),
            c: BV::from_u64(ctx, self.c, SIZE),
            program: self.program,
            constraints: Bool::from_bool(ctx, true),
        }
    }
}

impl<'ctx> Program<BV<'ctx>, Bool<'ctx>> {
    fn ctx(&self) -> &'ctx Context {
        self.constraints.get_ctx()
    }

    fn combo(&self, operand: u8) -> BV<'ctx> {
        match operand {
            0 | 1 | 2 | 3 => BV::from_u64(self.ctx(), operand as u64, SIZE),
            4 => self.a.clone(),
            5 => self.b.clone(),
            6 => self.c.clone(),
            _ => panic!(),
        }
    }

    fn and(&mut self, constraint: Bool<'ctx>) {
        self.constraints = Bool::and(self.ctx(), &[&self.constraints, &constraint])
    }

    fn run(mut self, i: usize, output: usize) -> Box<dyn Iterator<Item = Bool<'ctx>> + 'ctx> {
        if output > self.program.len() {
            return Box::new(empty());
        }
        if i >= self.program.len() {
            return if output == self.program.len() {
                Box::new(std::iter::once(self.constraints))
            } else {
                Box::new(empty())
            };
        }
        match self.program[i] {
            0 => self.a = self.a.bvashr(&self.combo(self.program[i + 1])),
            1 => self.b ^= &BV::from_u64(self.ctx(), self.program[i + 1] as u64, SIZE),
            2 => {
                self.b = self
                    .combo(self.program[i + 1])
                    .bvurem(&BV::from_u64(self.ctx(), 8, SIZE))
            }
            3 => {
                let mut no_jump = self.clone();
                no_jump.and(self.a._eq(&BV::from_u64(self.ctx(), 0, SIZE)));
                self.and(BV::distinct(
                    self.ctx(),
                    &[&self.a, &BV::from_u64(self.ctx(), 0, SIZE)],
                ));
                let jump_to = self.program[i + 1] as usize;
                return Box::new(no_jump.run(i + 2, output).chain(self.run(jump_to, output)));
            }
            4 => self.b ^= &self.c,
            5 => {
                return if output >= self.program.len() {
                    Box::new(empty())
                } else {
                    self.and(
                        self.combo(self.program[i + 1])
                            .bvurem(&BV::from_u64(self.ctx(), 8, SIZE))
                            ._eq(&BV::from_u64(self.ctx(), self.program[output] as u64, SIZE)),
                    );
                    self.run(i + 2, output + 1)
                };
            }
            6 => self.b = self.a.bvashr(&self.combo(self.program[i + 1])),
            7 => {
                self.c = self.a.bvashr(&self.combo(self.program[i + 1]));
            }
            _ => panic!(),
        }
        self.run(i + 2, output)
    }
}

fn main() {
    let mut cfg = Config::new();
    cfg.set_model_generation(true);
    let ctx = Context::new(&cfg);
    let program = Program::parse().corrupt(&ctx);
    let a = program.a.clone();
    let optimize = Optimize::new(&ctx);
    optimize.minimize(&a);
    for constraints in program.run(0, 0) {
        optimize.push();
        optimize.assert(&constraints);
        if let SatResult::Sat = optimize.check(&[]) {
            let model = optimize.get_model().unwrap();
            println!("{}", model.eval(&a, true).unwrap().as_u64().unwrap())
        }
        optimize.pop();
    }
}
