use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(25, 3).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: Input) -> Output {
    input.lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let highest = bytes[0..bytes.len() - 1].iter().max().unwrap();
            let high_pos = bytes.iter().position(|b| b == highest).unwrap();
            let next = bytes[high_pos + 1..].iter().max().unwrap();
            (*highest - b'0') as usize * 10 + (*next - b'0') as usize
        })
        .sum()
}

fn part2(input: Input) -> Output {
    input.lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let len = bytes.len();
            let mut left = 0;
            (0..12).rev().fold(0usize, |acc, i| {
                let bytes = &bytes[left..len - i];
                let highest = bytes.iter().max().unwrap();
                let high_pos = bytes.iter().position(|b| b == highest).unwrap();
                left += high_pos + 1;
                let highest = (highest - b'0') as usize;
                acc * 10 + highest
            })
        })
        .sum()
}

#[test]
fn default() {
    let input = get_input(25, 3).unwrap();
    // assert_eq!(YY, part1(&input));
    // assert_eq!(YY, part2(&input));
}

#[test]
fn test1() {
    let input = r"987654321111111
811111111111119
234234234234278
818181911112111";
    assert_eq!(357, part1(&input));
}

#[test]
fn test2() {
    let input = r"987654321111111
811111111111119
234234234234278
818181911112111";
    assert_eq!(3121910778619, part2(&input));
}
