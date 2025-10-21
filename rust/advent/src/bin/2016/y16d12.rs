use advent::utilities::get_input::get_input;
use advent::utilities::assembunny::Assembunny;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Assembunny;
type Output = i64;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(16, 12).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.into()
}

fn part1(mut asmb: Input) -> Output {
    asmb.run(None);
    asmb['a']
}

fn part2(mut asmb: Input) -> Output {
    asmb['c'] = 1;
    asmb.run(None);
    asmb['a']
}

#[test]
fn default() {
    let input = get_input(16, 12).unwrap();
    let input = parse_input(&input);
    assert_eq!(318117, part1(input.clone()));
    assert_eq!(9227771, part2(input));
}

// Input parsed (510μs)
// 1. 318117 (2ms)
// 2. 9227771 (62ms)
// Total: 65ms
