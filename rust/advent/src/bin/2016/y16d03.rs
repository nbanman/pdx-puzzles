use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{collation::Collate, parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input = Vec<usize>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(16, 3).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.get_numbers().collect()
}

fn solve<'a>(numbers: &Input) -> Output
{
    numbers
        .into_iter()
        .copied()
        .chunks(3)
        .into_iter()
        .map(|chunk| chunk.sorted_unstable().collect_tuple().unwrap())
        .filter(|&(a, b, c)| a + b > c)
        .count()
}

fn part1(numbers: &Input) -> Output {
    solve(numbers)
}

fn part2(numbers: &Input) -> Output {
    let numbers = numbers.into_iter()
        .copied()
        .collate(3)
        .into_iter()
        .flat_map(|collation| collation.into_iter())
        .collect();
    solve(&numbers)
}

#[test]
fn default() {
    let input = get_input(16, 3).unwrap();
    let input = parse_input(&input);
    assert_eq!(1032, part1(&input));
    assert_eq!(1838, part2(&input));
}

// Input parsed (102μs)
// 1. 1032 (68μs)
// 2. 1838 (125μs)
// Total: 299μs
