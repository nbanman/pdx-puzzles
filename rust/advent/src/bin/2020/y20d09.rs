use advent::utilities::get_input::get_input;
use rustc_hash::FxHashMap;
use std::collections::HashMap;
use utilities::parsing::get_numbers::ContainsNumbers;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<Output>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(20, 9).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.get_numbers().collect()
}

fn part1(numbers: &Input) -> Output {
    let preamble = 25;

    // Prep cache
    let mut cache: HashMap<Output, Output, _> = FxHashMap::default();
    for (idx, &l) in numbers[0..preamble - 1].iter().enumerate() {
        for &u in numbers[idx + 1..preamble].iter() {
            cache.insert(l + u, idx);
        }
    }

    // Try each subsequent number
    for i in preamble..numbers.len() {
        let current = numbers[i];
        let index_of_sum = cache.get(&current).copied();
        if let Some(index_of_sum) = index_of_sum {
            if index_of_sum < i - preamble {
                return current;
            }
        } else {
            return current;
        }
        for l in i + 1 - preamble..i {
            let next = numbers[l] + current;
            let existing = cache.get(&next);
            if existing.is_none() || l > *existing.unwrap() {
                cache.insert(next, l);
            }
        }
    }
    unreachable!();
}

fn part2(numbers: &Input) -> Output {
    let weakness = part1(numbers);
    let mut l = 0;
    let mut u = 1;
    let mut sum = numbers[l];
    loop {
        sum += numbers[u];
        if sum == weakness {
            return answer(numbers, l, u);
        }
        if sum > weakness {
            sum -= numbers[l];
            l += 1;
            if sum == weakness {
                return answer(numbers, l, u);
            }
            while sum > weakness {
                sum -= numbers[u];
                u -= 1;
                if sum == weakness {
                    return answer(numbers, l, u);
                }
            }
        }
        u += 1;
    }
}

fn answer(numbers: &Input, l: Output, u: Output) -> Output {
    let min = numbers[l..=u].iter().min().unwrap();
    let max = numbers[l..=u].iter().max().unwrap();
    min + max
}

#[test]
fn default() {
    let input = get_input(20, 9).unwrap();
    let input = parse_input(&input);
    assert_eq!(552655238, part1(&input));
    assert_eq!(70672245, part2(&input));
}

// Input parsed (41μs)
// 1. 552655238 (419μs)
// 2. 70672245 (346μs)
// Total: 809μs