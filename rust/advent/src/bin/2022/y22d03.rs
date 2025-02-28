use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Int = u64;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(22, 3).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn priority(b: u8) -> u8 {
    if b.is_ascii_lowercase() {
        b - b'a' + 1
    } else {
        b - b'A' + 27
    }
}

fn bitset(sack: &[u8]) -> Int {
    sack.iter()
        .fold(0, |acc, &b| acc | (1 << (priority(b) as Int)))
}

fn part1(rucksacks: Input) -> Int {
    rucksacks
        .trim_end()
        .as_bytes()
        .split(|&b| b == b'\n')
        .map(|sack| {
            let (a, b) = sack.split_at(sack.len() / 2);
            (bitset(a) & bitset(b)).trailing_zeros() as u64
        })
        .sum()
}

fn part2(rucksacks: Input) -> Int {
    rucksacks
        .trim_end()
        .as_bytes()
        .split(|&b| b == b'\n')
        .map(|sack| bitset(sack))
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            chunk
                .reduce(std::ops::BitAnd::bitand)
                .unwrap()
                .trailing_zeros() as Int
        })
        .sum()
}

#[test]
fn default() {
    let input = get_input(22, 3).unwrap();
    assert_eq!(7428, part1(&input));
    assert_eq!(2650, part2(&input));
}

// Input parsed (15μs)
// 1. 7428 (19μs)
// 2. 2650 (16μs)
// Total: 54μs
