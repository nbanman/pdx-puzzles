use std::iter::successors;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = &'a str;
type Output = u64;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(20, 25).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: Input) -> Output {
    let divisor = 20201227;
    let (card_key, door_key) = input.get_numbers::<u64>().collect_tuple().unwrap();
    let loop_size = successors(Some(1), |&n| Some((n * 7) % divisor))
        .position(|n| n == card_key)
        .unwrap();
    successors(Some(door_key % divisor), |&n| Some((n * door_key) % divisor))
        .take(loop_size)
        .last()
        .unwrap()
}

#[test]
fn default() {
    let input = get_input(20, 25).unwrap();
    assert_eq!(296776, part1(&input));
}

// Input parsed (15μs)
// 1. 296776 (2ms)
// Total: 2ms
