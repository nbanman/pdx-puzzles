use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use advent::utilities::opcode::{Op, Parameters, Registers};
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input = (Vec<Trainer>, Vec<Code>);
type Output = usize;

#[derive(Debug, Clone)]
struct Code {
    opcode: usize,
    parameters: Parameters,
}

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
struct Trainer {
    before: Registers,
    code: Code,
    after: Registers,
}

impl Trainer {
    fn valid_ops(&self, ops: &FxHashSet<Op>) -> impl Iterator<Item = Op> {
        ops.into_iter()
            .filter(|op| {
                self.after == op.execute(&self.before, &self.code.parameters)
            })
            .copied()
    }
}

fn parse_input(input: &str) -> Input {
    let (trainers, code) = input.split_once("\n\n\n\n").unwrap();
    let trainers = trainers.get_numbers().chunks(12).into_iter()
        .map(|numbers| {
            let (ba, bb, bc, bd, opcode, a, b, c, aa, ab, ac, ad) = numbers.into_iter().collect_tuple().unwrap();
            Trainer {
                before: vec![ba, bb, bc, bd],
                code: Code { opcode, parameters: Parameters { a, b, c } },
                after: vec![aa, ab, ac, ad],
            }
        })
        .collect();

    let code = code.get_numbers().chunks(4).into_iter()
        .map(|numbers| {
            let (opcode, a, b, c) = numbers.into_iter().collect_tuple().unwrap();
            Code { opcode, parameters: Parameters { a, b, c } }
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
        .fold(vec![0; 4], |acc, code| {
            let op = translator.get(&code.opcode).unwrap();
            op.execute(&acc, &code.parameters)
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
