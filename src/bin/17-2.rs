use std::{io::stdin, iter::successors, sync::mpsc::Sender, thread};

#[derive(Clone)]
struct Program<P> {
    a: usize,
    b: usize,
    c: usize,
    program: P,
}

impl Program<Vec<u8>> {
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

    fn a(&self, a: usize) -> Program<&[u8]> {
        Program {
            a,
            b: self.b,
            c: self.c,
            program: &self.program,
        }
    }
}

impl Program<&[u8]> {
    fn combo(&self, operand: u8) -> usize {
        match operand {
            0 | 1 | 2 | 3 => operand as usize,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!(),
        }
    }

    fn run(&mut self, i: usize, mut output: usize) -> bool {
        if i >= self.program.len() - 1 || output >= self.program.len() {
            return output == self.program.len();
        }
        match self.program[i] {
            0 => self.a >>= self.combo(self.program[i + 1]),
            1 => self.b ^= self.program[i + 1] as usize,
            2 => self.b = self.combo(self.program[i + 1]) % 8,
            3 => {
                if self.a != 0 {
                    return self.run(self.program[i + 1] as usize, output);
                }
            }
            4 => self.b ^= self.c,
            5 => {
                if self.program[output] != ((self.combo(self.program[i + 1]) % 8) as u8) {
                    return false;
                } else {
                    output += 1;
                }
            }
            6 => self.b = self.a >> self.combo(self.program[i + 1]),
            7 => self.c = self.a >> self.combo(self.program[i + 1]),
            _ => panic!(),
        }
        self.run(i + 2, output)
    }
}

fn thread(program: Program<Vec<u8>>, step: usize, sender: Sender<usize>, offset: usize) {
    for a in successors(Some(offset), |a| Some(a + step)) {
        if program.a(a).run(0, 0) {
            sender.send(a).unwrap();
            return;
        }
    }
}

fn main() {
    let program = Program::parse();
    let (sender, receiver) = std::sync::mpsc::channel::<usize>();
    let num_cpus = num_cpus::get();
    println!("spawning {num_cpus} threads...");
    for offset in 0..num_cpus {
        let program = program.clone();
        let sender = sender.clone();
        thread::spawn(move || thread(program, num_cpus, sender, offset));
    }
    while let Ok(result) = receiver.recv() {
        println!("{result}")
    }
}
