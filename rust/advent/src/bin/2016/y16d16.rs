use std::iter::successors;

use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<bool>;
type Output = String;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(16, 16).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.as_bytes().iter().map(|&b| b == b'1').collect()
}

fn checksum(sum: Input) -> String {
    successors(Some(sum), |acc| {
        Some(
            (0..acc.len() / 2)
                .map(|i| !(acc[i * 2] ^ acc[i * 2 + 1]))
                .collect(),
        )
    })
    .skip(1)
    .find(|sum| sum.len() & 1 == 1)
    .map(|sum| sum.into_iter().map(|b| if b { '1' } else { '0' }).collect())
    .unwrap()
}

fn dragon_curve(a: Input, disk_size: usize) -> Input {
    successors(Some(a), |acc| {
        let next = (0..acc.len() * 2 + 1)
            .map(|i| {
                let acc_len = acc.len();
                if i < acc_len {
                    acc[i]
                } else if i > acc_len {
                    !acc[acc_len * 2 - i]
                } else {
                    false
                }
            })
            .collect();
        Some(next)
    })
    .find(|it| it.len() >= disk_size)
    .map(|it| {
        it[..disk_size].to_vec()
    })
    .unwrap()
}

fn solve(a: Input, disk_size: usize) -> Output {
    checksum(dragon_curve(a, disk_size))
}

fn part1(input: Input) -> Output {
    solve(input, 272)
}

fn part2(input: Input) -> Output {
    solve(input, 35651584)
}

#[test]
fn default() {
    let input = get_input(16, 16).unwrap();
    let input = parse_input(&input);
    assert_eq!("10010101010011101".to_string(), part1(input.clone()));
    assert_eq!("01100111101101111".to_string(), part2(input));
}

// Input parsed (18μs)
// 1. 10010101010011101 (11μs)
// 2. 01100111101101111 (120ms)
// Total: 120ms