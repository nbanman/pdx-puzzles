use std::collections::VecDeque;

use itertools::Itertools;
use utilities::parsing::get_numbers::ContainsNumbers;

const EXTRA_MEMORY: usize = 3_000;

#[derive(Debug, Clone)]
pub enum State {
    Input,
    Output(i64),
    Halted,
}

#[derive(Debug, Clone)]
pub struct IntCode {
    cursor: i64,
    base: i64,
    pub code: Vec<i64>,
    input: VecDeque<i64>,
}

impl From<&str> for IntCode {
    fn from(value: &str) -> Self {
        let initial_code = value.get_numbers().collect_vec();
        Self::new(&initial_code)
    }
}

impl IntCode {
    pub fn new(initial_code: &[i64]) -> Self {
        let mut code = Vec::with_capacity(initial_code.len() + EXTRA_MEMORY);
        code.extend(initial_code.iter());
        code.resize(initial_code.len() + EXTRA_MEMORY, 0);
        Self {
            cursor: 0,
            base: 0,
            code,
            input: VecDeque::new(),
        }
    }

    pub fn input(&mut self, value: i64) {
        self.input.push_back(value);
    }

    pub fn input_ascii(&mut self, ascii: &str) {
        self.input.extend(ascii.bytes().map(|b| b as i64));
    }

    pub fn reset(&mut self) {
        self.cursor = 0;
        self.base = 0;
        self.input.clear();
    }

    pub fn run_while_able(&mut self) -> (State, VecDeque<i64>) {
        let mut output: VecDeque<i64> = VecDeque::new();
        loop {
            match self.run() {
                State::Input => { return (State::Input, output); },
                State::Output(value) => { output.push_back(value); },
                State::Halted => { return (State::Halted, output); },
            }
        }
    }

    pub fn run(&mut self) -> State {
        while let Some(&op) = self.code.get(self.cursor as usize) {
            match op % 100 {
                // add
                1 => {
                    let first = self.address(op / 100, 1);
                    let second = self.address(op / 1_000, 2);
                    let third = self.address(op / 10_000, 3);
                    self.code[third] = self.code[first] + self.code[second];
                    self.cursor += 4;
                }
                // multiply
                2 => {
                    let first = self.address(op / 100, 1);
                    let second = self.address(op / 1_000, 2);
                    let third = self.address(op / 10_000, 3);
                    self.code[third] = self.code[first] * self.code[second];
                    self.cursor += 4;
                }
                // read
                3 => {
                    let Some(value) = self.input.pop_front() else {
                        return State::Input;
                    };
                    let first = self.address(op / 100, 1);
                    self.code[first] = value;
                    self.cursor += 2;
                }
                // write
                4 => {
                    let first = self.address(op / 100, 1);
                    let value = self.code[first];
                    self.cursor += 2;
                    return State::Output(value);
                }
                // jump if > 0
                5 => {
                    let first = self.address(op / 100, 1);
                    self.cursor = if self.code[first] == 0 {
                        self.cursor + 3
                    } else {
                        let second = self.address(op / 1_000, 2);
                        self.code[second as usize]
                    };
                }
                // jump if 0
                6 => {
                    let first = self.address(op / 100, 1);
                    self.cursor = if self.code[first] != 0 {
                        self.cursor + 3
                    } else {
                        let second = self.address(op / 1_000, 2);
                        self.code[second as usize]
                    }
                }
                // less than
                7 => {
                    let first = self.address(op / 100, 1);
                    let second = self.address(op / 1_000, 2);
                    let third = self.address(op / 10_000, 3);
                    let value = (self.code[first] < self.code[second]) as i64;
                    self.code[third] = value;
                    self.cursor += 4;
                }
                // less than
                8 => {
                    let first = self.address(op / 100, 1);
                    let second = self.address(op / 1_000, 2);
                    let third = self.address(op / 10_000, 3);
                    let value = (self.code[first] == self.code[second]) as i64;
                    self.code[third] = value;
                    self.cursor += 4;
                }
                // change base
                9 => {
                    let first = self.address(op / 100, 1);
                    self.base += self.code[first];
                    self.cursor += 2;
                }
                _ => return State::Halted,
            }
        }
        unreachable!()
    }

    fn address(&self, mode: i64, offset: i64) -> usize {
        match mode % 10 {
            0 => self.code[(self.cursor + offset) as usize] as usize,
            1 => (self.cursor + offset) as usize,
            2 => (self.base + self.code[(self.cursor + offset) as usize]) as usize,
            d => panic!("{d} is not a valid mode"),
        }
    }
}
