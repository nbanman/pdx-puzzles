use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = usize;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 3).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    todo!()
}

fn part1(input: &Input) -> Output {
    todo!()
}

fn part2(input: &Input) -> Output {
    todo!()
}

#[test]
fn default() {
    let input = get_input(24, 2).unwrap();
    let input = parse_input(&input);
    assert_eq!(0, part1(&input));
    assert_eq!(0, part2(&input));
}

#[test]
fn examples() {
    let inputs = [r"", ];
    let input = parse_input(&inputs[0]);
    assert_eq!(0, part1(&input));
    assert_eq!(0, part2(&input));
}