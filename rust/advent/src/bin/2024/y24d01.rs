use std::collections::HashMap;

use advent::utilities::get_input::get_input;
use itertools::{Either, Itertools};
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 1).unwrap();
    println!("Inputs loaded ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    input.get_numbers::<usize>().enumerate().partition_map(|(idx, n)| {
        if idx & 1 == 0 {
            Either::Left(n)
        } else {
            Either::Right(n)
        }
    })
}

fn part1(input: &str) -> usize {
    let (mut a, mut b) = parse_input(input);
    a.sort_unstable();
    b.sort_unstable();
    a.into_iter().zip(b.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

fn part2(input: &str) -> usize {
    let (a, b) = parse_input(input);
    let mut freq = HashMap::new();
    for n in b {
        freq.entry(n)
            .and_modify(|count| { *count += 1 })
            .or_insert(1);
    }
    a.into_iter().map(|n| n * freq.get(&n).unwrap_or(&0)).sum()   
}