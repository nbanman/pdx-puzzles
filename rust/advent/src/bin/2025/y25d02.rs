use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::stopwatch::{ReportDuration, Stopwatch},
};

type Input = Vec<(u64, u64)>;
type Output = u64;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(25, 2).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input
        .get_numbers()
        .tuples()
        .map(|(lo, hi)| (lo, hi))
        .collect()
}

fn get_next_invalid_1(n: u64) -> u64 {
    let digits = get_digits(n);
    if digits & 1 == 1 {
        next_if_odd(digits)
    } else {
        get_next_by_portion(n, 2, digits)
    }
}

fn get_digits(mut n: u64) -> u32 {
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count
}

fn next_if_odd(digits: u32) -> u64 {
    match digits {
        1 => 11,
        3 => 1010,
        5 => 100100,
        7 => 10001000,
        9 => 1000010000,
        d => panic!("{} is invalid! must be odd between 1-9", d),
    }
}

fn get_next_by_portion(n: u64, portion: u32, digits: u32) -> u64 {
    let top = n / (10u64.pow(digits / portion * (portion - 1)));
    let candidate = (1..portion).fold(top, |acc, _| {
        acc * 10u64.pow(digits / portion) + top
    });
    if candidate >= n {
        return candidate;
    }
    (1..portion).fold(top + 1, |acc, _| {
        acc * 10u64.pow(digits / portion) + top + 1
    })
}

fn get_next_invalid_2(n: u64) -> u64 {
    let digits = get_digits(n);
    if digits == 1 {
        return 11;
    }
    let portions = match digits {
        2 => vec![2],
        3 => vec![3],
        4 => vec![2],
        5 => vec![5],
        6 => vec![2, 3],
        7 => vec![7],
        8 => vec![2],
        9 => vec![3],
        10 => vec![2, 5],
        d => panic!("{} is invalid!", d),
    };
    portions
        .into_iter()
        .map(|portion| get_next_by_portion(n, portion, digits))
        .min()
        .unwrap()
}

fn solve<F>(ids: &Input, get_next_invalid: F) -> Output
where
    F: Fn(u64) -> u64,
{
    let mut invalid_ids = 0;
    for &(lo, hi) in ids {
        let mut n = lo;
        while n <= hi {
            let next_invalid = get_next_invalid(n);
            if next_invalid <= hi {
                invalid_ids += next_invalid;
            }
            n = next_invalid + 1;
        }
    }
    invalid_ids
}

fn part1(ids: &Input) -> Output {
    solve(ids, get_next_invalid_1)
}

fn part2(ids: &Input) -> Output {
    solve(ids, get_next_invalid_2)
}

#[test]
fn default() {
    let input = get_input(25, 2).unwrap();
    let input = parse_input(&input);
    assert_eq!(28846518423, part1(&input));
    assert_eq!(31578210022, part2(&input));
}

#[test]
fn test1() {
    let input = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
    let input = parse_input(input);
    assert_eq!(1227775554, part1(&input));
}

#[test]
fn test2() {
    let input = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
    let input = parse_input(input);
    assert_eq!(4174379265, part2(&input));
}

// Input parsed (20μs)
// 1. 28846518423 (8μs)
// 2. 31578210022 (14μs)
// Total: 45μs
