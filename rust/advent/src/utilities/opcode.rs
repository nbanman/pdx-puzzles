pub type Registers = Vec<usize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Op {
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti,
    Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr
}

#[derive(Clone, Debug)]
pub struct Parameters {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

impl Op {
    pub const VARIANTS: [Self; 16] = [Self::Addr, Self::Addi, Self::Mulr, Self::Muli, Self::Banr, Self::Bani, Self::Borr,
        Self::Bori, Self::Setr, Self::Seti, Self::Gtir, Self::Gtri, Self::Gtrr, Self::Eqir, Self::Eqri, Self::Eqrr];

    pub fn execute(&self, reg: &Registers, parameters: &Parameters) -> Registers {
        let mut output = reg.clone();
        let &Parameters { a, b, c } = parameters;
        output[c] = match self {
            Op::Addr => reg[a] + reg[b],
            Op::Addi => reg[a] + b,
            Op::Mulr => reg[a] * reg[b],
            Op::Muli => reg[a] * b,
            Op::Banr => reg[a] & reg[b],
            Op::Bani => reg[a] & b,
            Op::Borr => reg[a] | reg[b],
            Op::Bori => reg[a] | b,
            Op::Setr => reg[a],
            Op::Seti => a,
            Op::Gtir => if a > reg[b] { 1 } else { 0 },
            Op::Gtri => if reg[a] > b { 1 } else { 0 },
            Op::Gtrr => if reg[a] > reg[b] { 1 } else { 0 },
            Op::Eqir => if a == reg[b] { 1 } else { 0 },
            Op::Eqri => if reg[a] == b { 1 } else { 0 },
            Op::Eqrr => if reg[a] == reg[b] { 1 } else { 0 },
        };
        output
    }
}

impl From<&str> for Op {
    fn from(value: &str) -> Self {
        match value {
            "addr" => Op::Addr,
            "addi" => Op::Addi,
            "mulr" => Op::Mulr,
            "muli" => Op::Muli,
            "banr" => Op::Banr,
            "bani" => Op::Bani,
            "borr" => Op::Borr,
            "bori" => Op::Bori,
            "setr" => Op::Setr,
            "seti" => Op::Seti,
            "gtir" => Op::Gtir,
            "gtri" => Op::Gtri,
            "gtrr" => Op::Gtrr,
            "eqir" => Op::Eqir,
            "eqri" => Op::Eqri,
            "eqrr" => Op::Eqrr,
            _ => panic!()
        }
    }
}