use std::cmp::max;

use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(22, 6).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn solve(input: Input, n: usize) -> Output {
    let mut index_map = [0; 26];
    let mut duplicate_index = 0;
    let mut index = 0;

    input
        .as_bytes()
        .iter()
        .enumerate()
        .find(|&(_, &b)| {
            let i = (b - b'a') as usize;

            let last_seen = index_map[i];
            index_map[i] = index;
            duplicate_index = max(duplicate_index, last_seen);

            let predicate = index - duplicate_index >= n;
            index += 1;

            predicate
        })
        .unwrap()
        .0
        + 1
}

fn part1(input: Input) -> Output {
    solve(input, 4)
}

fn part2(input: Input) -> Output {
    solve(input, 14)
}

#[test]
fn default() {
    let input = get_input(22, 6).unwrap();
    assert_eq!(1361, part1(&input));
    assert_eq!(3263, part2(&input));
}

// Input parsed (14μs)
// 1. 1361 (6μs)
// 2. 3263 (5μs)
// Total: 27μs
