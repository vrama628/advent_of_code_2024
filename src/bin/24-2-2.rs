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
        if let Some(x) = gate.strip_prefix('x') {
            let x: usize = x.parse().unwrap();
            Ok(Behavior::X(x))
        } else if let Some(y) = gate.strip_prefix('y') {
            let y: usize = y.parse().unwrap();
            Ok(Behavior::Y(y))
        } else {
            let (a, op, b) = &self.gates[gate];
            let a = self.behavior(a)?;
            let b = self.behavior(b)?;
            match op {
                Op::And => Behavior::And(Box::new(a), Box::new(b)).normalize(),
                Op::Or => Behavior::Or(Box::new(a), Box::new(b)).normalize(),
                Op::Xor => Behavior::Xor(Box::new(a), Box::new(b)).normalize(),
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
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

impl From<u8> for Valuation {
    fn from(value: u8) -> Self {
        if value >= 8 {
            panic!()
        }
        let x = ((value >> 0) & 1) == 1;
        let y = ((value >> 1) & 1) == 1;
        let c = ((value >> 2) & 1) == 1;
        Self { x, y, c }
    }
}

impl Behavior {
    fn digit(&self) -> Result<usize, String> {
        match self {
            Behavior::X(i) | Behavior::Y(i) | Behavior::Z(i) => Ok(*i),
            Behavior::Carry(i) => Ok(*i + 1),
            Behavior::And(a, b) | Behavior::Or(a, b) | Behavior::Xor(a, b) => {
                let a = a.digit()?;
                let b = b.digit()?;
                if a != b {
                    Err(format!("digit mismatch in {self}"))
                } else {
                    Ok(a)
                }
            }
        }
    }

    fn normalize(&self) -> Result<Self, String> {
        let i = self.digit()?;
        match self.blast(i)? {
            0b11101000 => Ok(Self::Carry(i)),
            0b10010110 => Ok(Self::Z(i)),
            0b10001000 if i == 0 => Ok(Self::Carry(0)),
            0b01100110 if i == 0 => Ok(Self::Z(0)),
            _ => Ok(self.clone()),
        }
    }

    fn blast(&self, i: usize) -> Result<u8, String> {
        let mut res = 0;
        for v in 0..8 {
            res |= ((self.eval(i, &v.into())?) as u8) << v;
        }
        Ok(res)
    }

    fn eval(&self, i: usize, valuation: &Valuation) -> Result<bool, String> {
        let err = || Err(format!("Encountered {self} when evaluating {i:02}"));
        match self {
            Behavior::X(xi) => {
                if *xi == i {
                    Ok(valuation.x)
                } else {
                    err()
                }
            }
            Behavior::Y(yi) => {
                if *yi == i {
                    Ok(valuation.y)
                } else {
                    err()
                }
            }
            Behavior::Z(_) => err(),
            Behavior::Carry(ci) => {
                if *ci + 1 == i {
                    Ok(valuation.c)
                } else {
                    err()
                }
            }
            Behavior::And(a, b) => Ok(a.eval(i, valuation)? & b.eval(i, valuation)?),
            Behavior::Or(a, b) => Ok(a.eval(i, valuation)? | b.eval(i, valuation)?),
            Behavior::Xor(a, b) => Ok(a.eval(i, valuation)? ^ b.eval(i, valuation)?),
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
