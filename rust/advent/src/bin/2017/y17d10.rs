use advent::utilities::{get_input::get_input, hashes::{dense_hash, knot_hash}};
use itertools::Itertools;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::stopwatch::{ReportDuration, Stopwatch},
};

type Input<'a> = &'a str;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(17, 10).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: Input) -> usize {
    let lengths: Vec<usize> = input.get_numbers().collect();
    let ring = (0..256).collect_vec();
    let mut knot_hash = knot_hash(ring, &lengths, 0);
    let knot_len = knot_hash.len();
    let shift = lengths.iter().sum::<usize>()
        + (1..lengths.len()).sum::<usize>();
    knot_hash.rotate_right(shift % knot_len);
    knot_hash[0] * knot_hash[1]
}

fn part2(input: Input) -> String {
    let lengths = input.as_bytes().iter()
        .map(|&b| b as usize)
        .chain([17, 31, 73, 47, 23].into_iter())
        .collect_vec();
    dense_hash(&lengths)
}

#[test]
fn default() {
    let input = get_input(17, 10).unwrap();
    assert_eq!(23874, part1(&input));
    assert_eq!(
        "e1a65bfb5a5ce396025fab5528c25a87".to_string(),
        part2(&input)
    );
}

// Input parsed (11μs)
// 1. 23874 (16μs)
// 2. e1a65bfb5a5ce396025fab5528c25a87 (499μs)
// Total: 528μs
