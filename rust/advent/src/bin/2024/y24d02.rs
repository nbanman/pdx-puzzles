use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::stopwatch::{ReportDuration, Stopwatch},
};

type Input = Vec<Vec<isize>>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 2).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.get_numbers().collect())
        .collect()
}

fn is_safe(level: &[isize]) -> bool {
    let rng = if level[0] < level[1] { -3..=-1 } else { 1..=3 };
    level
        .iter()
        .tuple_windows()
        .all(|(a, b)| rng.contains(&(a - b)))
}

fn is_somewhat_safe(level: &[isize]) -> bool {
    let diffs = level
        .iter()
        .tuple_windows()
        .filter(|&(&a, &b)| b - a > 0)
        .count();
    let last_index = level.len() - 1;
    let penultimate_index = last_index - 1;
    let rng = match diffs {
        0 | 1 => 1..=3,
        x if x == last_index || x == penultimate_index => -3..=-1,
        _ => {
            return false;
        }
    };
    let mut removed = false;
    let mut i = 0;
    while i < last_index {
        if !rng.contains(&(level[i] - level[i + 1])) {
            if removed {
                return false;
            }
            removed = true;
            if i != 0 {
                if !rng.contains(&(level[i - 1] - level[i + 1])) {
                    if i != penultimate_index && !rng.contains(&(level[i] - level[i + 2])) {
                        return false;
                    }
                    i += 1;
                }
            } else if rng.contains(&(level[0] - level[2])) {
                i += 1;
            }
        }
        i += 1;
    }
    true
}

fn solve<F>(levels: &Input, predicate: F) -> Output
where
    F: Fn(&[isize]) -> bool,
{
    levels.iter().filter(|&level| predicate(level)).count()
}

fn part1(levels: &Input) -> Output {
    solve(levels, is_safe)
}

fn part2(levels: &Input) -> Output {
    solve(levels, is_somewhat_safe)
}

#[test]
fn default() {
    let input = get_input(24, 2).unwrap();
    let input = parse_input(&input);
    assert_eq!(591, part1(&input));
    assert_eq!(621, part2(&input));
}

#[test]
fn examples() {
    let inputs = [r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"];
    let input = parse_input(inputs[0]);
    assert_eq!(2, part1(&input));
    assert_eq!(4, part2(&input));
}

// Input parsed (299μs)
// 1. 591 (12μs)
// 2. 621 (24μs)
// Total: 337μs
