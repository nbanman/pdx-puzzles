use std::{collections::VecDeque, ops::RangeInclusive};

use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = &'a str;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(25, 13);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn solve(ranges: impl Iterator<Item = RangeInclusive<u64>>, total_turns: u64) -> u64 {
    let mut lock: VecDeque<(RangeInclusive<u64>, bool)> = VecDeque::with_capacity(500);
    lock.push_back((1..=1, true));
    let mut start = 0;
    let mut total: u64 = 1;

    for (idx, rng) in ranges.enumerate() {
        total += rng.end() - rng.start() + 1;
        if idx & 1 == 0 {
            lock.push_back((rng, true));
        } else {
            start += 1;
            lock.push_front((rng, false))
        }      
    }

    let total_turns = (total_turns + 1) % total;
    let mut turns: u64 = 0;
    
    for i in start.. {
        let (rng, is_forward) = lock.get(i % lock.len()).unwrap();
        turns += rng.end() - rng.start() + 1;
        if turns >= total_turns {
            let diff = turns - total_turns;
            return if *is_forward {
                rng.end() - diff
            } else {
                rng.start() + diff
            }
        }
    }
    unreachable!()
}

fn part1(input: Input) -> u64 {
    let ranges = input.get_numbers().map(|n| n..=n);
    solve(ranges, 2025)
}

fn part2(input: Input) -> u64 {
    let ranges = input.get_numbers().tuples().map(|(a, b)| a..=b);
    solve(ranges, 20_252_025)
}

fn part3(input: Input) -> u64 {
    let ranges = input.get_numbers().tuples().map(|(a, b)| a..=b);
    solve(ranges, 202_520_252_025)
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(25, 13);
    assert_eq!(353, part1(&input1));
    assert_eq!(7613, part2(&input2));
    assert_eq!(217823, part3(&input3));
}

// Input parsed (28μs)
// 1. 353 (8μs)
// 2. 7613 (6μs)
// 3. 217823 (20μs)
// Total: 65μs
