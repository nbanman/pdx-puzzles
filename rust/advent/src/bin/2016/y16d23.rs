use advent::utilities::{assembunny::Assembunny, get_input::get_input};
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};
use std::ops::Mul;

type Input<'a> = &'a str;
type Output = i64;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(16, 23).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: Input) -> Output {
    let mut asmb: Assembunny = input.into();
    asmb['a'] = 7;
    asmb.run(None);
    asmb['a']
}

fn part2(input: Input) -> Output {
    let (a, b) = input.lines().skip(19).take(2)
        .map(|line| line.get_numbers::<i64>().next().unwrap())
        .collect_tuple()
        .unwrap();
    (1..=12).reduce(i64::mul).unwrap() + a * b
}

#[test]
fn default() {
    let input = get_input(16, 23).unwrap();
    assert_eq!(12748, part1(&input));
    assert_eq!(479009308, part2(&input));
}

// Input parsed (16μs)
// 1. 12748 (184μs)
// 2. 479009308 (3μs)
// Total: 206μs
