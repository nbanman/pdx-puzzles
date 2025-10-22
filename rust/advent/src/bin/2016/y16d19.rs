use std::cmp::{max, min};

use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = usize;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(16, 19).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.parse().unwrap()
}

fn part1(elves: Input) -> Output {
    let exponent = (f64::log10(elves as f64) / f64::log10(2.0)) as u32;
    (elves - 2usize.pow(exponent)) * 2 + 1
}

fn part2(elves: Input) -> Output {
    let exponent = (f64::log10(elves as f64) / f64::log10(3.0)) as u32;
    let last_up = 3usize.pow(exponent);
    let diff = elves - last_up;
    let ones = min(diff, last_up);
    let twos = max(diff - ones, 0);
    ones + twos * 2
}

#[test]
fn default() {
    let input = get_input(16, 19).unwrap();
    let input = parse_input(&input);
    assert_eq!(1816277, part1(input));
    assert_eq!(1410967, part2(input));
}

// Input parsed (18μs)
// 1. 1816277 (12μs)
// 2. 1410967 (2μs)
// Total: 35μs
