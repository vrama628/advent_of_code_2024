use std::{collections::BTreeMap, fmt::Display, io::stdin, str::FromStr, vec};

use regex::Regex;

#[derive(Clone)]
enum Op {
    And,
    Or,
    Xor,
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

const INPUT_BITS: usize = 45;

#[derive(Clone)]
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

    fn behavior(&self, gate: &str) -> Result<Behavior, String> {
        if let Some(x) = gate.strip_prefix('x') {
            let x: usize = x.parse().unwrap();
            Ok(Behavior::X(x))
        } else if let Some(y) = gate.strip_prefix('y') {
            let y: usize = y.parse().unwrap();
            Ok(Behavior::Y(y))
        } else {
            let (a, op, b) = &self.gates[gate];
            let a_behavior = self.behavior(a)?;
            let b_behavior = self.behavior(b)?;
            match op {
                Op::And => Behavior::And(Box::new(a_behavior), Box::new(b_behavior)).normalize(),
                Op::Or => Behavior::Or(Box::new(a_behavior), Box::new(b_behavior)).normalize(),
                Op::Xor => Behavior::Xor(Box::new(a_behavior), Box::new(b_behavior)).normalize(),
            }
        }
    }

    fn swap(&mut self, a: &str, b: &str) {
        let a_gate = self.gates.remove(a).unwrap();
        let b_gate = self.gates.insert(b.to_owned(), a_gate).unwrap();
        let none = self.gates.insert(a.to_owned(), b_gate);
        debug_assert!(none.is_none())
    }
}

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
            Behavior::X(i) | Behavior::Y(i) => Ok(*i),
            Behavior::Z(i) => Err(format!("Encountered z{i:02} when getting digit")),
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
            0b11101000 if i == INPUT_BITS - 1 => Ok(Self::Z(INPUT_BITS)),
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
        let err = || Err(format!("Encountered {self} when evaluating z{i:02}"));
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

fn main() {
    let mut circuit = Circuit::parse();
    circuit.swap("z08", "cdj");
    circuit.swap("z16", "mrb");
    circuit.swap("z32", "gfm");
    circuit.swap("dhm", "qjd");
    for i in 0..=circuit.input_bits {
        match circuit.behavior(&format!("z{i:02}")) {
            Err(e) => {
                println!("ERROR: on z{i:02}: {e}");
            }
            Ok(Behavior::Z(zi)) => {
                if zi == i {
                    println!("Everything from z{i:02} is FINE");
                } else {
                    println!("Expected z{i:02}, got z{zi:02}");
                }
            }
            Ok(b) => {
                println!("Expected z{i:02}, got {b}");
            }
        }
    }
    let mut res = vec!["z08", "cdj", "z16", "mrb", "z32", "gfm", "dhm", "qjd"];
    res.sort();
    println!("{}", res.join(","));
}
