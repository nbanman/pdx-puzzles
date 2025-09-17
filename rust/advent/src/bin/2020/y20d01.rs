use std::collections::HashSet;
use itertools::Itertools;
use advent::utilities::get_input::get_input;
use utilities::parsing::get_numbers::ContainsNumbers;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = (Vec<Output>, HashSet<Output>);
type Output = i64;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(20, 1).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let entries: Vec<Output> = input.get_numbers().collect();
    let entry_set: HashSet<Output> = entries.clone().into_iter().collect();
    (entries, entry_set)
}

fn part1((_, entry_set): &Input) -> Output {
    let entry = *entry_set.iter()
        .find(|&entry| entry_set.contains(&(2020 - *entry)))
        .unwrap();
    entry * (2020 - entry)
}

fn part2((entries, entry_set): &Input) -> Output {
    let (&entry_a, &entry_b) = entries.into_iter()
        .tuple_combinations()
        .find(|&(a, b)| entry_set.contains(&(2020 - *a - *b)))
        .unwrap();
    entry_a * entry_b * (2020 - entry_a - entry_b)
}

#[test]
fn default() {
    let input = get_input(20, 1).unwrap();
    let input = parse_input(&input);
    assert_eq!(1015476, part1(&input));
    assert_eq!(200878544, part2(&input));
}

// Input parsed (35μs)
// 1. 1015476 (5μs)
// 2. 200878544 (42μs)
// Total: 86μs