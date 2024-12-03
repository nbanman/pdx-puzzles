use advent::utilities::get_input::get_input;
use lazy_regex::Regex;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};
use std::ops::Mul;

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 3).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: Input) -> Output {
    let rx = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    rx.find_iter(input)
        .map(|rx_match| rx_match.as_str().get_numbers().reduce(usize::mul).unwrap())
        .sum()
}

fn part2(input: Input) -> Output {
    let rx = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    rx.find_iter(input)
        .map(|rx_match| rx_match.as_str())
        .fold((0, true), |(sum, active), instruction| {
            let new_active = match instruction {
                "do()" => true,
                "don't()" => false,
                _ => active,
            };
            let new_sum = sum + if active && instruction.starts_with("mul") {
                instruction.get_numbers().reduce(usize::mul).unwrap()
            } else {
                0
            };
            (new_sum, new_active)
        })
        .0
}

#[test]
fn default() {
    let input = get_input(24, 3).unwrap();
    assert_eq!(191183308, part1(&input));
    assert_eq!(92082041, part2(&input));
}

#[test]
fn examples() {
    let inputs = [r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))", 
    r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",];
    assert_eq!(161, part1(inputs[0]));
    assert_eq!(48, part2(inputs[1]));
}

// Input parsed (37μs)
// 1. 191183308 (513μs)
// 2. 92082041 (401μs)
// Total: 956μs
