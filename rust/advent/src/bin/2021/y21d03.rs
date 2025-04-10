use std::cmp::Ordering;

use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<Vec<bool>>;
type Output = usize;

trait Let: Sized {
    fn let_<R>(self, f: impl FnOnce(Self) -> R) -> R {
        f(self)
    }
}

impl<T> Let for T {}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(21, 3).unwrap();
    let input = parse_input(input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: String) -> Input {
    input.lines()
        .map(|line| line.chars().map(|c| c == '1').collect())
        .collect()
}

fn find_rate(codes: &Input, target: bool) -> Output {
    let mut v = 0;
    for i in 0..codes.first().unwrap().len() {
        v *= 2;
        let n = codes.iter()
            .filter(|code| code[i] == target)
            .count();
        if n * 2 >= codes.len() {
            v += 1;
        }
    } 
    v
}

fn find_rating<F>(codes: &Input, predicate: F) -> Output 
where F: Fn(Ordering) -> bool
{
    let mut codes = codes.clone();
    for i in 0..codes.first().unwrap().len() {
        codes = codes.clone().into_iter()
            .filter(|code| {
                let criteria = codes
                    .iter()
                    .filter(|it| it[i])
                    .count()
                    .let_(|it| it * 2)
                    .cmp(&codes.len());
               predicate(criteria) == code[i]
            })
            .collect();
        if codes.len() == 1 { break; }
    }
    codes[0].iter().fold(0, |acc, b| {
       let next = if *b { 1 } else { 0 };
       acc * 2 + next 
    })
}

fn part1(codes: &Input) -> Output {
    let gamma = find_rate(codes, true);
    let epsilon = find_rate(codes, false);
    gamma * epsilon
}

fn part2(codes: &Input) -> Output {
    let o2_gen = find_rating(codes, |ord| {
        match ord {
            Ordering::Equal | Ordering::Greater => true,
            _ => false,
        } 
    });
     
    let co2_scrubber = find_rating(codes, |ord| ord == Ordering::Less);
    o2_gen * co2_scrubber
}

#[test]
fn default() {
    let input = get_input(21, 3).unwrap();
    let input = parse_input(input);
    assert_eq!(3969000, part1(&input));
    assert_eq!(4267809, part2(&input));
}

// Input parsed (80μs)
// 1. 3969000 (17μs)
// 2. 4267809 (1ms)
// Total: 1ms