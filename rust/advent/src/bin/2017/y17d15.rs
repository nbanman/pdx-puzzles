use std::iter::successors;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input = (usize, usize);
type Output = usize;

const FACTOR_A: usize = 16_807;
const FACTOR_B: usize = 48_271;
const MOD: usize = 2_147_483_647;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(17, 15).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.get_numbers().collect_tuple().unwrap()
}

fn generator(seed: usize, factor: usize, multiples: Option<usize>) -> impl Iterator<Item = u16> {
    successors(Some(seed), move |&it| Some((it * factor) % MOD))
        .skip(1)
        .filter(move |&it| {
            if let Some(multiples) = multiples {
                it % multiples == 0
            } else {
                true
            }
        })
        .map(|it| it as u16)
}

fn solve(input: Input, comparisons: usize, multiples_a: Option<usize>, multiples_b: Option<usize>) -> Output {
    let (seed_a, seed_b) = input;
    generator(seed_a, FACTOR_A, multiples_a)
        .zip(generator(seed_b, FACTOR_B, multiples_b))
        .take(comparisons)
        .filter(|(a, b)| a == b)
        .count()
}

fn part1(input: Input) -> Output {
    solve(input, 40_000_000, None, None)
}

fn part2(input: Input) -> Output {
    solve(input, 5_000_000, Some(4), Some(8))
}

#[test]
fn default() {
    let input = get_input(17, 15).unwrap();
    let input = parse_input(&input);
    assert_eq!(594, part1(input));
    assert_eq!(328, part2(input));
}

// Input parsed (168μs)
// 1. 594 (132ms)
// 2. 328 (186ms)
// Total: 319ms
