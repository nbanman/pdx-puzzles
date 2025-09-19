use advent::utilities::get_input::get_input;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(19, 1).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn base_fuel(weight: usize) -> usize {
    (weight / 3).checked_sub(2).unwrap_or_default()
}

fn total_fuel(weight: usize) -> usize {
    let mut remaining = weight;
    let mut total = 0;
    while remaining != 0 {
        remaining = base_fuel(remaining);
        total += remaining;            
    }
    total
}
fn part1(modules: Input) -> Output {
    modules.get_numbers().map(base_fuel).sum()
}

fn part2(modules: Input) -> Output {
    modules.get_numbers().map(total_fuel).sum()
}

#[test]
fn default() {
    let input = get_input(19, 1).unwrap();
    assert_eq!(3325347, part1(&input));
    assert_eq!(4985145, part2(&input));
}

// Input parsed (15μs)
// 1. 3325347 (6μs)
// 2. 4985145 (4μs)
// Total: 28μs
