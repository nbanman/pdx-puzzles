extern crate core;

use std::ops::Mul;
use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = &'a str;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(25, 4);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn first_and_last(input: Input) -> (u64, u64) {
    let first = input.as_bytes().iter().position(|it| !it.is_ascii_digit()).unwrap();
    let first = input[0..first].parse().unwrap();
    let last = input.trim_end().as_bytes().iter().rposition(|it| !it.is_ascii_digit()).unwrap();
    let last = input.trim_end()[last + 1..].parse().unwrap();
    (first, last)
}

fn part1(input: Input) -> u64 {
    let (first, last) = first_and_last(input);
    2025 * first / last
}

fn part2(input: Input) -> u64 {
    let (first, last) = first_and_last(input);
    (10_000_000_000_000 * last).div_ceil(first)
}

fn part3(input: Input) -> u64 {
    input.get_numbers::<u64>()
        .tuples()
        .map(|(a, b)| a as f64 / b as f64)
        .fold(100f64, f64::mul) as u64
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(25, 4);
    assert_eq!(12980, part1(&input1));
    assert_eq!(2394789579159, part2(&input2));
    assert_eq!(220503433846, part3(&input3));
}

// Input parsed (29μs)
// 1. 12980 (4μs)
// 2. 2394789579159 (1μs)
// 3. 220503433846 (3μs)
// Total: 40μs