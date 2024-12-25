use std::io::stdin;

use itertools::Itertools;

struct Program {
    a: usize,
    b: usize,
    c: usize,
    program: Vec<u8>,
}

impl Program {
    fn parse() -> Self {
        let mut lines = stdin().lines();
        let a = lines
            .next()
            .unwrap()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let b = lines
            .next()
            .unwrap()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let c = lines
            .next()
            .unwrap()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        lines.next().unwrap().unwrap();
        let program = lines
            .next()
            .unwrap()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        Self { a, b, c, program }
    }

    fn combo(&self, operand: u8) -> usize {
        match operand {
            0 | 1 | 2 | 3 => operand as usize,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!(),
        }
    }

    fn run(&mut self, output: &mut Vec<u8>, i: usize) {
        if i >= self.program.len() {
            return;
        }
        match self.program[i] {
            0 => self.a >>= self.combo(self.program[i + 1]),
            1 => self.b ^= self.program[i + 1] as usize,
            2 => self.b = self.combo(self.program[i + 1]) % 8,
            3 => {
                if self.a != 0 {
                    return self.run(output, self.program[i + 1] as usize);
                }
            }
            4 => self.b ^= self.c,
            5 => output.push((self.combo(self.program[i + 1]) % 8) as u8),
            6 => self.b = self.a >> self.combo(self.program[i + 1]),
            7 => self.c = self.a >> self.combo(self.program[i + 1]),
            _ => panic!(),
        }
        self.run(output, i + 2)
    }
}

fn main() {
    let mut program = Program::parse();
    let mut output = Vec::new();
    program.run(&mut output, 0);
    let result = output.into_iter().map(|o| o.to_string()).join(",");
    println!("{result}");
}
