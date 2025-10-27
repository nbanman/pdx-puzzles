use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
use rustc_hash::FxHashSet;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = usize;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(15, 20).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.parse().unwrap()
}

fn solve<F>(minimum_presents: Input, multiplier: usize, predicate: F) -> usize
where
    F: Fn(usize, usize) -> bool + std::marker::Sync,
{
    (1..)
        .par_bridge()
        .map(|house_number| {
            let elves: Vec<usize> = expand_factors(prime_factors(house_number), vec![1])
                .into_iter()
                .filter(|&it| predicate(house_number, it))
                .collect_vec();
            let presents = elves.into_iter().fold(0, |acc, i| acc + i * multiplier);
            (house_number, presents >= minimum_presents)
        })
        .find_any(|(_, predicate)| *predicate)
        .map(|(house_number, _)| house_number)
        .unwrap()
}

fn prime_factors(mut n: usize) -> Vec<usize> {
    let mut factors = Vec::new();
    while n & 1 == 0 {
        factors.push(2);
        n /= 2;
    }

    for i in (3..=n.isqrt()).step_by(2) {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
    }

    if n > 2 {
        factors.push(n);
    }
    factors
}

fn expand_factors(prime_factors: Vec<usize>, factors: Vec<usize>) -> Vec<usize> {
    if prime_factors.is_empty() {
        factors
    } else {
        let first = prime_factors[0];
        let latest: FxHashSet<usize> = match prime_factors.iter().rposition(|&it| it == first) {
            Some(i) => &prime_factors[..=i],
            None => &[],
        }
            .iter()
            .scan(1, |state, &factor| {
                *state *= factor;
                Some(*state)
            })
            .collect();
        let mut new_factors = Vec::new();
        for factor in factors {
            new_factors.push(factor);
            new_factors.extend(latest.iter().map(|&it| it * factor));
        }
        let prime_factors = prime_factors.into_iter()
            .filter(|it| !latest.contains(it))
            .collect_vec();
        expand_factors(prime_factors, new_factors)
    }
}

fn part1(minimum_presents: Input) -> Output {
    solve(minimum_presents, 10, |_, _| true)
}

fn part2(minimum_presents: Input) -> Output {
    solve(minimum_presents, 11, |house_number, elf| elf * 50 > house_number)
}

#[test]
fn default() {
    let input = get_input(15, 20).unwrap();
    let input = parse_input(&input);
    assert_eq!(776160, part1(input));
    assert_eq!(786240, part2(input));
}

// Input parsed (18μs)
// 1. 776160 (64ms)
// 2. 786240 (62ms)
// Total: 127ms
