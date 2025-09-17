use std::ops::{BitXor, RangeInclusive};
use itertools::Itertools;
use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<PassPolicy>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(20, 2).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

struct PassPolicy {
    letter: char,
    range: RangeInclusive<usize>,
    password: String,
}

impl PassPolicy {
    fn valid_old(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.letter).count();
        self.range.contains(&count)
    }
    fn valid_new(&self) -> bool {
        let a = self.password.as_bytes()[self.range.start() - 1] == self.letter as u8;
        let b = self.password.as_bytes()[self.range.end() - 1] == self.letter as u8;
        a.bitxor(b)
    }
}

fn parse_input(input: &str) -> Input {
    input.lines()
        .map(|line| {
            let (lower, upper, letter, password) = line
                .split(['-', ' '])
                .collect_tuple()
                .unwrap();
            PassPolicy {
                letter: letter.chars().next().unwrap(),
                range: lower.parse().unwrap()..=upper.parse().unwrap(),
                password: password.to_string(),
            }
        })
        .collect()
}

fn part1(policies: &Input) -> Output {
    policies.iter().filter(|p| p.valid_old()).count()
}

fn part2(policies: &Input) -> Output {
    policies.iter().filter(|p| p.valid_new()).count()
}

#[test]
fn default() {
    let input = get_input(20, 2).unwrap();
    let input = parse_input(&input);
    assert_eq!(445, part1(&input));
    assert_eq!(491, part2(&input));
}

// Input parsed (126μs)
// 1. 445 (18μs)
// 2. 491 (3μs)
// Total: 149μs