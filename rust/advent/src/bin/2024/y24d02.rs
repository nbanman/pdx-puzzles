use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

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
    input.lines().map(|line| line.get_numbers().collect()).collect()
}

fn safe(level: &Vec<isize>) -> bool {
    let rng = if level[0] < level[1] { -3..=-1 } else { 1..=3 };
    level.iter().tuple_windows().all(|(a, b)| rng.contains(&(a - b)))
}

fn part1(levels: &Input) -> Output {
    levels.iter().filter(|&level| safe(level)).count()
}

fn part2(levels: &Input) -> Output {
    levels.iter()
        .filter(|&level| {
            if safe(level) {
                true
            } else {
                (0..level.len())
                    .map(|exclude| {
                        level.iter().enumerate()
                            .filter_map(|(idx, &value)| {
                                if idx == exclude {
                                    None
                                } else {
                                    Some(value)
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

#[test]
fn examples() {
    let inputs = [r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
", ];
    let input = parse_input(&inputs[0]);
    assert_eq!(2, part1(&input));
    assert_eq!(4, part2(&input));
}

// Input parsed (300μs)
// 1. 591 (16μs)
// 2. 621 (156μs)
// Total: 474μs
