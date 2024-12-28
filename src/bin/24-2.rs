use std::{collections::HashMap, io::stdin, str::FromStr};

use regex::Regex;
use z3::{
    ast::{Ast, BV},
    Config, Context, SatResult, Solver,
};

#[derive(Clone)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn eval<'ctx>(&self, a: BV<'ctx>, b: BV<'ctx>) -> BV<'ctx> {
        match self {
            Op::And => a & b,
            Op::Or => a | b,
            Op::Xor => a ^ b,
        }
    }
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Op::And),
            "OR" => Ok(Op::Or),
            "XOR" => Ok(Op::Xor),
            _ => Err(()),
        }
    }
}

struct Circuit<'ctx> {
    ctx: &'ctx Context,
    x: BV<'ctx>,
    y: BV<'ctx>,
    z: BV<'ctx>,
    wires: HashMap<String, BV<'ctx>>,
}

impl<'ctx> Circuit<'ctx> {
    fn parse(ctx: &'ctx Context) -> Self {
        let input = stdin()
            .lines()
            .map(Result::unwrap)
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| line.chars().last().unwrap())
            .collect::<String>();
        debug_assert_eq!(input.len() % 2, 0);
        let bits = input.len() / 2;
        let x = BV::from_u64(
            &ctx,
            u64::from_str_radix(&input[..bits].chars().rev().collect::<String>(), 2).unwrap(),
            bits as u32,
        );
        let y = BV::from_u64(
            &ctx,
            u64::from_str_radix(&input[bits..].chars().rev().collect::<String>(), 2).unwrap(),
            bits as u32,
        );
        let z = BV::new_const(ctx, "z", bits as u32 + 1);
        let wires = HashMap::new();
        Self {
            ctx,
            x,
            y,
            z,
            wires,
        }
    }

    fn wire(&mut self, wire: &str) -> BV<'ctx> {
        if let Some(i) = wire.strip_prefix("x") {
            let i = i.parse().unwrap();
            self.x.extract(i, i)
        } else if let Some(i) = wire.strip_prefix("y") {
            let i = i.parse().unwrap();
            self.y.extract(i, i)
        } else if let Some(i) = wire.strip_prefix("z") {
            let i = i.parse().unwrap();
            self.z.extract(i, i)
        } else {
            self.wires
                .entry(wire.to_string())
                .or_insert_with(|| BV::new_const(self.ctx, wire, 1))
                .clone()
        }
    }

    fn solve(&mut self) -> u64 {
        let regex = Regex::new(r"(\w+) (AND|OR|XOR) (\w+) -> (\w+)").unwrap();
        let solver = Solver::new(self.ctx);
        for line in stdin().lines().map(Result::unwrap) {
            let [in1, op, in2, out] = regex.captures(&line).unwrap().extract().1;
            let op = op.parse::<Op>().unwrap();
            solver.assert(&op.eval(self.wire(in1), self.wire(in2))._eq(&self.wire(out)));
        }
        let SatResult::Sat = solver.check() else {
            panic!()
        };
        let model = solver.get_model().unwrap();
        model.eval(&self.z, true).unwrap().as_u64().unwrap()
    }
}

fn main() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let mut circuit = Circuit::parse(&ctx);
    let result = circuit.solve();
    println!("{}", result);
}
