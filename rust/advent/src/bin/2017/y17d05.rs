use advent::utilities::get_input::get_input;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input = Vec<i64>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(17, 5).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.get_numbers().collect()
}

fn solve(jumps: &Input, increment: fn(i64) -> i64) -> Output {
    let mut offsets: Vec<i64> = jumps.iter().copied().collect();
    let mut i = 0;
    let mut steps = 0;
    while i < offsets.len() {
        steps += 1;
        let old_i = i;
        i = (i as i64 + offsets[i]) as usize;
        offsets[old_i] += increment(offsets[old_i])
    }
    steps
}

fn part1(jumps: &Input) -> Output {
    solve(jumps, |_| 1)
}

fn part2(jumps: &Input) -> Output {
    solve(jumps, |it| if it >= 3 { -1 } else { 1 })
}

#[test]
fn default() {
    let input = get_input(17, 5).unwrap();
    let input = parse_input(&input);
    assert_eq!(373160, part1(&input));
    assert_eq!(26395586, part2(&input));
}

// Input parsed (261μs)
// 1. 373160 (512μs)
// 2. 26395586 (35ms)
// Total: 36ms
