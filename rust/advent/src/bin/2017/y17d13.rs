use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input = Vec<(usize, usize)>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(17, 13).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.get_numbers().tuples().collect()
}

fn part1(layers: &Input) -> Output {
    layers.iter()
        .map(|&(depth, range)| {
            if depth % ((range - 1) * 2) == 0 {
                depth * range
            } else {
                0
            }
        })
        .sum()
}

fn part2(layers: &Input) -> Output {
    for offset in 0.. {
        if !layers.iter().any(|&(depth, range)| (depth + offset) % ((range - 1) * 2) == 0) {
            return offset;
        }
    }
    unreachable!()
}

#[test]
fn default() {
    let input = get_input(17, 13).unwrap();
    let input = parse_input(&input);
    assert_eq!(1528, part1(&input));
    assert_eq!(3896406, part2(&input));
}

// Input parsed (166μs)
// 1. 1528 (4μs)
// 2. 3896406 (13ms)
// Total: 14ms
