use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = &'a str;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(25, 3);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: Input) -> usize {
    input.get_numbers::<usize>().sorted_unstable().unique().sum()
}

fn part2(input: Input) -> usize {
    input.get_numbers::<usize>().sorted().unique().take(20).sum()
}

fn part3(input: Input) -> usize {
    *input.get_numbers::<usize>().counts().values().max().unwrap()
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(25, 3);
    assert_eq!(2569, part1(&input1));
    assert_eq!(296, part2(&input2));
    assert_eq!(3204, part3(&input3));
}

// Input parsed (41μs)
// 1. 2569 (30μs)
// 2. 296 (14μs)
// 3. 3204 (172μs)
// Total: 261μs
