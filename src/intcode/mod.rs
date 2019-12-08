use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Clone)]
pub struct Intcode {
    memory: Vec<i64>,
    inputs: VecDeque<i64>,
    pc: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StopCondition {
    Halt,
    NeedInput,
}

impl Intcode {
    pub fn new(program: &str) -> Intcode {
        Intcode {
            memory: Intcode::parse(program),
            inputs: VecDeque::new(),
            pc: 0,
        }
    }

    pub fn inputs(mut self, inputs: &[i64]) -> Intcode {
        self.inputs.extend(inputs);
        self
    }

    pub fn update(mut self, pos: usize, val: i64) -> Intcode {
        self.memory[pos] = val;
        self
    }

    pub fn value(&self, pos: usize) -> i64 {
        self.memory[pos]
    }

    pub fn push_input(&mut self, input: i64) {
        self.inputs.push_back(input);
    }

    pub fn run(&mut self) -> (Vec<i64>, StopCondition) {
        let mut outs = Vec::with_capacity(16);

        loop {
            let op = self.memory[self.pc] as u64;

            let opc = op % 100;
            let mode = op / 100;

            match opc {
                1 => {
                    let ps = self.load_params(2, mode);
                    let rd = self.memory[self.pc + 3] as usize;
                    self.memory[rd] = ps[0] + ps[1];
                    self.pc += 4;
                }
                2 => {
                    let ps = self.load_params(2, mode);
                    let rd = self.memory[self.pc + 3] as usize;
                    self.memory[rd] = ps[0] * ps[1];
                    self.pc += 4;
                }
                3 => {
                    let rd = self.memory[self.pc + 1] as usize;
                    if let Some(input) = self.inputs.pop_front() {
                        self.memory[rd] = input;
                        self.pc += 2;
                    } else {
                        return (outs, StopCondition::NeedInput);
                    }
                }
                4 => {
                    let ps = self.load_params(1, mode);
                    outs.push(ps[0]);
                    self.pc += 2;
                }
                5 => {
                    let ps = self.load_params(2, mode);
                    if ps[0] != 0 {
                        self.pc = ps[1] as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                6 => {
                    let ps = self.load_params(2, mode);
                    if ps[0] == 0 {
                        self.pc = ps[1] as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                7 => {
                    let ps = self.load_params(2, mode);
                    let rd = self.memory[self.pc + 3] as usize;
                    self.memory[rd] = (ps[0] < ps[1]) as i64;
                    self.pc += 4;
                }
                8 => {
                    let ps = self.load_params(2, mode);
                    let rd = self.memory[self.pc + 3] as usize;
                    self.memory[rd] = (ps[0] == ps[1]) as i64;
                    self.pc += 4;
                }
                99 => break,
                _ => panic!("unexpected opcode: {}", self.memory[self.pc]),
            }
        }

        (outs, StopCondition::Halt)
    }

    fn load_params(&self, n: usize, mode: u64) -> Vec<i64> {
        (self.pc + 1..=self.pc + n)
            .fold((mode, Vec::with_capacity(n)), |(mode, mut v), idx| {
                if mode % 10 == 0 {
                    v.push(self.memory[self.memory[idx] as usize]);
                } else {
                    v.push(self.memory[idx]);
                }
                (mode / 10, v)
            })
            .1
    }

    fn parse(program: &str) -> Vec<i64> {
        program
            .split(',')
            .map(|s| i64::from_str(s).unwrap())
            .collect::<Vec<_>>()
    }
}
