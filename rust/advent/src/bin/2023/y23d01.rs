use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(23, 1).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn calibrate(input: Input) -> usize {
    input
        .lines()
        .map(|line| {
            let first = line
                .chars()
                .find(|c| c.is_digit(10))
                .unwrap()
                .to_digit(10)
                .unwrap() as usize;
            let second = line
                .chars()
                .rfind(|c| c.is_digit(10))
                .unwrap()
                .to_digit(10)
                .unwrap() as usize;
            first * 10 + second
        })
        .sum()
}


fn part1(input: Input) -> Output {
    calibrate(input)
}

fn part2(input: Input) -> Output {
    let replacements = [
        ["one", "o1e"],
        ["two", "t2o"],
        ["three", "t3e"],
        ["four", "4"],
        ["five", "5e"],
        ["six", "6"],
        ["seven", "7n"],
        ["eight", "e8t"],
        ["nine", "n9e"],
    ];

    let replaced_input = replacements
        .iter()
        .fold(input.to_string(), |acc, [original, replacement]| {
            acc.replace(original, &replacement)
        });

    calibrate(&replaced_input)
}

#[test]
fn default() {
    let input = get_input(23, 1).unwrap();
    assert_eq!(54388, part1(&input));
    assert_eq!(53515, part2(&input));
}
