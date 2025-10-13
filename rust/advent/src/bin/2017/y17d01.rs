use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<usize>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(17, 1).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.trim_end().as_bytes().iter().map(|&b| (b - b'0') as usize).collect()
}

fn solve(numbers: &Input, comparison_index: fn(usize) -> usize) -> Output {
    let num_len = numbers.len();
    numbers.iter().enumerate()
        .filter(|&(index, &i)| {
            let other_index = (index + comparison_index(num_len)) % num_len;
            numbers[other_index] == i
        })
        .map(|(_, &i)| i)
        .sum()
}

fn part1(input: &Input) -> Output {
    solve(input, |_| 1)
}

fn part2(input: &Input) -> Output {
    solve(input, |len| len / 2)
}

#[test]
fn default() {
    let input = get_input(17, 1).unwrap();
    let input = parse_input(&input);
    assert_eq!(1182, part1(&input));
    assert_eq!(1152, part2(&input));
}

// Input parsed (21μs)
// 1. 1182 (7μs)
// 2. 1152 (5μs)
// Total: 35μs
