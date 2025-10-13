use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = (i64, i64);
type Output = i64;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(17, 8).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, Copy, Clone)]
struct Instruction<'a> {
    operand: &'a str,
    amount: i64,
    con_var: &'a str,
    con_op: &'a str,
    con_amt: i64,
}

impl<'a> Instruction<'a> {
    fn execute(&self, register: &mut FxHashMap<&'a str, i64>) -> i64 {
        let con_val = *register.get(self.con_var).unwrap_or(&0);
        let meets_condition = match self.con_op {
            "<=" => con_val <= self.con_amt,
            "<" => con_val < self.con_amt,
            "==" => con_val == self.con_amt,
            "!=" => con_val != self.con_amt,
            ">=" => con_val >= self.con_amt,
            ">" => con_val > self.con_amt,
            _ => unreachable!(),
        };
        if meets_condition {
            register.entry(self.operand)
                .and_modify(|v| *v += self.amount)
                .or_insert(self.amount);
        }
        *register.get(self.operand).unwrap_or(&0)
    }
}

fn parse_input(input: &str) -> Input {
    let instructions = input.lines()
        .map(|line| {
            let (operand, operation, amount, _, con_var, con_op, con_amt) = line.split(' ').collect_tuple().unwrap();
            let amount: i64 = amount.parse().unwrap();
            let amount = if operation == "dec" { -amount } else { amount };
            let con_amt: i64 = con_amt.parse().unwrap();
            Instruction { operand, amount, con_var, con_op, con_amt }
        })
        .collect_vec();
    let mut register = FxHashMap::default();
    let highest = instructions
        .iter()
        .map(|instruction| instruction.execute(&mut register))
        .max()
        .unwrap();
    (*register.values().max().unwrap(), highest)
}

fn part1(input: Input) -> Output {
    input.0
}

fn part2(input: Input) -> Output {
    input.1
}

#[test]
fn default() {
    let input = get_input(17, 8).unwrap();
    let input = parse_input(&input);
    assert_eq!(6343, part1(input));
    assert_eq!(7184, part2(input));
}

// Input parsed (210μs)
// 1. 6343 (5μs)
// 2. 7184 (2μs)
// Total: 221μs