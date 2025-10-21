use std::ops::{Index, IndexMut};

type Registers = [i64; 4];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Register(usize);

impl From<char> for Register {
    fn from(c: char) -> Self {
        Self((c as u8 - b'a') as usize)
    }
}

impl From<&str> for Register {
    fn from(s: &str) -> Self {
        let c = s.chars().next().unwrap();
        Self((c as u8 - b'a') as usize)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Value(i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Arg {
    Register(Register),
    Value(Value),
}

impl From<&str> for Arg {
    fn from(value: &str) -> Self {
        match value.chars().next().unwrap() {
            c if c == '-' || c.is_numeric() => Arg::Value(Value(value.parse::<i64>().unwrap())),
            c => Arg::Register(c.into()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Op {
    Cpy(Arg, Arg),
    Dec(Register),
    Inc(Register),
    Jnz(Arg, Arg),
    Out(Register),
    Tgl(Arg),
}

impl From<&str> for Op {
    fn from(s: &str) -> Self {
        let mut tokens = s.split(' ');
        let op = tokens.next().unwrap();
        match op {
            "cpy" => {
                let a: Arg = tokens.next().unwrap().into();
                let b: Arg = tokens.next().unwrap().into();
                Op::Cpy(a, b)
            }
            "dec" => {
                let a: Register = tokens.next().unwrap().into();
                Op::Dec(a)
            }
            "inc" => {
                let a: Register = tokens.next().unwrap().into();
                Op::Inc(a)
            }
            "jnz" => {
                let a: Arg = tokens.next().unwrap().into();
                let b: Arg = tokens.next().unwrap().into();
                Op::Jnz(a, b)
            }
            "out" => {
                let a: Register = tokens.next().unwrap().into();
                Op::Out(a)
            }
            "tgl" => {
                let a: Arg = tokens.next().unwrap().into();
                Op::Tgl(a)
            }
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Assembunny {
    registers: Registers,
    instructions: Vec<Op>,
    cursor: i64,
    toggle: Vec<bool>,
}

impl Assembunny {
    pub fn run(&mut self, limit: Option<usize>) {

        let mut count = 0;
        let has_limit = limit.is_some();
        let limit = limit.unwrap_or(usize::MAX);

        while (0..self.instructions.len() as i64).contains(&self.cursor) {
            let cursor = self.cursor as usize;
            if self.toggle[cursor] {
                match self.instructions[cursor] {
                    Op::Cpy(a, b) => { self.jnz(a, b); },
                    Op::Dec(a) => { self.inc(a); }
                    Op::Inc(a) => { self.dec(a); }
                    Op::Jnz(a, b) => { self.cpy(a, b); },
                    Op::Out(_) => {} // no-op; never called
                    Op::Tgl(a) => {
                        match a {
                            Arg::Register(register) => { self.inc(register) }
                            Arg::Value(_) => unreachable!()
                        }
                    }
                }
            } else {
                match self.instructions[cursor] {
                    Op::Cpy(a, b) => { self.cpy(a, b); },
                    Op::Dec(a) => { self.dec(a); }
                    Op::Inc(a) => { self.inc(a); }
                    Op::Jnz(a, b) => { self.jnz(a, b); },
                    Op::Out(_) => {} // no-op; never called
                    Op::Tgl(a) => { self.tgl(a); }
                }
            }
            self.cursor += 1;
            if has_limit {
                count += 1;
                if count == limit {
                    break;
                }
            }
        }
    }

    fn value_of(&self, arg: Arg) -> i64 {
        match arg {
            Arg::Register(Register(register)) => self.registers[register],
            Arg::Value(Value(value)) => value,
        }
    }

    fn cpy(&mut self, a: Arg, b: Arg) {
        match b {
            Arg::Register(Register(register)) => {
                self.registers[register] = self.value_of(a);
            }
            Arg::Value(_) => unreachable!(),
        }
    }

    fn dec(&mut self, reg: Register) {
        self.registers[reg.0] -= 1;
    }

    fn inc(&mut self, reg: Register) {
        self.registers[reg.0] += 1;
    }

    fn jnz(&mut self, a: Arg, b: Arg) {
        if self.value_of(a) != 0 {
            self.cursor += self.value_of(b) - 1;
        }
    }

    fn tgl(&mut self, a: Arg) {
        let tgl_index = self.cursor + self.value_of(a);
        if (0..self.instructions.len() as i64).contains(&tgl_index) {
            let tgl_index = tgl_index as usize;
            self.toggle[tgl_index] = !self.toggle[tgl_index];
        }
    }
}

impl From<&str> for Assembunny {
    fn from(s: &str) -> Self {
        let instructions: Vec<Op> = s.lines().map(Op::from).collect();
        let toggle = vec![false; instructions.len()];
        Self { registers: [0; 4], instructions, cursor: 0, toggle }
    }
}

impl Index<char> for Assembunny {
    type Output = i64;

    fn index(&self, index: char) -> &Self::Output {
        &self.registers[Register::from(index).0]
    }
}

impl IndexMut<char> for Assembunny {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        self.registers.get_mut(Register::from(index).0).unwrap()
    }
}

// Input parsed (29μs)
// 1. 318117 (2ms)
// 2. 9227771 (65ms)
// Total: 68ms