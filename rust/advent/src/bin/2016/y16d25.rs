use std::iter::successors;

use advent::utilities::{assembunny::Assembunny, get_input::get_input};
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(16, 25).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: Input) -> Output {
    let mut asmb: Assembunny = input.into();
    asmb.run(Some(10_000));
    let offset = asmb['d'] as usize;
    (1..)
        .find(|&i| {
            successors(Some(i + offset), |&it| {
                if it == 0 {
                    None
                } else {
                    Some(it >> 1)
                }
            })
                .enumerate()
                .all(|(idx, value)| idx & 1 == value &1)
        })
        .unwrap()
}

#[test]
fn default() {
    let input = get_input(16, 25).unwrap();
    assert_eq!(175, part1(&input));
}

// Input parsed (179μs)
// 1. 175 (33μs)
// Total: 214μs
