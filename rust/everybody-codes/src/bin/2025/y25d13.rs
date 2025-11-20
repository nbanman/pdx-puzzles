use std::collections::VecDeque;
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

fn solve(ranges: impl Iterator<Item = (u32, u32)>, total_turns: u64) -> u32 {
    // ranges stored in VecDeque to approximate 'circular' layout of the ranges
    let mut lock: VecDeque<(u32, u32)> = VecDeque::with_capacity(500);
    lock.push_back((1, 1));

    // we don't know how many ranges there are in the lock to start, so we track it as we
    // iterate. When the left half gets pushed on to the Deque, the index will shift so
    // this will ensure that we start at the 12 o'clock position once the lock is built.
    let mut start = 0;

    // The total number of spots on the lock, not the range, but as if the range were exploded.
    // Starts at 1 to account for the 12 o'clock spot that is not in the input.
    let mut total = 1;

    // This bool cycles on and off, telling us to place ranges forward or backward on the lock.
    let mut forward = true;

    for (lo, hi) in ranges {
        total += hi - lo + 1;
        if forward {
            lock.push_back((lo, hi));
        } else {
            start += 1;
            lock.push_front((lo, hi))
        }
        forward = !forward;
    }

    // use mod math to eliminate a bunch of full circles. Also bump turns by one, which allows
    // the below loop to load the appropriate hi/lo pair.
    let total_turns = ((total_turns + 1) % total as u64) as u32;
    let mut turns = 0;
    
    for i in start.. {
        let i = i % lock.len();
        let &(lo, hi) = lock.get(i).unwrap();
        turns += hi - lo + 1;
        if turns >= total_turns {
            let diff = turns - total_turns;
            return if i >= start {
                hi - diff
            } else {
                lo + diff
            }
        }
    }
    unreachable!()
}

fn part1(input: Input) -> u32 {
    let ranges = input
        .get_numbers()
        .map(|n| (n, n));
    solve(ranges, 2025)
}
fn part2(input: Input) -> u32 {
    let ranges = input
        .get_numbers()
        .tuples()
        .map(|(a, b)| (a, b));
    solve(ranges, 20_252_025)
}
fn part3(input: Input) -> u32 {
    let ranges = input
        .get_numbers()
        .tuples()
        .map(|(a, b)| (a, b));
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
// 1. 353 (10μs)
// 2. 7613 (4μs)
// 3. 217823 (14μs)
// Total: 59μs
