use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{
    parsing::get_numbers::get_numbers,
    structs::stopwatch::{ReportDuration, Stopwatch},
};

type Input = Vec<usize>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(21, 1).unwrap();
    let input = get_numbers(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(measurements: &Input) -> Output {
    measurements
        .iter()
        .tuple_windows()
        .filter(|&(a, b)| a < b)
        .count()
}

fn part2(measurements: &Input) -> Output {
    measurements
        .windows(3)
        .map(|it| it.iter().sum::<usize>())
        .tuple_windows()
        .filter(|&(a, b)| a < b)
        .count()
}

#[test]
fn default() {
    let input = get_input(21, 1).unwrap();
    let input = get_numbers(&input);
    assert_eq!(1342, part1(&input));
    assert_eq!(1378, part2(&input));
}

// Input parsed (44μs)
// 1. 1342 (5μs)
// 2. 1378 (2μs)
// Total: 55μs