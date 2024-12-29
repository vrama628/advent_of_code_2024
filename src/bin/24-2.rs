use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    io::stdin,
    iter::once,
    str::FromStr,
};

use itertools::Itertools;
use regex::Regex;
use z3::{
    ast::{forall_const, Array, Ast, Bool, Datatype, BV},
    Config, Context, DatatypeBuilder, DatatypeSort, FuncDecl, Params, Pattern, SatResult, Solver,
    Sort, Tactic,
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

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::And => write!(f, "&"),
            Op::Or => write!(f, "|"),
            Op::Xor => write!(f, "^"),
        }
    }
}

struct Circuit {
    input_bits: u32,
    gates: HashMap<String, (String, Op, String)>,
}

fn sort_key(name: &str) -> usize {
    if let Some(i) = name.strip_prefix(&['x', 'y']) {
        i.parse().unwrap()
    } else {
        1000
    }
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
        self.gates
            .iter()
            .sorted_by_key(|(_, (a, _, b))| sort_key(a) + sort_key(b))
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

const NUM_SWAPS: usize = 4;

fn main() {
    let circuit = Circuit::parse();
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let wires = circuit.wires(&ctx);
    let solver = Solver::new(&ctx);

    /*let swaps = (0..NUM_SWAPS * 2)
        .map(|_| Datatype::fresh_const(&ctx, "swap", &wires.datatype.sort))
        .tuples::<(_, _)>()
        .collect_vec();
    solver.assert(&Ast::distinct(
        &ctx,
        &swaps.iter().flat_map(|(a, b)| [a, b]).collect_vec(),
    ));
    match solver.check() {
        SatResult::Sat => println!("SWAPS"),
        result => panic!("{result:?}"),
    }*/

    let permute = FuncDecl::new(
        &ctx,
        "permute",
        &[&wires.datatype.sort],
        &wires.datatype.sort,
    );
    /*let a = Datatype::fresh_const(&ctx, "a", &wires.datatype.sort);
    solver.assert(&forall_const(
        &ctx,
        &[&a],
        &[],
        &permute
            .apply(&[&permute.apply(&[&a])])
            .as_datatype()
            .unwrap()
            ._eq(&a),
    ));*/
    /*let w = Datatype::fresh_const(&ctx, "w", &wires.datatype.sort);
    let applied = permute.apply(&[&w]).as_datatype().unwrap();
    solver.assert(&forall_const(
        &ctx,
        &[&w],
        &[],
        &swaps.iter().fold(applied._eq(&w), |acc, (a, b)| {
            Bool::ite(
                &w._eq(&a),
                &applied._eq(&b),
                &Bool::ite(&w._eq(&b), &applied._eq(&a), &acc),
            )
        }),
    ));
    match solver.check() {
        SatResult::Sat => print!("PERMUTE"),
        result => panic!("{result:?}"),
    }*/

    let eval = FuncDecl::new(
        &ctx,
        "eval",
        &[
            &Sort::bitvector(&ctx, circuit.input_bits),
            &Sort::bitvector(&ctx, circuit.input_bits),
            &wires.datatype.sort,
        ],
        &Sort::bitvector(&ctx, 1),
    );

    let mut i = 0;
    for (out, (in1, op, in2)) in circuit.gates() {
        let x = BV::fresh_const(&ctx, "x", circuit.input_bits);
        let y = BV::fresh_const(&ctx, "y", circuit.input_bits);
        let in_wire = |name: &str| {
            if let Some(i) = name.strip_prefix("x") {
                let i = i.parse().unwrap();
                x.extract(i, i)
            } else if let Some(i) = name.strip_prefix("y") {
                let i = i.parse().unwrap();
                y.extract(i, i)
            } else {
                let wire = wires.construct(name);
                eval.apply(&[&x, &y, &wire]).as_bv().unwrap()
            }
        };
        let out = wires.construct(&out);
        let quantified = eval
            .apply(&[&x, &y, &permute.apply(&[&out])])
            .as_bv()
            .unwrap()
            ._eq(&op.eval(in_wire(&in1), in_wire(&in2)));
        solver.assert(&forall_const(&ctx, &[&x, &y], &[], &quantified));
        match solver.check() {
            SatResult::Sat => println!("GATE {out} from {in1}, {in2}"),
            result => panic!("{result:?}"),
        }
        let zs = (sort_key(&in1) + sort_key(&in2)) as u32 / 2;
        if zs <= circuit.input_bits && i < zs {
            let x = BV::fresh_const(&ctx, "x", circuit.input_bits);
            let y = BV::fresh_const(&ctx, "y", circuit.input_bits);
            let z = x.zero_ext(1).bvadd(&y.zero_ext(1));
            let out = wires.construct(&format!("z{i:02}"));
            let quantified = eval
                .apply(&[&x, &y, &out])
                .as_bv()
                .unwrap()
                ._eq(&z.extract(i, i));
            solver.assert(&forall_const(&ctx, &[&x, &y], &[], &quantified));
            match solver.check() {
                SatResult::Sat => println!("ZZZZ {out}"),
                result => panic!("{result:?}"),
            }
            i += 1;
        }
    }

    for i in 0..circuit.input_bits + 1 {
        let x = BV::fresh_const(&ctx, "x", circuit.input_bits);
        let y = BV::fresh_const(&ctx, "y", circuit.input_bits);
        let z = x.zero_ext(1).bvadd(&y.zero_ext(1));
        let out = wires.construct(&format!("z{i:02}"));
        let quantified = eval
            .apply(&[&x, &y, &out])
            .as_bv()
            .unwrap()
            ._eq(&z.extract(i, i));
        solver.assert(&forall_const(&ctx, &[&x, &y], &[], &quantified));
    }
    match solver.check() {
        SatResult::Sat => println!("Solved."),
        result => panic!("{result:?}"),
    }

    let model = solver.get_model().unwrap();
    for variant in wires.datatype.variants.iter() {
        let wire = variant.constructor.apply(&[]);
        let permuted = permute.apply(&[&wire]);
        println!(
            "{:?} -> {:?}",
            model.eval(&wire, true).unwrap(),
            model.eval(&permuted, true).unwrap()
        );
    }
    /*for (a, b) in swaps {
        println!(
            "{:?}<->{:?}",
            model.eval(&a, true).unwrap(),
            model.eval(&b, true).unwrap()
        );
    }*/
}
