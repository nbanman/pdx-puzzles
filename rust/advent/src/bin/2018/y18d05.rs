use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<char>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(18, 5).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    react(input.trim_end().chars(), None)
}

fn react<I>(chars: I, removed: Option<char>) -> Vec<char>
where
    I: IntoIterator<Item = char>,
{
    let mut reacted = Vec::new();
    for unit in chars {
        if let Some(removed) = removed {
            if unit == removed || unit == removed.to_ascii_uppercase() { continue; }
        }
        if reacted.is_empty() || (*reacted.last().unwrap() as i8 - unit as i8).abs() != 32 {
            reacted.push(unit);
        } else {
            reacted.pop();
        }
    }
    reacted
}

fn part1(polymer: &Input) -> Output {
    polymer.len()
}

fn part2(polymer: &Input) -> Output {
    ('a'..'z')
        .map(|removed| react(polymer.iter().copied(), Some(removed)).len())
        .min()
        .unwrap()
}

#[test]
fn default() {
    let input = get_input(18, 5).unwrap();
    let input = parse_input(&input);
    assert_eq!(10972, part1(&input));
    assert_eq!(5278, part2(&input));
}

// Input parsed (223μs)
// 1. 10972 (4μs)
// 2. 5278 (327μs)
// Total: 557μs
