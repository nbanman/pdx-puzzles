use std::ops::Range;
use itertools::Itertools;
use advent::utilities::get_input::get_input;
use advent::utilities::intcode::{IntCode, State};
use utilities::parsing::get_numbers::ContainsNumbers;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<i64>;
type Output = i64;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(19, 7).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.get_numbers().collect()
}

fn solve(code: &Input, range: Range<i64>, loop_output: bool) -> Output {
    range.permutations(5)
        .map(|combo| {
            let mut computers: Vec<IntCode> = combo.into_iter()
                .map(|n| {
                    let mut comp = IntCode::new(code);
                    comp.input(n);
                    comp
                })
                .collect();
            computers[0].input(0);
            let mut last = 0;
            'outer: loop {
                for id in 0..5 {
                    match computers[id].run() {
                        State::Output(value) => {
                            if id == 4 {
                                last = value;
                                if loop_output {
                                    computers[0].input(value);
                                }
                            } else {
                                computers[id + 1].input(value);
                            }
                        },
                        State::Halted => {
                            break 'outer last;
                        }
                        State::Input => {
                            unreachable!()
                        },
                    }
                }
            }
        })
        .max()
        .unwrap()
}

fn part1(code: &Input) -> Output {
    solve(code, 0..5, false)
}

fn part2(code: &Input) -> Output {
    solve(code, 5..10, true)
}

#[test]
fn default() {
    let input = get_input(19, 7).unwrap();
    let input = parse_input(&input);
    assert_eq!(24405, part1(&input));
    assert_eq!(8271623, part2(&input));
}

// Input parsed (23μs)
// 1. 24405 (111μs)
// 2. 8271623 (205μs)
// Total: 342μs