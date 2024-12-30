use std::{backtrace, collections::BTreeMap, fmt::Display, io::stdin, str::FromStr, vec};

use im::OrdSet;
use regex::Regex;

#[derive(Clone)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn eval(&self, a: bool, b: bool) -> bool {
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
    input_bits: usize,
    gates: BTreeMap<String, (String, Op, String)>,
}

impl Circuit {
    fn parse() -> Self {
        let mut lines = stdin().lines().map(Result::unwrap);
        let input_bits = lines.by_ref().take_while(|line| !line.is_empty()).count();
        debug_assert_eq!(input_bits % 2, 0);
        let input_bits = input_bits / 2;
        let regex = Regex::new(r"(\w+) (AND|OR|XOR) (\w+) -> (\w+)").unwrap();
        let gates = lines
            .map(|line| {
                let (_, [in1, op, in2, out]) = regex.captures(&line).unwrap().extract();
                (
                    out.to_owned(),
                    (in1.to_owned(), op.parse().unwrap(), in2.to_owned()),
                )
            })
            .collect();
        Self { input_bits, gates }
    }

    fn reachable_from(&self, wire: &str, z: usize) -> OrdSet<String> {
        if let Some(i) = wire.strip_prefix(['x', 'y']) {
            let i: usize = i.parse().unwrap();
            if i > z {
                println!("ANOMALY: {wire} is reachable from z{z:02}");
            }
            OrdSet::unit(wire.to_owned())
        } else {
            let (a, _, b) = &self.gates[wire];
            self.reachable_from(a, z)
                .union(self.reachable_from(b, z))
                .update(wire.to_owned())
        }
    }

    fn detect_anomalies(&self) {
        let mut prev_reachable = OrdSet::new();
        for z in 0..self.input_bits {
            let reachable = self.reachable_from(&format!("z{z:02}"), z);
            for xy in 0..z {
                if !reachable.contains(&format!("x{:02}", xy)) {
                    println!("ANOMALY: x{xy:02} is not reachable from z{z:02}");
                }
                if !reachable.contains(&format!("y{:02}", xy)) {
                    println!("ANOMALY: y{xy:02} is not reachable from z{z:02}");
                }
            }
            for prev in prev_reachable
                .clone()
                .relative_complement(reachable.clone())
            {
                if prev != format!("z{:02}", z - 1) {
                    println!(
                        "NOTE: {prev} was reachable from z{:02} but not from z{z:02}",
                        z - 1
                    );
                }
            }
            prev_reachable = reachable;
        }
    }

