use std::collections::VecDeque;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use lazy_regex::regex;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<Monkey>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(22, 11).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, Clone)]
struct Monkey {
    /// The items held by this monkey
    items: VecDeque<usize>,
    /// The operation that is applied when inspecting an item
    op: Op,
    /// The divisor to test the worry level with
    div: usize,
    /// The monkey to throw the item to if the test passes
    mt: usize,
    /// The monkey to throw the item to if the test fails
    mf: usize,
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add(usize),
    Mul(usize),
    Square,
}

fn parse_input(input: &str) -> Input {
    let re = regex!(
        r"Monkey \d+:
\s*Starting items: (?P<items>[\s\d,]+)
\s*Operation: new = old (?P<op>(\*|\+)) (?P<n>old|\d+)
\s*Test: divisible by (?P<div>\d+)
\s*If true: throw to monkey (?P<true>\d+)
\s*If false: throw to monkey (?P<false>\d+)"
    );
    input
        .split("\n\n")
        .map(|m| {
            let caps = re.captures(m).unwrap();
            let items = caps["items"]
                .split(", ")
                .map(str::parse)
                .map(Result::unwrap)
                .collect();
            let op = match (&caps["op"], &caps["n"]) {
                ("*", "old") => Op::Square,
                ("+", n) => Op::Add(n.parse().unwrap()),
                ("*", n) => Op::Mul(n.parse().unwrap()),
                _ => unreachable!(),
            };
            let div = caps["div"].parse().unwrap();
            let mt = caps["true"].parse().unwrap();
            let mf = caps["false"].parse().unwrap();
            Monkey {
                items,
                op,
                div,
                mt,
                mf,
            }
        })
        .collect()
}

fn solve<F>(mut monkeys: Input, rounds: usize, f: F) -> Output
    where
        F: Fn(usize) -> usize,
{
    let mut inspects = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            while let Some(old) = monkeys[m].items.pop_front() {
                let Monkey {
                    op, div, mt, mf, ..
                } = monkeys[m];
                let new = f(match op {
                    Op::Add(n) => old + n,
                    Op::Mul(n) => old * n,
                    Op::Square => old * old,
                });
                let to = if new % div == 0 { mt } else { mf };
                monkeys[to].items.push_back(new);
                inspects[m] += 1;
            }
        }
    }

    inspects.into_iter().sorted().rev().take(2).product()
}


fn part1(monkeys: Input) -> Output {
    solve(monkeys, 20, |w| w / 3)
}

fn part2(monkeys: Input) -> Output {
    let m: usize = monkeys.iter().map(|m| m.div).product();
    solve(monkeys, 10_000, |w| w % m)
}

#[test]
fn default() {
    let input = get_input(22, 11).unwrap();
    let input = parse_input(&input);
    assert_eq!(88208, part1(input.clone()));
    assert_eq!(21115867968, part2(input));
}

// Input parsed (620μs)
// 1. 88208 (16μs)
// 2. 21115867968 (4ms)
// Total: 4ms