use advent::utilities::get_input::get_input;
use lazy_regex::regex;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(15, 8).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn chars_in_memory(s: &str) -> usize {
    let rx = regex!(r#"\\\\|\\"|\\x[\da-f]{2}"#);
    rx.replace_all(&s[1..s.len() - 1], "X").len()
}

fn encoded_length(s: &str) -> usize {
    s.len() + 2 + s.as_bytes().iter()
        .filter(|&&b| b == b'"' || b == b'\\')
        .count()
}

fn part1(input: Input) -> Output {
    let total_length = input.len() - input.as_bytes().iter()
        .filter(|&&b| b == b'\n')
        .count();
    total_length - input.lines()
        .map(|line| chars_in_memory(line))
        .sum::<usize>()
}

fn part2(input: Input) -> Output {
    let total_length = input.len() - input.as_bytes().iter()
        .filter(|&&b| b == b'\n')
        .count();
    input.lines()
        .map(|line| encoded_length(line))
        .sum::<usize>() - total_length
}

#[test]
fn default() {
    let input = get_input(15, 8).unwrap();
    assert_eq!(1333, part1(&input));
    assert_eq!(2046, part2(&input));
}

// Input parsed (42μs)
// 1. 1333 (385μs)
// 2. 2046 (16μs)
// Total: 445μs
