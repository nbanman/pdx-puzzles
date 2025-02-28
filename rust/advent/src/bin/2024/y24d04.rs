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
    let dirs = [-width - 1, -width, -width + 1, -1, 1, width - 1, width, width + 1];
    let starts: Vec<isize> = jumble.iter().enumerate()
        .filter(|&(_, &c)| c == b'X')
        .map(|(idx, _)| idx as isize)
        .collect();
    starts.iter()
        .flat_map(|start| {
            dirs.iter().map(|&dir| {
                (0..3)
                    .scan(*start, |state, _| {
                        *state += dir;
                        Some(*state)
                    })
                    .filter_map(|pos| jumble.try_get(pos))
                    .map(|c| *c as char)
                    .collect::<String>()
            })
        })
        .filter(|word| "MAS" == word)
        .count()
}

fn part2(input: Input) -> Output {
    let width = (input.find('\n').unwrap() + 1) as isize;
    let jumble = input.as_bytes();
    let starts: Vec<isize> = jumble.iter().enumerate()
        .filter(|&(_, &c)| c == b'A')
        .map(|(idx, _)| idx as isize)
        .collect();
    let ms: HashSet<u8> = "MS".as_bytes().iter().copied().collect();
    let lr = [-width - 1, width + 1];
    let rl = [-width + 1, width - 1];
    starts.iter()
        .filter(|&start| {
            let lr: HashSet<u8> = lr.iter()
                .filter_map(|pos| jumble.try_get(start + pos))
                .copied()
                .collect();
            let rl: HashSet<u8> = rl.iter()
                .filter_map(|pos| jumble.try_get(start + pos))
                .copied()
                .collect();
            lr == ms && rl == ms
        })
        .count()
}

#[test]
fn default() {
    let input = get_input(24, 4).unwrap();
    assert_eq!(2534, part1(&input));
    assert_eq!(1866, part2(&input));
}

#[test]
fn examples() {
    let inputs = [r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
", ];
    assert_eq!(18, part1(inputs[0]));
    assert_eq!(9, part2(inputs[0]));
}

// Input parsed (27μs)
// 1. 2534 (1ms)
// 2. 1866 (701μs)
// Total: 1ms
