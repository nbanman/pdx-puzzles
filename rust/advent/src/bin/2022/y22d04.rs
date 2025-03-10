use advent::utilities::get_input::get_input;
use itertools::Itertools;
use std::ops::RangeInclusive;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::stopwatch::{ReportDuration, Stopwatch},
};

type Int = i16;
type Input = Vec<(RangeInclusive<Int>, RangeInclusive<Int>)>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(22, 4).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input
        .get_numbers()
        .tuples()
        .map(|(low1, high1, low2, high2)| (low1..=high1, low2..=high2))
        .collect()
}

fn contains_all(a: &RangeInclusive<Int>, b: &RangeInclusive<Int>) -> bool {
    a.start() <= b.start() && a.end() >= b.end()
}

fn overlaps(a: &RangeInclusive<Int>, b: &RangeInclusive<Int>) -> bool {
    if a.start() <= b.start() {
        a.end() >= b.start()
    } else {
        b.end() >= a.start()
    }
}

fn part1(ranges: &Input) -> Output {
    ranges
        .iter()
        .filter(|(left, right)| contains_all(left, right) || contains_all(right, left))
        .count()
}

fn part2(ranges: &Input) -> Output {
    ranges
        .iter()
        .filter(|(left, right)| overlaps(left, right))
        .count()
}

#[test]
fn default() {
    let input = get_input(22, 4).unwrap();
    let input = parse_input(&input);
    assert_eq!(605, part1(&input));
    assert_eq!(914, part2(&input));
}

// Input parsed (63μs)
// 1. 605 (9μs)
// 2. 914 (2μs)
// Total: 77μs
