use std::collections::HashMap;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::structs::{grid::Grid2, stopwatch::{ReportDuration, Stopwatch}};

type Input = Vec<HashMap<char, usize>>;
type Output = String;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(16, 6).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    Grid2::try_from(input).unwrap()
        .columns()
        .map(move |column| column.into_iter().copied().counts())
        .collect_vec()
}

fn part1(columns: &Input) -> Output {
    columns.iter()
        .map(|column| {
            column.iter()
                .max_by_key(|&(_, &count)| count)
                .unwrap()
                .0
        })
    .collect()
}

fn part2(columns: &Input) -> Output {
    columns.iter()
        .map(|column| {
            column.iter()
                .min_by_key(|&(_, &count)| count)
                .unwrap()
                .0
        })
    .collect()
}

#[test]
fn default() {
    let input = get_input(16, 6).unwrap();
    let input = parse_input(&input);
    assert_eq!("asvcbhvg".to_string(), part1(&input));
    assert_eq!("odqnikqv".to_string(), part2(&input));
}

// Input parsed (114μs)
// 1. asvcbhvg (9μs)
// 2. odqnikqv (3μs)
// Total: 129μs
