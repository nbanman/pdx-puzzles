use advent::utilities::get_input::get_input;
use rustc_hash::FxHashSet;
use utilities::{collation::Collate, structs::{coord::Coord2, stopwatch::{ReportDuration, Stopwatch}}};

type Input<'a> = &'a str;
type Output = usize;
type Pos = Coord2;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(15, 3).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn deliver(chars: impl Iterator<Item = char>) -> FxHashSet<Pos> {
    std::iter::once(Pos::origin())
        .chain(chars.scan(Pos::origin(), |state, c| {
            *state = state.char_move(c).unwrap();
            Some(*state)
        }))
        .collect()
}

fn part1(input: Input) -> Output {
    deliver(input.chars()).len()
}

fn part2(input: Input) -> Output {
    input.chars()
        .collate(2)
        .into_iter()
        .map(|thread| deliver(thread.into_iter()))
        .reduce(|acc, other| {
            acc.union(&other).copied().collect()
        })
        .unwrap()
        .len()
}

#[test]
fn default() {
    let input = get_input(15, 3).unwrap();
    assert_eq!(2081, part1(&input));
    assert_eq!(2341, part2(&input));
}

// Input parsed (224μs)
// 1. 2081 (345μs)
// 2. 2341 (360μs)
// Total: 933μs
