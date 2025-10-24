use std::ops::Mul;
use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input = Vec<[usize; 3]>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(15, 2).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.get_numbers().tuples()
        .map(|(a, b, c)| {
            let mut dimensions = [a, b, c];
            dimensions.sort_unstable();
            dimensions
        })
        .collect()
}

fn part1(boxes: &Input) -> Output {
    boxes.iter()
        .map(|dimensions| {
            let surface_area = dimensions.into_iter().tuple_combinations()
                .map(|(&a, &b)| 2 * a * b)
                .sum::<usize>();
            let smallest_side_area = dimensions[0] * dimensions[1];
            surface_area + smallest_side_area
        })
        .sum()
}

fn part2(boxes: &Input) -> Output {
    boxes.iter()
        .map(|dimensions| {
            let cubic_volume = dimensions.iter().copied().reduce(usize::mul).unwrap();
            let ribbon_to_wrap = 2 * (dimensions[0] + dimensions[1]);
            cubic_volume + ribbon_to_wrap
        })
        .sum()
}

#[test]
fn default() {
    let input = get_input(15, 2).unwrap();
    let input = parse_input(&input);
    assert_eq!(1588178, part1(&input));
    assert_eq!(3783758, part2(&input));
}

// Input parsed (211μs)
// 1. 1588178 (8μs)
// 2. 3783758 (2μs)
// Total: 224μs
