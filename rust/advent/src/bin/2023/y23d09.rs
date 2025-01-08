use std::iter::successors;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Output = i64;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(23, 9).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn patterns(input: Input) -> impl Iterator<Item = Vec<i64>> + use<'_> {
    input
        .lines()
        .map(|line| line.split(' ').map(|n| n.parse().unwrap()).collect())

}

fn find_next(pattern: Vec<i64>) -> i64 {
    successors(Some(pattern), |it| {
        Some(it.iter().tuple_windows().map(|(a, b)| b - a).collect())
    })
    .take_while(|next| next.iter().any(|it| it != &0) )
    .map(|it| *it.last().unwrap())
    .sum()
}

fn part1(input: Input) -> Output {
    patterns(input)
        .map(find_next)
        .sum()
}

fn part2(input: Input) -> Output {
    patterns(input)
        .map(|mut pattern| {
            pattern.reverse();
            find_next(pattern)
        })
        .sum()
}

#[test]
fn default() {
    let input = get_input(23, 9).unwrap();
    assert_eq!(1974913025, part1(&input));
    assert_eq!(884, part2(&input));
}

// Input parsed (20μs)
// 1. 1974913025 (152μs)
// 2. 884 (139μs)
// Total: 314μs