use std::collections::HashMap;

use advent::utilities::get_input::get_input;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::stopwatch::{ReportDuration, Stopwatch},
};

type Input = Vec<usize>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(20, 15).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.get_numbers().collect()
}

fn last_number_spoken(start: &Input, iterations: usize) -> Output {
    let bounds = iterations / 10;
    let mut low = vec![0usize; bounds];
    let mut high = HashMap::new();
    for (turn, &n) in start.iter().enumerate() {
        low[n] = turn + 1;
    }
    let mut current = 0;
    for turn in start.len() + 1..iterations {
        if current < bounds {
            let prev = low[current];
            low[current] = turn;
            current = if prev == 0 { 0 } else { turn - prev };
        } else {
            current = high
                .insert(current, turn)
                .map(|prev| turn - prev)
                .unwrap_or_default();
        }
    }
    current
}
fn part1(start: &Input) -> Output {
    last_number_spoken(start, 2020)
}

fn part2(start: &Input) -> Output {
    last_number_spoken(start, 30_000_000)
}

#[test]
fn default() {
    let input = get_input(20, 15).unwrap();
    let input = parse_input(&input);
    assert_eq!(929, part1(&input));
    assert_eq!(16671510, part2(&input));
}

// Input parsed (13μs)
// 1. 929 (31μs)
// 2. 16671510 (283ms)
// Total: 283ms
