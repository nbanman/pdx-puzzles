use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

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

fn parse_input(input: &str) -> Vec<Vec<isize>> {
    input.lines().map(|line| line.get_numbers().collect()).collect()
}

fn safe(level: &Vec<isize>) -> bool {
    let level: Vec<(isize, isize)> = level.iter().copied().tuple_windows().collect();
    level.iter().all(|(a, b)| (1..=3).contains(&(a - b))) 
        || level.iter().all(|(a, b)| (1..=3).contains(&(b - a))) 
}

fn part1(levels: &Vec<Vec<isize>>) -> usize {
    levels.iter().filter(|&level| safe(level)).count()
}

fn part2(levels: &Vec<Vec<isize>>) -> usize {
    levels.iter()
        .filter(|&level| {
            if safe(level) {
                true
            } else {
                (0..level.len())
                    .map(|i| {
                        level.iter().enumerate()
                            .filter_map(|(idx, &value)| {
                                if i != idx {
                                    Some(value)
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<isize>>()
                    })
                    .any(|abridged_level| safe(&abridged_level))
            }
        })
        .count()
}

#[test]
fn default() {
    let input = get_input(24, 2).unwrap();
    let input = parse_input(&input);
    assert_eq!(591, part1(&input));
    assert_eq!(621, part2(&input));
}

    // Input parsed (302μs)
    // 1. 591 (85μs)
    // 2. 621 (272μs)
    // Total: 662μs
