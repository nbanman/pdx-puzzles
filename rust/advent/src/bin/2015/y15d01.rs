use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<i32>;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(15, 1).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.as_bytes().iter().map(|&b| if b == b'(' { 1 } else { -1 }).collect()
}

fn part1(floor_changes: &Input) -> i32 {
    floor_changes.iter().sum()
}

fn part2(floor_changes: &Input) -> usize {
    floor_changes.iter()
        .scan(0, |state, &change| {
            *state += change;
            Some(*state)
        })
        .position(|floor| floor == -1)
        .unwrap() + 1
}

#[test]
fn default() {
    let input = get_input(15, 1).unwrap();
    let input = parse_input(&input);
    assert_eq!(280, part1(&input));
    assert_eq!(1797, part2(&input));
}

// Input parsed (32μs)
// 1. 280 (4μs)
// 2. 1797 (2μs)
// Total: 43μs