use std::iter;
use std::ops::Mul;
use itertools::Itertools;
use advent::utilities::get_input::get_input;
use utilities::parsing::get_numbers::ContainsNumbers;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<Output>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(20, 10).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    // parse adapters from outlet, sort from lowest rating. Then add charging outlet and end devices.
    // Finally convert to a list of the joltage differences between devices.
    let adapters: Vec<Output> = input.get_numbers()
        .sorted_unstable()
        .collect();
    let last_adapter = iter::once(adapters[adapters.len() - 1] + 3);
    iter::once(0)
        .chain(adapters)
        .chain(last_adapter)
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect()
}

fn part1(joltage_differences: &Input) -> Output {
    let jolt3s = joltage_differences.iter()
        .filter(|diff| **diff == 3)
        .count();
    let jolt1s = joltage_differences.len() - jolt3s;
    jolt1s * jolt3s
}

fn part2(joltage_differences: &Input) -> Output {
    // basic idea to reduce the calculations is divide and conquer. Wherever there is a 3-jolt
    // difference that adapter and the adapter before it *must* be in the combination. So split the
    // list using the 3-jolt differences. You then have a bunch of sublists with 1-jolt differences.
    // The maximum number of 1s you see is 4, so you can use a lookup table to count the number of
    // possible permutations in each sublist. Multiply them all together and you get your answer.
    joltage_differences.iter()
        .map(|&diff| (diff as u8 + 48) as char)
        .collect::<String>() // join differences to one string before splitting in a different way
        .split('3') // both devices 3 apart must be in chain so don't permute
        .map(|ones| {
            // each string of 1s represents devices one away from each other.
            // if there's nothing between two threes, that's blank, but return 1 for the fold
            match ones.len() {
                4 => 7, // 1111, 1101, 1011, 1001, 0111, 0101, 0011 (0001 not allowed b/c that's 4 apart)
                3 => 4, // 111, 101, 011, 001
                2 => 2, // 11, 01
                0 | 1 => 1, // 1
                c => panic!("{c}: should never be more than 4 devices one away from each other")
            }
        })
        .reduce(Output::mul)
        .unwrap()
}

#[test]
fn default() {
    let input = get_input(20, 10).unwrap();
    let input = parse_input(&input);
    assert_eq!(1890, part1(&input));
    assert_eq!(49607173328384, part2(&input));
}

// Input parsed (22μs)
// 1. 1890 (6μs)
// 2. 49607173328384 (3μs)
// Total: 35μs