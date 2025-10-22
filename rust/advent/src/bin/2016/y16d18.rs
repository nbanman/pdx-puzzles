use std::iter::successors;

use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Row = Vec<bool>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(16, 18).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

// store the row as a u128, add a dummy 
fn parse_input(input: &str) -> Row {
    input.chars().map(|c| c == '.').collect()
}

fn next(row: &Row) -> Row {
    (0..row.len()).map(|i| {
        match i {
            0 => row[1],
            i if i == row.len() - 1 => row[i - 1],
            i => row[i - 1] == row[i + 1],
        }
    }).collect()
}

fn solve(row: Row, rows: usize) -> Output {
    successors(Some(row), |acc| Some(next(acc)))
        .take(rows)
        .map(|row| row.iter().filter(|&&x| x).count())
        .sum()
}

fn part1(row: Row) -> Output {
    solve(row, 40)
}

fn part2(row: Row) -> Output {
    solve(row, 400_000)
}

#[test]
fn default() {
    let input = get_input(16, 18).unwrap();
    let input = parse_input(&input);
    assert_eq!(1987, part1(input.clone()));
    assert_eq!(19984714, part2(input));
}

// Input parsed (16μs)
// 1. 1987 (9μs)
// 2. 19984714 (31ms)
// Total: 31ms
