use std::collections::VecDeque;
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

pub type Word = i128;

#[derive(Clone)]
pub struct Intcode {
    memory: Vec<Word>,
    inputs: VecDeque<Word>,
    pc: usize,
    rb: usize,
}

#[derive(Clone, Copy, Debug)]
enum Argument {
    Absolute(usize),
    Relative(isize),
    Parameter(Word),
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
            rb: 0,
        }
    }

    pub fn inputs(mut self, inputs: &[Word]) -> Intcode {
        self.inputs.extend(inputs);
        self
    }

    pub fn update(mut self, pos: usize, val: Word) -> Intcode {
        self.memory[pos] = val;
        self
    }

    pub fn value(&self, pos: usize) -> Word {
        self.memory[pos]
    }

    pub fn push_input(&mut self, input: Word) {
        self.inputs.push_back(input);
    }

    pub fn run(&mut self) -> (Vec<Word>, StopCondition) {
        let mut outs = Vec::with_capacity(16);

        loop {
            let op = self.memory[self.pc];

            let opc = op % 100;
            let mode = (op / 100).try_into().unwrap();

            match opc {
                1 => {
                    let ps = self.args(3, mode);
                    let (a, b) = (self.rd(ps[0]), self.rd(ps[1]));
                    self.wr(ps[2], a + b);
                    self.pc += 4;
                }
                2 => {
                    let ps = self.args(3, mode);
                    let (a, b) = (self.rd(ps[0]), self.rd(ps[1]));
                    self.wr(ps[2], a * b);
                    self.pc += 4;
                }
                3 => {
                    let ps = self.args(1, mode);
                    if let Some(input) = self.inputs.pop_front() {
                        self.wr(ps[0], input);
                        self.pc += 2;
                    } else {
                        return (outs, StopCondition::NeedInput);
                    }
                }
                4 => {
                    let ps = self.args(1, mode);
                    outs.push(self.rd(ps[0]));
                    self.pc += 2;
                }
                5 => {
                    let ps = self.args(2, mode);
                    if self.rd(ps[0]) != 0 {
                        self.pc = self.rd(ps[1]).try_into().unwrap();
                    } else {
                        self.pc += 3;
                    }
                }
                6 => {
                    let ps = self.args(2, mode);
                    if self.rd(ps[0]) == 0 {
                        self.pc = self.rd(ps[1]).try_into().unwrap();
                    } else {
                        self.pc += 3;
                    }
                }
                7 => {
                    let ps = self.args(3, mode);
                    let (a, b) = (self.rd(ps[0]), self.rd(ps[1]));
                    self.wr(ps[2], (a < b).try_into().unwrap());
                    self.pc += 4;
                }
                8 => {
                    let ps = self.args(3, mode);
                    let (a, b) = (self.rd(ps[0]), self.rd(ps[1]));
                    self.wr(ps[2], (a == b).try_into().unwrap());
                    self.pc += 4;
                }
                9 => {
                    let ps = self.args(1, mode);
                    self.rb = (Word::try_from(self.rb).unwrap() + self.rd(ps[0]))
                        .try_into()
                        .unwrap();
                    self.pc += 2;
                }
                99 => break,
                _ => panic!("unexpected opcode: {}", self.memory[self.pc]),
            }
        }

        (outs, StopCondition::Halt)
    }

    fn rd(&mut self, arg: Argument) -> Word {
        match arg {
            Argument::Absolute(pos) => self.memory[pos],
            Argument::Relative(pos) => {
                self.memory[usize::try_from(isize::try_from(self.rb).unwrap() + pos).unwrap()]
            }
            Argument::Parameter(p) => p,
        }
    }

    fn wr(&mut self, arg: Argument, w: Word) {
        match arg {
            Argument::Absolute(pos) => {
                if pos >= self.memory.len() {
                    self.memory.resize(pos + 1, 0);
                }
                self.memory[pos] = w;
            }
            Argument::Relative(pos) => {
                let idx = usize::try_from(isize::try_from(self.rb).unwrap() + pos).unwrap();
                if idx >= self.memory.len() {
                    self.memory.resize(idx + 1, 0);
                }
                self.memory[idx] = w;
            }
            Argument::Parameter(_) => panic!("cannot write in parameter mode"),
        }
    }

    fn args(&self, n: usize, mode: u64) -> Vec<Argument> {
        (self.pc + 1..=self.pc + n)
            .fold((mode, Vec::with_capacity(n)), |(mode, mut v), idx| {
                let n = self.memory[idx];

                let arg = match mode % 10 {
                    0 => Argument::Absolute(n.try_into().unwrap()),
                    1 => Argument::Parameter(n),
                    2 => Argument::Relative(n.try_into().unwrap()),
                    _ => panic!("invalid argument mode"),
                };

                v.push(arg);

                (mode / 10, v)
            })
            .1
    }

    fn parse(program: &str) -> Vec<Word> {
        program
            .split(',')
            .map(|s| Word::from_str(s).unwrap())
            .collect::<Vec<_>>()
    }
}
