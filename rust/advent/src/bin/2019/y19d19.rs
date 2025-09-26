use advent::utilities::{get_input::get_input, intcode::{IntCode, State}};
use itertools::Itertools;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = IntCode;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(19, 19).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.into()
}

fn part1(ic: &Input) -> usize {
    let mut count = 0;
    for (y, x) in (0..50).cartesian_product(0..50) {
        let mut ic = ic.clone();
        ic.input(x);
        ic.input(y);
        let (_, output) = ic.run_while_able();
        count += output.into_iter().filter(|&n| n == 1).count();
    }
    count
}

fn part2(ic: &Input) -> i64 {
    let mut right_edge = 0;
    let mut left_edge = 0;
    let mut y = 0;
    let width = 100;

    while left_edge + width - 1 != right_edge {
        y += 1;
        let mut right_add = 2;
        loop {
            let mut ic = ic.clone();
            ic.input(right_edge + right_add);
            ic.input(y);
            if let State::Output(v) = ic.run() {
                if v == 0 {
                    right_edge += right_add - 1;
                    break;
                }
            }
            right_add += 1;
        }
        let mut left_add = 1;
        loop {
            let mut ic = ic.clone();
            ic.input(left_edge + left_add);
            ic.input(y + width - 1);
            if let State::Output(v) = ic.run() {
                if v == 1 {
                    left_edge += left_add;
                    break;
                }
            }
            left_add += 1;
        }
    }
    left_edge * 10_000 + y
}

#[test]
fn default() {
    let input = get_input(19, 19).unwrap();
    let input = parse_input(&input);
    assert_eq!(179, part1(&input));
    assert_eq!(9760485, part2(&input));
}

// Input parsed (32μs)
// 1. 179 (6ms)
// 2. 9760485 (4ms)
// Total: 11ms
