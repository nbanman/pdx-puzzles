use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(25, 3).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: Input) -> Output {
    solve(input, 2)
}

fn part2(input: Input) -> Output {
    solve(input, 12)
}

fn solve(input: Input, digits: usize) -> Output {
    input.lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let len = bytes.len();
            let mut left = 0;
            (0..digits).rev().fold(0usize, |acc, i| {
                let bytes = &bytes[left..len - i];
                let mut highest = 0;
                let mut high_pos = 0;
                for (idx, &byte) in bytes.iter().enumerate() {
                    if byte > highest {
                        highest = byte;
                        high_pos = idx;
                    }
                }
                left += high_pos + 1;
                let highest = (highest - b'0') as usize;
                acc * 10 + highest
            })
        })
        .sum()
}

#[test]
fn default() {
    let input = get_input(25, 3).unwrap();
    assert_eq!(17343, part1(&input));
    assert_eq!(172664333119298, part2(&input));
}

#[test]
fn test1() {
    let input = r"987654321111111
811111111111119
234234234234278
818181911112111";
    assert_eq!(357, part1(&input));
}

#[test]
fn test2() {
    let input = r"987654321111111
811111111111119
234234234234278
818181911112111";
    assert_eq!(3121910778619, part2(&input));
}

// Input parsed (23μs)
// 1. 17343 (25μs)
// 2. 172664333119298 (44μs)
// Total: 94μs