    fn behavior(&self, gate: &str) -> Result<Behavior, String> {
        if let Some(i) = gate.strip_prefix('x') {
            Ok(Behavior::X(i.parse().unwrap()))
        } else if let Some(i) = gate.strip_prefix('y') {
            Ok(Behavior::Y(i.parse().unwrap()))
        } else {
            let (a, op, b) = &self.gates[gate];
            match (self.behavior(a)?, op, self.behavior(b)?) {
                (Behavior::X(0), Op::Xor, Behavior::Y(0))
                | (Behavior::Y(0), Op::Xor, Behavior::X(0)) => Ok(Behavior::Z(0)),
                (Behavior::X(0), Op::And, Behavior::Y(0))
                | (Behavior::Y(0), Op::And, Behavior::Y(0)) => Ok(Behavior::Carry(0)),
                (Behavior::Xor(p, q), Op::Xor, r) | (r, Op::Xor, Behavior::Xor(p, q)) => {
                    let mut operands = vec![*p, *q, r];
                    operands.sort();
                    match operands[..] {
                        [Behavior::X(xi), Behavior::Y(yi), Behavior::Carry(zi)] => {
                            if xi == yi && xi == zi + 1 && yi == zi + 1 {
                                Ok(Behavior::Z(xi))
                            } else {
                                Err(format!("{gate}: Almost Z: {:?}", operands))
                            }
                        }
                        _ => {
                            for op in &operands {
                                println!("  {op}");
                            }
                            Err(format!("{gate}: Not close Z: {:?}", operands))
                        }
                    }
                }
                (Behavior::And(_, _), Op::Xor, _) | (_, Op::Xor, Behavior::And(_, _)) => {
                    Err(format!("{gate}: AND in XOR"))
                }
                (Behavior::Or(_, _), Op::Xor, _) | (_, Op::Xor, Behavior::Or(_, _)) => {
                    Err(format!("{gate}: OR in XOR"))
                }
                (Behavior::Or(p, q), Op::Or, r) | (r, Op::Or, Behavior::Or(p, q)) => {
                    let mut operands = vec![*p, *q, r];
                    operands.sort();
                    match operands[..] {
                        [Behavior::And(ref l1, ref r1), Behavior::And(ref l2, ref r2), Behavior::And(ref l3, ref r3)] => {
                            Err(format!(
                                "TODO: Recognize Carry {l1:?} & {r1:?} | {l2:?} & {r2:?} | {l3:?} & {r3:?}"
                            ))
                        }
                        _ => Err(format!("{gate}: Not close Carry: {:?}", operands)),
                    }
                }
                (Behavior::Or(p, q), Op::And, r) | (r, Op::And, Behavior::Or(p, q)) => {
                    let mut operands = vec![*p, *q, r];
                    operands.sort();
                    match operands[..] {
                        [Behavior::X(xi), Behavior::Y(yi), Behavior::Carry(zi)] => {
                            if xi == yi && xi == zi && yi == zi {
                                Ok(Behavior::Carry(xi))
                            } else {
                                Err(format!("{gate}: Almost Carry: {:?}", operands))
                            }
                        }
                        _ => Err(format!("{gate}: Not close Carry: {:?}", operands)),
                    }
                }
                (a, Op::Xor, b) => Ok(Behavior::Xor(Box::new(a), Box::new(b))),
                (a, Op::And, b) => Ok(Behavior::And(Box::new(a), Box::new(b))),
                (a, Op::Or, b) => Ok(Behavior::Or(Box::new(a), Box::new(b))),
            }
        }
    }
}

fn mode<I: IntoIterator<Item = usize>>(iterator: I) -> usize {
    todo!()
}

/*
a & b | b & c | c & a == a & (b | c) | b & c
*/

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Behavior {
    X(usize),
    Y(usize),
    Z(usize),
    Carry(usize),
    And(Box<Behavior>, Box<Behavior>),
    Or(Box<Behavior>, Box<Behavior>),
    Xor(Box<Behavior>, Box<Behavior>),
}

struct Valuation {
    x: bool,
    y: bool,
    c: bool,
}

impl Behavior {
    fn eval(&self, i: usize, &Valuation { x, y, c }: &Valuation) -> Result<bool, String> {
        match self {
            Behavior::X(_) => todo!(),
            Behavior::Y(_) => todo!(),
            Behavior::Z(_) => Err("unexpected Z".to_owned()),
            Behavior::Carry(_) => todo!(),
            Behavior::And(behavior, behavior1) => todo!(),
            Behavior::Or(behavior, behavior1) => todo!(),
            Behavior::Xor(behavior, behavior1) => todo!(),
        }
    }
}

impl Display for Behavior {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X(i) => write!(f, "x{i:02}"),
            Self::Y(i) => write!(f, "y{i:02}"),
            Self::Z(i) => write!(f, "z{i:02}"),
            Self::Carry(i) => write!(f, "c{i:02}"),
            Self::And(a, b) => write!(f, "({a} & {b})"),
            Self::Or(a, b) => write!(f, "({a} | {b})"),
            Self::Xor(a, b) => write!(f, "({a} ^ {b})"),
        }
    }
}

// first anomaly: z08. It's obviously wrong, but need to figure out what to swap it with

fn main() {
    let circuit = Circuit::parse();
    for i in 0..=circuit.input_bits {
        match circuit.behavior(&format!("z{i:02}")) {
            Err(e) => {
                println!("ERROR: on z{i:02}: {e}");
                break;
            }
            Ok(Behavior::Z(zi)) => {
                if zi == i {
                    println!("Everything from z{i:02} is FINE");
                } else {
                    println!("Expected z{i:02}, got z{zi:02}");
                    break;
                }
            }
            Ok(b) => {
                println!("Expected z{i:02}, got {b:?}");
                break;
            }
        }
    }
}
