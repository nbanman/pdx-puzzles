use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::stopwatch::{ReportDuration, Stopwatch},
};

type Input<'a> = &'a str;
type Output = i64;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 13).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn solve(input: Input, additional: i64) -> Output {
    input
        .get_numbers()
        .tuples()
        .filter_map(|(a1, a2, b1, b2, c1, c2)| {
            let c1 = c1 + additional;
            let c2 = c2 + additional;
            tokens(a1, a2, b1, b2, c1, c2)
        })
        .map(|(a, b)| a * 3 + b)
        .sum()
}

fn tokens(a1: i64, a2: i64, b1: i64, b2: i64, c1: i64, c2: i64) -> Option<(i64, i64)> {
    // use elimination method of solving system of equations. Using these equations:
    //   1) a1x + b1y = c1
    //   2) a2x + b2y = c2
    // we'll find the lcm of a1 and a2. then we'll multiply equation 1 so that a1 is the lcm,
    // and we'll multiply equation 2 so that a2 is -lcm. Then add the two equations together,
    // which gives all we need to solve for y
    let common_multiple = a1 * a2;
    let f1 = common_multiple / a1;
    let f2 = -common_multiple / a2;
    let y_numerator = c1 * f1 + c2 * f2;
    let y_denominator = b1 * f1 + b2 * f2;
    let y = y_numerator / y_denominator;
    if y * y_denominator != y_numerator {
        return None;
    }
    let x_numerator = c1 - b1 * y;
    let x_denominator = a1;
    let x = x_numerator / x_denominator;
    if x * x_denominator != x_numerator {
        return None;
    }
    Some((x, y))
}

fn part1(input: Input) -> Output {
    solve(input, 0)
}
fn part2(input: Input) -> Output {
    solve(input, 10_000_000_000_000)
}

#[test]
fn default() {
    let input = get_input(24, 13).unwrap();
    assert_eq!(37128, part1(&input));
    assert_eq!(74914228471331, part2(&input));
}

// Input parsed (27μs)
// 1. 37128 (69μs)
// 2. 74914228471331 (65μs)
// Total: 165μs
