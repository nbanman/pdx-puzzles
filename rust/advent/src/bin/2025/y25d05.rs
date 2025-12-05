use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::stopwatch::{ReportDuration, Stopwatch},
};

type Input = (Ranges, Vec<u64>);
type Ranges = Vec<(u64, u64)>;

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
    let mut ranges: Ranges = ranges.get_numbers().tuples().sorted_unstable().collect();
    condense_ranges(&mut ranges);
    let ids: Vec<u64> = ids.get_numbers().sorted_unstable().collect();
    (ranges, ids)
}

fn condense_ranges(ranges: &mut Ranges) {
    loop {
        let mut changed = false;
        for i in (1..ranges.len()).rev() {
            let (a_from, a_to) = &ranges[i - 1];
            let (b_from, b_to) = &ranges[i];
            if a_to >= b_from {
                ranges[i - 1] = (*a_from, *a_to.max(b_to));
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
    ranges.iter()
        .map(|(start, end)| {
            let below = ids.binary_search(start).unwrap_or_else(|e| e);
            let within = ids.binary_search(end).unwrap_or_else(|e| e);
            within - below
        })
        .sum()
}

fn part2(input: &Input) -> u64 {
    let (ranges, _) = input;
    ranges.iter().map(|(from, to)| to - from + 1).sum()
}

#[test]
fn default() {
    let input = get_input(25, 5).unwrap();
    let input = parse_input(&input);
    assert_eq!(652, part1(&input));
    assert_eq!(341753674214273, part2(&input));
}

// Input parsed (71μs)
// 1. 652 (9μs)
// 2. 341753674214273 (2μs)
// Total: 84μs
