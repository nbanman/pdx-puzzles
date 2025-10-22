use std::iter::successors;

use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Row = u128;
type Output = u32;

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
fn parse_input(input: &str) -> (Row, usize) {
    let row = input.chars().fold(0u128, |acc, c| (acc << 1) + if c == '.' { 1 } else { 0 });
    (row, input.len())
}

fn next(row: Row, len: usize) -> Row {
    (0..len).fold(0, |acc, i| {
        let n = match i {
            0 => (row >> 1) & 1,
            i if i == len - 1 => (row >> (len - 2)) & 1,
            i => if ((row >> (i + 1)) & 1) == ((row >> (i - 1)) & 1) { 1 } else { 0 },
        };
        (acc << 1) + n
    })
}

fn solve(row: Row, len: usize, rows: usize) -> Output {
    successors(Some(row), |acc| Some(next(*acc, len)))
        .take(rows)
        .map(|row| row.count_ones())
        .sum()
}

fn part1((row, len): (Row, usize)) -> Output {
    solve(row, len, 40)
}

fn part2((row, len): (Row, usize)) -> Output {
    solve(row, len, 400_000)
}

#[test]
fn default() {
    let input = get_input(16, 18).unwrap();
    let input = parse_input(&input);
    assert_eq!(1987, part1(input.clone()));
    assert_eq!(19984714, part2(input));
}

// Input parsed (15μs)
// 1. 1987 (16μs)
// 2. 19984714 (79ms)
// Total: 79ms
