use std::{cmp::max, ops::Range};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input = Vec<Range<usize>>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(16, 20).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.get_numbers().tuples()
        .map(|(start, end)| start..end + 1)
        .sorted_unstable_by_key(|rng| rng.start)
        .collect()
}

fn ip_sequence(ranges: &Input) -> impl Iterator<Item = usize> {
    IpSequence { ranges: ranges.iter(), ip: 0, done: false }.flatten()
}

struct IpSequence<'a> {
    ranges: std::slice::Iter<'a, Range<usize>>,
    ip: usize,
    done: bool,
}

impl<'a> Iterator for IpSequence<'a> {
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        loop {
            let Some(rng) = self.ranges.next() else {
                self.done = true;
                return Some(self.ip..4_294_967_296);
            };
            let inverse = self.ip..rng.start;
            self.ip = max(self.ip, rng.end);
            if inverse.start < inverse.end {
                return Some(inverse);
            }
        }
    }
}

fn part1(ranges: &Input) -> Output {
    ip_sequence(ranges).next().unwrap()
}

fn part2(ranges: &Input) -> Output {
    ip_sequence(ranges).count()
}

#[test]
fn default() {
    let input = get_input(16, 20).unwrap();
    let input = parse_input(&input);
    assert_eq!(19449262, part1(&input));
    assert_eq!(119, part2(&input));
}

