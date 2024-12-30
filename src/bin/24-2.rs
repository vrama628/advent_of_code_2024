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
    ast::{exists_const, forall_const, Array, Ast, Bool, Datatype, Int, BV},
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

impl Circuit {
    fn parse() -> Self {
        let mut lines = stdin().lines().map(Result::unwrap);
        let total_input_bits = lines.by_ref().take_while(|line| !line.is_empty()).count();
        debug_assert_eq!(total_input_bits % 2, 0);
        let input_bits = (total_input_bits / 2) as u32;
        let input_bits = 9;
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

    fn sort_key_in(&self, name: &str) -> usize {
        if let Some(i) = name.strip_prefix(&['x', 'y']) {
            i.parse().unwrap()
        } else {
            let (a, _, b) = &self.gates[name];
            1000 + self.sort_key_in(a).max(self.sort_key_in(b))
        }
    }

    fn sort_key(&self, (out, (in1, _, in2)): (&String, &(String, Op, String))) -> usize {
        if let Some(i) = out.strip_prefix('z') {
            i.parse::<usize>().unwrap() * 2
        } else {
            self.sort_key_in(in1) + self.sort_key_in(in2)
        }
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
        self.gates.iter().sorted_by_key(|&g| self.sort_key(g))
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

const NUM_SWAPS: usize = 1;

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
    /*solver.assert(&Ast::distinct(
        &ctx,
        &swaps.iter().flat_map(|(a, b)| [a, b]).collect_vec(),
    ));*/
    /*let swap = FuncDecl::new(
        &ctx,
        "swap",
        &[&wires.datatype.sort, &wires.datatype.sort],
        &Sort::bool(&ctx),
    );
    {
        let w = Datatype::fresh_const(&ctx, "w", &wires.datatype.sort);
        solver.assert(&forall_const(
            &ctx,
            &[&w],
            &[],
            &swap.apply(&[&w, &w]).as_bool().unwrap().not(),
        ));
    }
    {
        let a = Datatype::fresh_const(&ctx, "a", &wires.datatype.sort);
        let b = Datatype::fresh_const(&ctx, "b", &wires.datatype.sort);
        solver.assert(&forall_const(
            &ctx,
            &[&a, &b],
            &[&Pattern::new(&ctx, &[&swap.apply(&[&a, &b])])],
            &swap
                .apply(&[&a, &b])
                .as_bool()
                .unwrap()
                .implies(&swap.apply(&[&b, &a]).as_bool().unwrap()),
        ))
    }*/
    /*for (a, b) in &swaps {
        solver.assert(&swap.apply(&[a, b]).as_bool().unwrap());
        solver.assert(&swap.apply(&[b, a]).as_bool().unwrap());
    }
    {
        let a = Datatype::fresh_const(&ctx, "a", &wires.datatype.sort);
        let b = Datatype::fresh_const(&ctx, "b", &wires.datatype.sort);
        let disjunction = swaps
            .iter()
            .flat_map(|(s1, s2)| {
                [
                    Bool::and(&ctx, &[&a._eq(s1), &b._eq(s2)]),
                    Bool::and(&ctx, &[&b._eq(s1), &a._eq(s2)]),
                ]
            })
            .collect_vec();
        solver.assert(&forall_const(
            &ctx,
            &[&a, &b],
            &[&Pattern::new(&ctx, &[&swap.apply(&[&a, &b])])],
            &(swap
                .apply(&[&a, &b])
                .as_bool()
                .unwrap()
                .implies(&Bool::or(&ctx, &disjunction.iter().collect_vec()))),
        ));
    }*/
    match solver.check() {
        SatResult::Sat => println!("SWAPS"),
        result => panic!("{result:?}"),
    }

    /*let permute = FuncDecl::new(
        &ctx,
        "permute",
        &[&wires.datatype.sort],
        &wires.datatype.sort,
    );
    let a = Datatype::fresh_const(&ctx, "a", &wires.datatype.sort);
    solver.assert(&forall_const(
        &ctx,
        &[&a],
        &[],
        &permute
            .apply(&[&permute.apply(&[&a])])
            .as_datatype()
            .unwrap()
            ._eq(&a),
    ));
    let w = Datatype::fresh_const(&ctx, "w", &wires.datatype.sort);
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

    for i in 0..46 {
        //circuit.input_bits + 1
        let x = BV::fresh_const(&ctx, "x", circuit.input_bits);
        let y = BV::fresh_const(&ctx, "y", circuit.input_bits);
        let z = x.zero_ext(1).bvadd(&y.zero_ext(1));
        let value = if i <= circuit.input_bits {
            z.extract(i, i)
        } else {
            BV::from_u64(&ctx, 0, 1)
        };
        let out = wires.construct(&format!("z{i:02}"));
        let quantified = eval.apply(&[&x, &y, &out]).as_bv().unwrap()._eq(&value);
        solver.assert(&forall_const(&ctx, &[&x, &y], &[], &quantified));
        match solver.check() {
            SatResult::Sat => println!("INITIAL z{i:02}"),
            result => panic!("{result:?}"),
        }
    }

    for g @ (out, (in1, op, in2)) in circuit.gates() {
        let quantified = {
            let x = BV::fresh_const(&ctx, "x", circuit.input_bits);
            let y = BV::fresh_const(&ctx, "y", circuit.input_bits);
            let in_wire = |name: &str| {
                if let Some(i) = name.strip_prefix("x") {
                    let i = i.parse().unwrap();
                    if i < circuit.input_bits {
                        x.extract(i, i)
                    } else {
                        BV::from_u64(&ctx, 0, 1)
                    }
                } else if let Some(i) = name.strip_prefix("y") {
                    let i = i.parse().unwrap();
                    if i < circuit.input_bits {
                        y.extract(i, i)
                    } else {
                        BV::from_u64(&ctx, 0, 1)
                    }
                } else {
                    let wire = wires.construct(name);
                    eval.apply(&[&x, &y, &wire]).as_bv().unwrap()
                }
            };
            let out = wires.construct(&out);
            let gate = op.eval(in_wire(&in1), in_wire(&in2));
            let disjunction = once(forall_const(
                &ctx,
                &[&x, &y],
                &[],
                &eval.apply(&[&x, &y, &out]).as_bv().unwrap()._eq(&gate),
            ))
            .chain(
                swaps
                    .iter()
                    .flat_map(|(a, b)| [(a, b), (b, a)])
                    .map(|(a, b)| {
                        Bool::and(
                            &ctx,
                            &[
                                &a._eq(&out),
                                &forall_const(
                                    &ctx,
                                    &[&x, &y],
                                    &[],
                                    &eval.apply(&[&x, &y, b]).as_bv().unwrap()._eq(&gate),
                                ),
                            ],
                        )
                    }),
            )
            .collect_vec();
            Bool::or(&ctx, &disjunction.iter().collect_vec())
        };
        solver.assert(&quantified);
        println!("TRYING {out} from {in1}, {in2}...");
        match solver.check() {
            SatResult::Sat => println!("GATE {out} from {in1}, {in2}"),
            result => panic!("{result:?}"),
        }
    }

    println!("Solving...");
    match solver.check() {
        SatResult::Sat => println!("Solved."),
        result => panic!("{result:?}"),
    }

    let model = solver.get_model().unwrap();
    for (a, b) in swaps {
        println!(
            "{:?} <-> {:?}",
            model.eval(&a, true).unwrap(),
            model.eval(&b, true).unwrap()
        );
    }
}
