use std::str::FromStr;

#[derive(Clone)]
pub struct Intcode {
    memory: Vec<i64>,
    input: i64,
}

impl Intcode {
    pub fn new(program: &str) -> Intcode {
        Intcode {
            memory: Intcode::load(program),
            input: 0,
        }
    }

    pub fn input(mut self, input: i64) -> Intcode {
        self.input = input;
        self
    }

    pub fn update(mut self, pos: usize, val: i64) -> Intcode {
        self.memory[pos] = val;
        self
    }

    pub fn value(&self, pos: usize) -> i64 {
        self.memory[pos]
    }

    pub fn run(&mut self) -> Vec<i64> {
        let mut pc = 0;

        loop {
            let ra = self.memory[pc + 1] as usize;
            let rb = self.memory[pc + 2] as usize;
            let rd = self.memory[pc + 3] as usize;

            match self.memory[pc] {
                1 => self.memory[rd] = self.memory[ra] + self.memory[rb],
                2 => self.memory[rd] = self.memory[ra] * self.memory[rb],
                99 => break,
                _ => panic!("unexpected opcode: {}", self.memory[pc]),
            }

            pc += 4;
        }

        vec![]
    }

    fn load(program: &str) -> Vec<i64> {
        program
            .split(',')
            .map(|s| i64::from_str(s).unwrap())
            .collect::<Vec<_>>()
    }
}
