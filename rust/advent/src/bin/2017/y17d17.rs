use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = usize;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(17, 17).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.parse().unwrap()
}

fn part1(input: Input) -> Output {
    let mut buffer = vec![0];
    let mut curr_pos = 0;
    for n in 1..=2017 {
        curr_pos = (curr_pos + input) % buffer.len() + 1;
        buffer.insert(curr_pos, n);
    }
    buffer[curr_pos + 1]
}

fn part2(input: Input) -> Output {
    let mut curr_pos = 0;
    let mut result = 0;
    let mut n = 0;
    let limit = 50_000_000;
    while n < limit {
        if curr_pos == 1 {
            result = n;
        }
        let fits = (n - curr_pos) / input;
        n += fits + 1;
        curr_pos = (curr_pos + (fits + 1) * (input + 1) - 1) % n + 1;
    }
    result
}

#[test]
fn default() {
    let input = get_input(17, 17).unwrap();
    let input = parse_input(&input);
    assert_eq!(1547, part1(input));
    assert_eq!(31154878, part2(input));
}

// Input parsed (10μs)
// 1. 1547 (77μs)
// 2. 31154878 (30μs)
// Total: 120μs
