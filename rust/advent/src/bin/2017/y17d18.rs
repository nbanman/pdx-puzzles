//! See my Kotlin solution for a straightforward implementation. This version just recapitulates /u/maneatingape's
//! reverse-engineered solution. See his code for explanation.

use advent::utilities::get_input::get_input;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::stopwatch::{ReportDuration, Stopwatch},
};

type Input = Vec<usize>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(17, 18).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

// Generate pseudorandom sequence (first section of program)
fn parse_input(input: &str) -> Input {
    let seed: usize = input.lines().nth(9).unwrap().get_numbers().next().unwrap();
    (0..127)
        .scan(seed, |state, _| {
            *state = (*state * 8505) % 0x7fffffff;
            *state = (*state * 129749 + 12345) % 0x7fffffff;
            Some(*state % 10_000)
        })
        .collect()
}

// Last value in the sequence
fn part1(sequence: &Input) -> Output {
    sequence[126]
}

// Perform bubble sort of sequence, counting "passes" until sort is completed.
fn part2(sequence: &Input) -> Output {
    let mut sequence = sequence.clone();
    let mut swapped = true;
    let mut count = 0;

    while swapped {
        swapped = false;
        for i in 1..127 - count {
            if sequence[i - 1] < sequence[i] {
                sequence.swap(i - 1, i);
                swapped = true;
            }
        }
        count += 1;
    }
    127 * count.div_ceil(2)
}

#[test]
fn default() {
    let input = get_input(17, 18).unwrap();
    let input = parse_input(&input);
    assert_eq!(9423, part1(&input));
    assert_eq!(7620, part2(&input));
}

// Input parsed (20μs)
// 1. 9423 (6μs)
// 2. 7620 (13μs)
// Total: 42μs
