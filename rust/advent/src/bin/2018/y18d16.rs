use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input = (Vec<Trainer>, Vec<Code>);
type Output = usize;
type Registers = [usize; 4];

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(18, 16).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, Clone)]
struct Code {
    opcode: usize,
    a: usize,
    b: usize,
    c: usize,
}

#[derive(Debug, Clone)]
struct Trainer {
    before: Registers,
    code: Code,
    after: Registers,
}

impl Trainer {
    fn valid_ops(&self, ops: &FxHashSet<Op>) -> impl Iterator<Item = Op> {
        ops.into_iter().filter(|op| self.after == op.execute(&self.before, &self.code)).copied()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Op {
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti,
    Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr
}

impl Op {
    const VARIANTS: [Self; 16] = [Self::Addr, Self::Addi, Self::Mulr, Self::Muli, Self::Banr, Self::Bani, Self::Borr,
        Self::Bori, Self::Setr, Self::Seti, Self::Gtir, Self::Gtri, Self::Gtrr, Self::Eqir, Self::Eqri, Self::Eqrr];
    
    fn execute(&self, reg: &Registers, code: &Code) -> Registers {
        let mut output = reg.clone();
        let &Code { a, b, c, .. } = code;
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

fn parse_input(input: &str) -> Input {
    let (trainers, code) = input.split_once("\n\n\n\n").unwrap();
    let trainers = trainers.get_numbers().chunks(12).into_iter()
        .map(|numbers| {
            let (ba, bb, bc, bd, opcode, a, b, c, aa, ab, ac, ad) = numbers.into_iter().collect_tuple().unwrap();
            Trainer {
                before: [ba, bb, bc, bd],
                code: Code { opcode, a, b, c },
                after: [aa, ab, ac, ad],
            }
        })
        .collect();

    let code = code.get_numbers().chunks(4).into_iter()
        .map(|numbers| {
            let (opcode, a, b, c) = numbers.into_iter().collect_tuple().unwrap();
            Code { opcode, a, b, c }
        })
        .collect();
        
    (trainers, code)
}

fn part1(input: &Input) -> Output {
    let (trainers, _) = input;
    let ops: FxHashSet<Op> = Op::VARIANTS.into_iter().collect();
    trainers.into_iter()
        .filter(|trainer| trainer.valid_ops(&ops).count() >= 3)
        .count()
}

fn part2(input: &Input) -> Output {
    let (trainers, code) = input;
    let mut trainers = trainers.clone();
    let mut ops: FxHashSet<Op> = Op::VARIANTS.into_iter().collect();
    let mut translator: FxHashMap<usize, Op> = FxHashMap::default();
    while !ops.is_empty() {
        let hot_local_singles: Vec<(Trainer, Op)> = trainers.iter()
            .map(|trainer| (trainer, trainer.valid_ops(&ops).collect_vec()))
            .filter(|(_, valid_ops)| valid_ops.len() == 1)
            .map(|(trainer, op_vec)| (trainer.clone(), op_vec[0]))
            .collect();
        for (trainer, op) in hot_local_singles {
            if !translator.contains_key(&trainer.code.opcode) {
                translator.insert(trainer.code.opcode, op); 
                ops.remove(&op);
                trainers = trainers.into_iter().filter(|it| it.code.opcode != trainer.code.opcode).collect();
            }
        }
    }
    code.iter()
        .fold([0; 4], |acc, code| {
            let op = translator.get(&code.opcode).unwrap();
            op.execute(&acc, code)
        })[0]
}

#[test]
fn default() {
    let input = get_input(18, 16).unwrap();
    let input = parse_input(&input);
    assert_eq!(529, part1(&input));
    assert_eq!(573, part2(&input));
}

// Input parsed (279μs)
// 1. 529 (134μs)
// 2. 573 (918μs)
// Total: 1ms
