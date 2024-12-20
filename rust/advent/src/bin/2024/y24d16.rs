use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 16).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    todo!()
}

fn part1(input: Input) -> Output {
    todo!()
}

fn part2(input: Input) -> Output {
    todo!()
}

#[test]
fn default() {
    let input = get_input(24, 16).unwrap();
    assert_eq!(105496, part1(&input));
    // assert_eq!(524, part2(&input));
}

// #[test]
// fn examples() {
//     let inputs = [r"", ];
//     assert_eq!(Y, part1(input[0]));
//     // assert_eq!(Y, part2(input[0]));
// }