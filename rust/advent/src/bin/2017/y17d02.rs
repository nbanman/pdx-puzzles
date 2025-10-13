use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{
    minmax::minmax,
    parsing::get_numbers::ContainsNumbers,
    structs::stopwatch::{ReportDuration, Stopwatch},
};

type Input = Vec<Vec<usize>>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(17, 2).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.get_numbers().collect())
        .collect()
}

fn solve(spreadsheet: &Input, line_op: fn(&Vec<usize>) -> usize) -> Output {
    spreadsheet.iter().map(line_op).sum()
}

fn part1(input: &Input) -> Output {
    solve(input, |row| {
        let (min, max) = row.iter().minmax().into_option().unwrap();
        max - min
    })
}

fn part2(input: &Input) -> Output {
    solve(input, |row| {
        row.iter()
            .tuple_combinations()
            .map(|(a, b)| {
                let (min, max) = minmax(a, b);
                if max % min == 0 { max / min } else { 0 }
            })
            .sum()
    })
}

#[test]
fn default() {
    let input = get_input(17, 2).unwrap();
    let input = parse_input(&input);
    assert_eq!(45972, part1(&input));
    assert_eq!(326, part2(&input));
}

// Input parsed (20μs)
// 1. 45972 (4μs)
// 2. 326 (4μs)
// Total: 32μs
