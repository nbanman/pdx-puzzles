use std::{cmp::max, ops::RangeInclusive};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::stopwatch::{ReportDuration, Stopwatch},
};

type Input = (Ranges, Vec<u64>);
type Ranges = Vec<RangeInclusive<u64>>;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(25, 5).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let mut ranges: Ranges = ranges.get_numbers().tuples().map(|(a, b)| a..=b).collect();
    condense_ranges(&mut ranges);
    let ids: Vec<u64> = ids.get_numbers().collect();
    (ranges, ids)
}

fn condense_ranges(ranges: &mut Ranges) {
    ranges.sort_by_cached_key(|rng| *rng.start());
    loop {
        let mut changed = false;
        for i in (1..ranges.len()).rev() {
            let a = &ranges[i - 1];
            let b = &ranges[i];
            if a.end() >= b.start() {
                ranges[i - 1] = *a.start()..=*max(a.end(), b.end());
                ranges.remove(i);
                changed = true;
            }
        }
        if !changed {
            return;
        }
    }
}

fn part1(input: &Input) -> usize {
    let (ranges, ids) = input;
    ids.iter()
        .filter(|&id| ranges.iter().any(|rng| rng.contains(id)))
        .count()
}

fn part2(input: &Input) -> u64 {
    let (ranges, _) = input;
    ranges.iter().map(|rng| rng.end() - rng.start() + 1).sum()
}

#[test]
fn default() {
    let input = get_input(25, 5).unwrap();
    let input = parse_input(&input);
    assert_eq!(652, part1(&input));
    assert_eq!(341753674214273, part2(&input));
}

// Input parsed (71μs)
// 1. 652 (38μs)
// 2. 341753674214273 (2μs)
// Total: 113μs
