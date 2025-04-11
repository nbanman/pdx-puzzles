use std::iter::successors;

use advent::utilities::get_input::get_input;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input = [usize; 9];
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(21, 6).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let mut fish = [0; 9];
    for day in input.get_numbers::<usize>() {
        fish[day] += 1; 
    }
    fish
}

fn solve(fish: &[usize; 9], days: usize) -> Output {
    successors(Some(*fish), |fish| {
        Some(propagate(fish))
    })
    .take(days + 1)
    .last()
    .unwrap()
    .iter()
    .sum()
}

fn propagate(fish: &[usize; 9]) -> [usize; 9] {
    let mut next = [0; 9];
    for day in 0..9 {
        next[day] = match day {
            6 => fish[0] + fish[7],
            8 => fish[0],
            d => fish[d + 1],
        };
    }
    next
}

fn part1(input: &Input) -> Output {
    solve(input, 80)
}

fn part2(input: &Input) -> Output {
    solve(input, 256)
}

#[test]
fn default() {
    let input = get_input(21, 6).unwrap();
    let input = parse_input(&input);
    assert_eq!(361169, part1(&input));
    assert_eq!(1634946868992, part2(&input));
}

// Input parsed (15μs)
// 1. 361169 (5μs)
// 2. 1634946868992 (1μs)
// Total: 24μs