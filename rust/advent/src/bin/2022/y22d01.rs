use std::collections::BinaryHeap;

use advent::utilities::get_input::get_input;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input = BinaryHeap<u32>;
type Output = u32;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(22, 1).unwrap();
    let mut input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&mut input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.split("\n\n")
        .map(|stanza| stanza.get_numbers::<u32>().sum())
        .collect()
}

fn part1(input: &Input) -> Output {
    *input.peek().unwrap()
}

fn part2(input: &mut Input) -> Output {
    input.iter().take(3).sum()
}

#[test]
fn default() {
    let input = get_input(22, 1).unwrap();
    let mut input = parse_input(&input);
    assert_eq!(71300, part1(&input));
    assert_eq!(209691, part2(&mut input));
}

// Input parsed (49μs)
// 1. 71300 (5μs)
// 2. 209691 (1μs)
// Total: 59μs