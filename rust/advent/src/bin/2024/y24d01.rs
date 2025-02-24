use std::collections::HashMap;

use advent::utilities::get_input::get_input;
use itertools::{Either, Itertools};
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::stopwatch::{ReportDuration, Stopwatch},
};

type Input = (Vec<usize>, Vec<usize>);
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = parse_input(&get_input(24, 1).unwrap());
    println!("Inputs loaded ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input
        .get_numbers::<usize>()
        .enumerate()
        .partition_map(|(idx, n)| {
            if idx & 1 == 0 {
                Either::Left(n)
            } else {
                Either::Right(n)
            }
        })
}

fn part1(input: Input) -> Output {
    let (mut a, mut b) = input;
    a.sort_unstable();
    b.sort_unstable();
    a.into_iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum()
}

fn part2(input: Input) -> Output {
    let (a, b) = input;
    let mut freq = HashMap::new();
    for n in b {
        freq.entry(n).and_modify(|count| *count += 1).or_insert(1);
    }
    a.into_iter().map(|n| n * freq.get(&n).unwrap_or(&0)).sum()
}

#[test]
fn default() {
    let input = parse_input(&get_input(24, 1).unwrap());
    assert_eq!(1222801, part1(input.clone()));
    assert_eq!(22545250, part2(input));
}

#[test]
fn examples() {
    let inputs = [r"3   4
4   3
2   5
1   3
3   9
3   3
"];
    let input = parse_input(inputs[0]);
    assert_eq!(11, part1(input.clone()));
    assert_eq!(31, part2(input));
}

// Inputs loaded (21μs)
// 1. 1222801 (96μs)
// 2. 22545250 (112μs)
// Total: 233μs
