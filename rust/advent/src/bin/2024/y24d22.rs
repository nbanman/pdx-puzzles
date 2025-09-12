use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::iter::successors;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::stopwatch::{ReportDuration, Stopwatch},
};

type Input<'a> = &'a str;
type Output = isize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 22).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn next_secret(prev: Output) -> Output {
    let a = mix_and_prune(prev, prev * 64);
    let b = mix_and_prune(a, a / 32);
    mix_and_prune(b, b * 2048)
}

fn mix_and_prune(prev: Output, next: Output) -> Output {
    let mix = prev ^ next;
    mix % 16777216
}

fn part1(buyers: Input) -> Output {
    buyers.get_numbers()
        .map(|buyer| {
            successors(Some(buyer), |secret| Some(next_secret(*secret)))
                .take(2001)
                .last()
                .unwrap()
        })
        .sum()
}

fn part2(buyers: Input) -> Output {
    let mut exchange_rate = FxHashMap::default();
    for buyer in buyers.get_numbers() {
        let prices: Vec<Output> = successors(Some(buyer), |secret| Some(next_secret(*secret)))
            .take(2001)
            .map(|secret| secret % 10)
            .collect();

        let changes_seq = prices.iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .tuple_windows()
            .map(|(a, b, c, d)| vec![a, b, c, d]);

        for (&price, changes) in prices.iter().dropping(4).zip(changes_seq) {
            let buyer_rate = exchange_rate.entry(buyer).or_insert(FxHashMap::default());
            if buyer_rate.contains_key(&changes) { continue; }
            buyer_rate.insert(changes, price);
        }
    }

    let mut total_rates = FxHashMap::default();
    for (_, rate) in exchange_rate {
        for (changes, price) in rate {
            let new_price = total_rates.entry(changes).or_insert(0);
            *new_price += price;
        }
    }

    *total_rates.values().max().unwrap()
}

#[test]
fn default() {
    let input = get_input(24, 22).unwrap();
    assert_eq!(16953639210, part1(&input));
    assert_eq!(1863, part2(&input));
}

// Input parsed (22μs)
// 1. 16953639210 (18ms)
// 2. 1863 (564ms)
// Total: 582ms