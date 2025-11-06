use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = &'a str;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(25, 3);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[inline(always)]
fn parse(input: Input) -> impl Iterator<Item = usize> {
    input.as_bytes().split(|b| b == &b',').map(|bytes| {
        bytes.iter().fold(0, |acc, &b| acc * 10 + (b - b'0') as usize)
    })
}

fn part1(input: Input) -> usize {
    parse(input).sorted_unstable().dedup().sum()
}

fn part2(input: Input) -> usize {
    parse(input).sorted_unstable().dedup().take(20).sum()
}

fn part3(input: Input) -> usize {
    let mut counts: [usize; 100] = [0; 100];
    for n in parse(input) {
        counts[n] += 1;
    }
    counts.into_iter().max().unwrap()
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(25, 3);
    assert_eq!(2569, part1(&input1));
    assert_eq!(296, part2(&input2));
    assert_eq!(3204, part3(&input3));
}

// Input parsed (37μs)
// 1. 2569 (10μs)
// 2. 296 (7μs)
// 3. 3204 (24μs)
// Total: 81μs