use advent::utilities::get_input::get_input;
use advent::utilities::intcode::{IntCode, State};
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Output = i64;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(19, 9).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn solve(input: Input, initial: i64) -> Output {
    let mut comp = IntCode::from(input);
    comp.input(initial);
    let State::Output(value) = comp.run() else { unreachable!() };
    value
}

fn part1(input: Input) -> Output {
    solve(input, 1)
}

fn part2(input: Input) -> Output {
    solve(input, 2)
}

#[test]
fn default() {
    let input = get_input(19, 9).unwrap();
    assert_eq!(2870072642, part1(&input));
    assert_eq!(58534, part2(&input));
}

// Input parsed (17μs)
// 1. 2870072642 (41μs)
// 2. 58534 (2ms)
// Total: 2ms