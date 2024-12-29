use std::{collections::HashMap, io::stdin, str::FromStr};

use itertools::Itertools;
use regex::Regex;
use z3::{
    ast::{forall_const, Ast, Bool, Datatype, Dynamic, BV},
    Config, Context, DatatypeBuilder, DatatypeSort, FuncDecl, SatResult, Solver, Sort,
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

struct Circuit {
    input_bits: u32,
    gates: HashMap<String, (String, Op, String)>,
}

impl Circuit {
    fn parse() -> Self {
        let mut lines = stdin().lines().map(Result::unwrap);
        let total_input_bits = lines.by_ref().take_while(|line| !line.is_empty()).count();
        debug_assert_eq!(total_input_bits % 2, 0);
        let input_bits = (total_input_bits / 2) as u32;
        let regex = Regex::new(r"(\w+) (AND|OR|XOR) (\w+) -> (\w+)").unwrap();
        let gates = lines
            .map(|line| {
                let [in1, op, in2, out] = regex.captures(&line).unwrap().extract().1;
                let op = op.parse::<Op>().unwrap();
                (out.to_owned(), (in1.to_owned(), op, in2.to_owned()))
            })
            .collect();
        Self { input_bits, gates }
    }

    fn wires<'ctx>(&self, ctx: &'ctx Context) -> Wires<'ctx> {
        let datatype = self
            .gates
            .keys()
            .fold(DatatypeBuilder::new(&ctx, "Wire"), |dtb, wire| {
                dtb.variant(&wire, vec![])
            })
            .finish();
        let variants = datatype
            .variants
            .iter()
            .enumerate()
            .map(|(i, variant)| (variant.constructor.name(), i))
            .collect();
        Wires { datatype, variants }
    }

    fn gates(&self) -> impl Iterator<Item = (&String, &(String, Op, String))> {
        self.gates.iter()
    }
    /*
    fn solve(&mut self) -> u64 {
        let mut gates = Bool::from_bool(self.ctx, true);
        for line in stdin().lines().map(Result::unwrap) {
            let [in1, op, in2, out] = regex.captures(&line).unwrap().extract().1;
            let op = op.parse::<Op>().unwrap();
            gates &= op.eval(self.wire(in1), self.wire(in2))._eq(&self.wire(out));
        }
        let goal = self.x.bvand(&self.y)._eq(&self.z);
        let solver = Solver::new(self.ctx);
        solver.assert(&forall_const(
            self.ctx,
            &[&self.x, &self.y, &self.z],
            &[],
            &gates.implies(&goal),
        ));
        let SatResult::Sat = solver.check() else {
            panic!()
        };
        let model = solver.get_model().unwrap();
        println!("x: {:?}", model.eval(&self.x, true));
        println!("y: {:?}", model.eval(&self.y, true));
        println!("z: {:?}", model.eval(&self.z, true));
        0
    }*/
}

struct Wires<'ctx> {
    datatype: DatatypeSort<'ctx>,
    variants: HashMap<String, usize>,
}

impl<'ctx> Wires<'ctx> {
    fn construct(&self, wire: &str) -> Datatype<'ctx> {
        self.datatype.variants[self.variants[wire]]
            .constructor
            .apply(&[])
            .as_datatype()
            .unwrap()
    }
}

const NUM_SWAPS: usize = 0; // 4

fn main() {
    let circuit = Circuit::parse();
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let wires = circuit.wires(&ctx);
    let solver = Solver::new(&ctx);
    let swaps = (0..NUM_SWAPS * 2)
        .map(|_| Datatype::fresh_const(&ctx, "swap", &wires.datatype.sort))
        .tuples::<(_, _)>()
        .collect_vec();
    solver.assert(&Ast::distinct(
        &ctx,
        &swaps.iter().flat_map(|(a, b)| [a, b]).collect_vec(),
    ));
    let permute = FuncDecl::new(
        &ctx,
        "permute",
        &[&wires.datatype.sort],
        &wires.datatype.sort,
    );
    let permute_constraint = {
        let x = Datatype::fresh_const(&ctx, "x", &wires.datatype.sort);
        let lhs = permute.apply(&[&x]).as_datatype().unwrap();
        let rhs = swaps.iter().fold(x.clone(), |e, (a, b)| {
            Bool::ite(&x._eq(a), b, &Bool::ite(&x._eq(b), a, &e))
        });
        forall_const(&ctx, &[&x], &[], &lhs._eq(&rhs))
    };
    solver.assert(&permute_constraint);
    let eval = FuncDecl::new(
        &ctx,
        "eval",
        &[&wires.datatype.sort],
        &Sort::bitvector(&ctx, 1),
    );
    let x = BV::fresh_const(&ctx, "x", circuit.input_bits);
    let y = BV::fresh_const(&ctx, "y", circuit.input_bits);
    let z = BV::fresh_const(&ctx, "z", circuit.input_bits); // + 1
    for i in 0..z.get_size() {
        // + 1
        let wire = wires.construct(&format!("z{i:02}"));
        let evaluated = eval.apply(&[&wire]).as_bv().unwrap();
        solver.assert(&evaluated._eq(&z.extract(i, i)));
    }
    let in_wire = |name: &str| {
        if let Some(i) = name.strip_prefix("x") {
            let i = i.parse().unwrap();
            x.extract(i, i)
        } else if let Some(i) = name.strip_prefix("y") {
            let i = i.parse().unwrap();
            y.extract(i, i)
        } else {
            let wire = wires.construct(name);
            eval.apply(&[&wire]).as_bv().unwrap()
        }
    };
    let out_wire = |name: &str| {
        let wire = wires.construct(name);
        let permuted = permute.apply(&[&wire]).as_datatype().unwrap();
        eval.apply(&[&permuted]).as_bv().unwrap()
    };
    let mut gates = Bool::from_bool(&ctx, true);
    for (out, (in1, op, in2)) in circuit.gates() {
        gates &= op.eval(in_wire(&in1), in_wire(&in2))._eq(&out_wire(&out));
    }
    let goal = x.bvand(&y)._eq(&z);
    solver.assert(&forall_const(
        &ctx,
        &[&x, &y, &z],
        &[],
        &gates.implies(&goal),
    ));
    solver.check();
    let SatResult::Sat = solver.check() else {
        panic!()
    };
    let model = solver.get_model().unwrap();
    for (a, b) in swaps {
        println!("{:?}<->{:?}", model.eval(&a, true), model.eval(&b, true));
    }
}
