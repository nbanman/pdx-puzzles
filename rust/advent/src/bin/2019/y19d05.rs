use advent::utilities::{get_input::get_input, intcode::{IntCode, State}};
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Output = i64;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(19, 5).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: Input) -> Output {
    let mut comp = IntCode::from(input);
    comp.input(1);
    let mut last = 0;
    while let State::Output(next) = comp.run() {
        last = next;
    }
    last
}

fn part2(input: Input) -> Output {
    let mut comp = IntCode::from(input);
    comp.input(5);
    let mut last = 0;
    while let State::Output(next) = comp.run() {
        last = next;
    }
    last
}

#[test]
fn default() {
    let input = get_input(19, 5).unwrap();
    assert_eq!(7839346, part1(&input));
    assert_eq!(447803, part2(&input));
}

// Input parsed (19μs)
// 1. 7839346 (30μs)
// 2. 447803 (19μs)
// Total: 72μs