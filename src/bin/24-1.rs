use std::{collections::BTreeMap, io::stdin, str::FromStr};

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

type Circuit = BTreeMap<String, Result<bool, (String, Op, String)>>;

fn eval(circuit: &Circuit, wire: &String) -> bool {
    match &circuit[wire] {
        &Ok(res) => res,
        Err((a, op, b)) => op.eval(eval(circuit, a), eval(circuit, b)),
    }
}

fn main() {
    let mut circuit: Circuit = BTreeMap::new();
    let mut lines = stdin().lines().map(Result::unwrap);
    for line in lines.by_ref().take_while(|line| !line.is_empty()) {
        let (wire, value) = line.split_once(": ").unwrap();
        circuit.insert(wire.to_owned(), Ok(value == "1"));
    }
    let regex = Regex::new(r"(\w+) (AND|OR|XOR) (\w+) -> (\w+)").unwrap();
    for line in lines {
        let captures = regex.captures(&line).unwrap();
        circuit.insert(
            captures[4].to_owned(),
            Err((
                captures[1].to_owned(),
                captures[2].parse().unwrap(),
                captures[3].to_owned(),
            )),
        );
    }
    let binary: String = circuit
        .keys()
        .rev()
        .take_while(|wire| wire.starts_with("z"))
        .map(|wire| if eval(&circuit, &wire) { '1' } else { '0' })
        .collect();
    let result = usize::from_str_radix(&binary, 2).unwrap();
    println!("{result}");
}
