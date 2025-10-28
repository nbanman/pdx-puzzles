use std::ops::Add;
use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(15, 25).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}
fn part1(input: Input) -> Output {
    let (row, col) = input.get_numbers::<usize>().collect_tuple().unwrap();
    let extra = (row..row + col).reduce(usize::add).unwrap() - row;
    let place = (1..row).reduce(usize::add).unwrap() + 1 + extra;
    (2..=place).fold(20_151_125, |acc, _| (acc * 252_533) % 33_554_393)
}

#[test]
fn default() {
    let input = get_input(15, 25).unwrap();
    assert_eq!(8997277, part1(&input));
}

// Input parsed (16μs)
// 1. 8997277 (58ms)
// Total: 58ms
