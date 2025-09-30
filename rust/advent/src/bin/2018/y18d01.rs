use advent::utilities::get_input::get_input;
use rustc_hash::FxHashSet;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input = Vec<i64>;
type Output = i64;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(18, 1).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.get_numbers().collect()
}

fn part1(changes: &Input) -> Output {
    changes.iter().sum()
}

fn part2(changes: &Input) -> Output {
    let mut record = FxHashSet::default();
    let mut answer = 0;
    for &change in changes.iter().cycle() {
        answer += change;
        if !record.insert(answer) { break; }
    }
    answer
}

#[test]
fn default() {
    let input = get_input(18, 1).unwrap();
    let input = parse_input(&input);
    assert_eq!(433, part1(&input));
    assert_eq!(256, part2(&input));
}

// Input parsed (33μs)
// 1. 433 (5μs)
// 2. 256 (3ms)
// Total: 3ms
