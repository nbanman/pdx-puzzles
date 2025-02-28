use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<(Int, Int)>;
type Int = i16;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(22, 2).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input
        .split_ascii_whitespace()
        .map(|c| match c {
            "A" | "X" => 0,
            "B" | "Y" => 1,
            "C" | "Z" => 2,
            _ => panic!("Invalid input"),
        })
        .tuples()
        .collect()
}

fn throw_score(my_throw: Int) -> Int {
    my_throw + 1
}

fn outcome_score(my_outcome: Int) -> Int {
    my_outcome * 3
}

fn my_outcome(my_throw: Int, opponent_throw: Int) -> Int {
    (my_throw - opponent_throw + 1).rem_euclid(3)
}

fn my_throw(my_outcome: Int, opponent_throw: Int) -> Int {
    (my_outcome + opponent_throw - 1).rem_euclid(3)
}

fn part1(rounds: &Input) -> Int {
    rounds
        .iter()
        .map(|&(opponent_throw, my_throw)| {
            let my_outcome = my_outcome(my_throw, opponent_throw);
            outcome_score(my_outcome) + throw_score(my_throw)
        })
        .sum()
}

fn part2(rounds: &Input) -> Int {
    rounds
        .iter()
        .map(|&(opponent_throw, my_outcome)| {
            let my_throw = my_throw(my_outcome, opponent_throw);
            outcome_score(my_outcome) + throw_score(my_throw)
        })
        .sum()
}

#[test]
fn default() {
    let input = get_input(22, 2).unwrap();
    let input = parse_input(&input);
    assert_eq!(9241, part1(&input));
    assert_eq!(14610, part2(&input));
}

// Input parsed (51μs)
// 1. 9241 (6μs)
// 2. 14610 (4μs)
// Total: 64μs
