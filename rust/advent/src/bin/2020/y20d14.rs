use std::collections::HashMap;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::stopwatch::{ReportDuration, Stopwatch},
};

type Input = Vec<Instruction>;
type Output = u64;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(20, 14).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

enum Instruction {
    Mask { one: u64, zero: u64, x: u64 },
    Mem { register: u64, value: u64 },
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let (id, remaining) = value.split_once(['[', ' ']).unwrap();
        match id {
            "mask" => {
                let (_, value) = remaining.split_at(2);
                let make_mask = |predicate: fn(u8) -> bool| {
                    value
                        .as_bytes()
                        .iter()
                        .enumerate()
                        .rfold(0, |acc, (idx, &b)| {
                            if predicate(b) {
                                acc + (1 << (value.len() - idx - 1))
                            } else {
                                acc
                            }
                        })
                };
                let one = make_mask(|b| b == b'1');
                let zero = make_mask(|b| b != b'0');
                let x = make_mask(|b| b == b'X');
                Self::Mask { one, zero, x }
            }
            "mem" => {
                let (register, value) = remaining.get_numbers().collect_tuple().unwrap();
                Self::Mem { register, value }
            }
            s => {
                panic!("{s} not a recognized instruction type");
            }
        }
    }
}

fn masked_registers(register: u64, one_mask: u64, x_mask: u64) -> Vec<u64> {
    let one_applied = register | one_mask;
    (0..36).fold(vec![0], |acc, place| {
        if 1 & (x_mask >> place) == 1 {
            acc.into_iter().flat_map(|n| [n, n + (1 << place)]).collect()
        } else if 1 & (one_applied >> place) == 1 {
            acc.into_iter().map(|n| n + (1 << place)).collect()
        } else {
            acc
        }
    })
}

fn masked_value(value: u64, one_mask: u64, zero_mask: u64) -> u64 {
    (value | one_mask) & zero_mask
}

fn parse_input(input: &str) -> Input {
    input.lines().map(Instruction::from).collect()
}

fn part1(instructions: &Input) -> Output {
    let mut registers = HashMap::new();
    let mut one_mask = 0;
    let mut zero_mask = 0;
    for instruction in instructions {
        match *instruction {
            Instruction::Mask { one, zero, x: _ } => {
                one_mask = one;
                zero_mask = zero;
            }
            Instruction::Mem { register, value } => {
                registers.insert(register, masked_value(value, one_mask, zero_mask));
            }
        }
    }
    registers.values().copied().sum()
}

fn part2(instructions: &Input) -> Output {
    let mut registers = HashMap::new();
    let mut one_mask = 0;
    let mut x_mask = 0;
    for instruction in instructions {
        match *instruction {
            Instruction::Mask { one, zero: _, x } => {
                one_mask = one;
                x_mask = x;
            }
            Instruction::Mem { register, value } => {
                for reg in masked_registers(register, one_mask, x_mask) {
                    registers.insert(reg, value);
                }
            }
        }
    }
    registers.values().copied().sum()
}

#[test]
fn default() {
    let input = get_input(20, 14).unwrap();
    let input = parse_input(&input);
    assert_eq!(11926135976176, part1(&input));
    assert_eq!(4330547254348, part2(&input));
}

// Input parsed (75μs)
// 1. 11926135976176 (38μs)
// 2. 4330547254348 (4ms)
// Total: 4ms