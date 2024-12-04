use std::collections::HashSet;

use advent::utilities::get_input::get_input;
use utilities::{parsing::try_get::TryGet, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 4).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: Input) -> Output {
    let width = (input.find('\n').unwrap() + 1) as isize;
    let jumble = input.as_bytes();
    let xmas: HashSet<u8> = "MAS".as_bytes().into_iter().cloned().collect();
    let dirs = [-width - 1, -width, -width + 1, -1, 1, width - 1, width, width + 1];
    let starts: Vec<isize> = jumble.iter().enumerate()
        .filter(|(_, &c)| c == b'X')
        .map(|(idx, _)| idx as isize)
        .collect();
    starts.iter()
        .flat_map(|start| {
            dirs.iter().map(|&dir| {
                (0..4)
                    .scan(*start, |state, _| {
                        *state += dir;
                        Some(*state)
                    })
                    .map(|pos| *jumble.try_get(pos).unwrap())
                    .collect::<HashSet<u8>>()
            })
        })
        .filter(|word| xmas == *word)
        .count()
}

fn part2(input: Input) -> Output {
    3
}

#[test]
fn default() {
    let input = get_input(24, X).unwrap();
    let input = parse_input(&input);
    assert_eq!(X, part1(&input));
    // assert_eq!(X, part2(&input));
}

#[test]
fn examples() {
    let inputs = [r"", ];
    let input = parse_input(&inputs[0]);
    assert_eq!(X, part1(&input));
    // assert_eq!(X, part2(&input));
}