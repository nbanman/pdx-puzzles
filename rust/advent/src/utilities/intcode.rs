use std::collections::VecDeque;

use itertools::Itertools;
use utilities::parsing::get_numbers::ContainsNumbers;

const EXTRA_MEMORY: usize = 3_000;

#[derive(Debug, Clone)]
pub enum State {
    Input,
    Output(usize),
    Halted,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct IntCode {
    cursor: usize,
    base: usize,
    pub code: Vec<usize>,
    input: VecDeque<usize>,
}

impl From<&str> for IntCode {
    fn from(value: &str) -> Self {
        let initial_code = value.get_numbers().collect_vec();
        Self::new(&initial_code)
    }
}

impl IntCode {
    pub fn new(initial_code: &[usize]) -> Self {
        let mut code = Vec::with_capacity(initial_code.len() + EXTRA_MEMORY);
        code.extend(initial_code.iter());
        Self {
            cursor: 0,
            base: 0,
            code,
            input: VecDeque::new(),
        }
    }

    pub fn reset(&mut self) {
        self.cursor = 0;
        self.base = 0;
        self.input.clear();
    }

    pub fn run(&mut self) -> State {
        while let Some(&op) = self.code.get(self.cursor) {
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
                // jump true
                5 => {
                    let first = self.address(op / 100, 1);
                    self.cursor = if first == 0 {
                        self.cursor + 3
                    } else {
                        self.address(op / 1_000, 2)
                    }
                }
                // jump false
                6 => {
                    let first = self.address(op / 100, 1);
                    self.cursor = if first != 0 {
                        self.cursor + 3
                    } else {
                        self.address(op / 1_000, 2)
                    }
                }
                // less than
                7 => {
                    let first = self.address(op / 100, 1);
                    let second = self.address(op / 1_000, 2);
                    let third = self.address(op / 10_000, 3);
                    let value = (self.code[first] < self.code[second]) as usize;
                    self.code[third] = value;
                    self.cursor += 4;
                }
                // less than
                8 => {
                    let first = self.address(op / 100, 1);
                    let second = self.address(op / 1_000, 2);
                    let third = self.address(op / 10_000, 3);
                    let value = (self.code[first] == self.code[second]) as usize;
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
        State::Error(format!("cursor {} out of range", self.cursor))
    }

    fn address(&self, mode: usize, offset: usize) -> usize {
        match mode % 10 {
            0 => self.code[self.cursor + offset],
            1 => self.cursor + offset,
            2 => self.base + self.code[self.cursor + offset],
            d => panic!("{d} is not a valid mode"),
        }
    }
}
