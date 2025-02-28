use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a [Output];
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(23, 4).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Vec<Output> {
    input
        .lines()
        .filter_map(|line| {
            let numbers = &line[line.find(':')? + 1..]
                .split_whitespace()
                .collect::<Vec<_>>();
            Some(numbers.len() - numbers.iter().unique().count())
        })
        .collect()
}

fn part1(cards: Input) -> Output {
    cards.iter().map(|count| 1 << count >> 1).sum()
}

fn part2(cards: Input) -> Output {
    let mut card_count = vec![1; cards.len()];
    cards.iter().enumerate().for_each(|(index, count)| {
        let range = index + 1..=index + count;
        let number_of_cards = card_count[index];
        range.for_each(|i| card_count[i] += number_of_cards)
    });
    card_count.iter().sum()
}

#[test]
fn default() {
    let input = get_input(23, 4).unwrap();
    let input = parse_input(&input);
    assert_eq!(23750, part1(&input));
    assert_eq!(13261850, part2(&input));
}
