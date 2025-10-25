use std::iter::successors;

use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<u8>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(15, 10).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn look_and_say(b: &[u8]) -> Vec<u8> {
    let mut next = Vec::new();
    let mut digit = b[0];
    let mut count = 1;
    let mut rev = Vec::new();
    for i in 1..b.len() {
        if b[i] == digit {
            count += 1;
        } else {
            add_to_next(&mut next, digit, count, &mut rev);
            digit = b[i];
            count = 1;
        }
    }
    add_to_next(&mut next, digit, count, &mut rev);
    next
}

fn add_to_next(next: &mut Vec<u8>, digit: u8, count: i32, rev: &mut Vec<u8>) {
    let mut count = count;

    while count > 0 {
        rev.push((count % 10) as u8);
        count /= 10;
    }

    next.extend(rev.drain(..).rev());
    next.push(digit);
}

fn solve(init: &Input, n: usize) -> usize {
    successors(Some(init.clone()), |acc| Some(look_and_say(acc)))
        .take(n + 1)
        .last()
        .unwrap()
        .len()
}

fn parse_input(input: &str) -> Input {
    input.as_bytes().iter()
        .map(|&b| b - b'0')
        .collect()
}

fn part1(init: &Input) -> Output {
    solve(init, 40)
}

fn part2(init: &Input) -> Output {
    solve(init, 50)
}

#[test]
fn default() {
    let input = get_input(15, 10).unwrap();
    let input = parse_input(&input);
    assert_eq!(492982, part1(&input));
    assert_eq!(6989950, part2(&input));
}

// Input parsed (13μs)
// 1. 492982 (7ms)
// 2. 6989950 (100ms)
// Total: 108ms
