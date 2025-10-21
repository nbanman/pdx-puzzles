use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input = Vec<Disc>;
type Output = i64;
type Disc = (i64, i64);

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(16, 15).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.get_numbers::<i64>()
        .tuples()
        .map(|(start_time, positions, _, position_at_t0)| {
            (positions, -start_time - position_at_t0)
        })
        .collect()
}

// Uses a sieve version of CRT
fn solve(discs: impl Iterator<Item = Disc>) -> Output {
    discs
        .reduce(|(interval, seconds), (positions, offset)| {
            // take the current number of seconds that works for the previous discs, and keep adding the current
            // interval until it works for the next disc.
            let next_seconds = (seconds..).step_by(interval as usize)
                .find(|&second| (second - offset).rem_euclid(positions) == 0)
                .unwrap();

            // values are coprime; next interval is current interval multiplied by the number of positions in next disc
            let next_interval = interval * positions;
            (next_interval, next_seconds)
        })
        .map(|(_, seconds)| seconds)
        .unwrap()
}

fn part1(discs: Input) -> Output {
    solve(discs.into_iter())
}

fn part2(discs: Input) -> Output {
    let discs = discs.into_iter().chain(std::iter::once((11, -7)));
    solve(discs)
}

#[test]
fn default() {
    let input = get_input(16, 15).unwrap();
    let input = parse_input(&input);
    assert_eq!(122318, part1(input.clone()));
    assert_eq!(3208583, part2(input));
}

// Input parsed (18μs)
// 1. 122318 (8μs)
// 2. 3208583 (2μs)
// Total: 30μs
