use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<Command>;
type Output = usize;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Op { Set, Sub, Mul, Jnz}

impl From<&str> for Op {
    fn from(value: &str) -> Self {
        match value {
            "set" => Op::Set,
            "sub" => Op::Sub,
            "mul" => Op::Mul,
            "jnz" => Op::Jnz,
            _ => { panic!("invalid op name"); },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Arg {
    Reg(char),
    Val(i64),
}

impl Arg {
    fn value(&self, register: &FxHashMap<char, i64>) -> i64 {
        match *self {
            Arg::Reg(reg) => *register.get(&reg).unwrap_or(&0),
            Arg::Val(val) => val,
        }
    }
}

impl From<&str> for Arg {
    fn from(value: &str) -> Self {
        value.parse::<i64>()
            .map(|i| Self::Val(i))
            .unwrap_or(Arg::Reg(value.chars().next().unwrap()))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Command {
    op: Op,
    arg1: Arg,
    arg2: Arg,
}


fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(17, 23).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.split([' ', '\n']).chunks(3).into_iter()
        .map(|chunk| {
            let (op, arg1, arg2) = chunk.collect_tuple()
                .map(|(op, arg1, arg2)| (op.into(), arg1.into(), arg2.into()))
                .unwrap();
            Command { op, arg1, arg2 }
        })
        .collect()
}

pub fn is_prime(x: i64) -> bool {
    match x {
        n if n <= 1 => false,
        n if n <= 3 => true,
        n if n & 1 == 0 => false,
        n if n % 3 == 0 => false,
        _ => {
            let limit = x.isqrt();
            !(5..=limit).step_by(6).any(|i| x % i == 0 || x % (i + 2) == 0)
        }
    }
}

fn part1(commands: &Input) -> Output {
    let mut register: FxHashMap<char, i64> = FxHashMap::default();
    let mut index: i64 = 0;
    let mut p1 = 0;
    while (0i64..commands.len() as i64).contains(&index) {
        let command = commands[index as usize];
        match command.op {
            Op::Set => {
                let Arg::Reg(reg) = command.arg1 else { panic!(); };
                register.insert(reg, command.arg2.value(&register));
            },
            Op::Sub => {
                let Arg::Reg(reg) = command.arg1 else { panic!(); };
                register.insert(reg, command.arg1.value(&register) - command.arg2.value(&register));
            },
            Op::Mul => {
                p1 += 1;
                let Arg::Reg(reg) = command.arg1 else { panic!(); };
                register.insert(reg, command.arg1.value(&register) * command.arg2.value(&register));
            },
            Op::Jnz => {
                if command.arg1.value(&register) != 0 {
                    index += command.arg2.value(&register);
                    continue;
                }
            },
        }
        index += 1;
    }
    p1
}

fn part2(commands: &Input) -> Output {
    let Arg::Val(b) = commands[0].arg2 else { panic!(); };
    let b = b * 100 + 100_000;
    (b..=b + 17_000).step_by(17).filter(|&x| !is_prime(x)).count()
}

#[test]
fn default() {
    let input = get_input(17, 23).unwrap();
    let input = parse_input(&input);
    assert_eq!(3025, part1(&input));
    assert_eq!(915, part2(&input));
}

// Input parsed (15μs)
// 1. 3025 (152μs)
// 2. 915 (29μs)
// Total: 199μs