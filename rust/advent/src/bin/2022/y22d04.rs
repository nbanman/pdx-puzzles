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

fn contained(a: &RangeInclusive<Int>, b: &RangeInclusive<Int>) -> bool {
    (a.start() <= b.start() && a.end() >= b.end()) ||
        (b.start() <= a.start() && b.end() >= a.end())
}

fn overlaps(a: &RangeInclusive<Int>, b: &RangeInclusive<Int>) -> bool {
    if a.start() <= b.start() {
        a.end() >= b.start()
    } else {
        b.end() >= a.start()
    }
}

fn solve<F>(ranges: &Input, predicate: F) -> Output 
where F: Fn(&RangeInclusive<Int>, &RangeInclusive<Int>) -> bool
{
    ranges
        .iter()
        .filter(|(left, right)| predicate(left, right))
        .count()
}

fn part1(ranges: &Input) -> Output {
    solve(ranges, contained)
}

fn part2(ranges: &Input) -> Output {
    solve(ranges, overlaps)
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
