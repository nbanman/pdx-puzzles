use std::cmp::min;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{
    parsing::get_numbers::get_numbers,
    structs::stopwatch::{ReportDuration, Stopwatch},
};

type Input = (Vec<i64>, Vec<Vec<Listing>>);
type Output = i64;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(23, 5).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Clone, Copy, Debug)]
struct Listing {
    source_start: i64,
    offset: i64,
    len: i64,
}

impl Listing {
    fn source_end(&self) -> i64 {
        self.source_start + self.len - 1
    }
}

fn parse_input(input: &str) -> Input {
    let stanzas: Vec<Vec<i64>> = input.split("\n\n").map(get_numbers).collect();

    let seeds = stanzas[0].to_owned();
    let conversions = stanzas[1..]
        .iter()
        .map(|map_numbers| {
            map_numbers
                .iter()
                .tuples()
                .map(|(destination_start, source_start, length)| Listing {
                    source_start: *source_start,
                    offset: destination_start - source_start,
                    len: *length,
                })
                .sorted_by_key(|listing| listing.source_start)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (seeds, conversions)
}

fn solve(conversions: &[Vec<Listing>], seed_ranges: Vec<(i64, i64)>) -> Output {
    seed_ranges
        .iter()
        .map(|seed_range| {
            let sub_ranges: Vec<(i64, i64)> =
                conversions
                    .iter()
                    .fold(vec![seed_range.to_owned()], |ranges, listings| {
                        let test: Vec<(i64, i64)> = ranges
                            .iter()
                            .flat_map(|range| {
                                let mut sub_ranges: Vec<(i64, i64)> = Vec::new();
                                let (range_first, range_last) = range;
                                let last = listings.iter().fold(*range_first, |next, listing| {
                                    if range_last >= &listing.source_start
                                        && next <= listing.source_end()
                                    {
                                        if next < listing.source_start {
                                            sub_ranges
                                                .push((next.to_owned(), listing.source_start - 1));
                                        }
                                        let se = listing.source_end();
                                        let map_end = min(range_last, &se);
                                        sub_ranges.push((
                                            next + listing.offset,
                                            map_end + listing.offset,
                                        ));
                                        *map_end + 1
                                    } else {
                                        next
                                    }
                                });
                                if &last <= range_last {
                                    sub_ranges.push((last.to_owned(), range_last.to_owned()));
                                }
                                sub_ranges
                            })
                            .collect();
                        test
                    });

            sub_ranges
                .iter()
                .map(|(first, _)| first.to_owned())
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

fn part1(input: &Input) -> Output {
    let (seeds, conversions) = input;
    let seed_ranges: Vec<(i64, i64)> = seeds.iter().map(|seed| (*seed, *seed)).collect();
    solve(conversions, seed_ranges)
}

fn part2(input: &Input) -> Output {
    let (seeds, conversions) = input;
    let seed_ranges: Vec<(i64, i64)> = seeds
        .chunks(2)
        .filter_map(|chunk| {
            if let [start, length] = chunk {
                Some((*start, start + length - 1))
            } else {
                None
            }
        })
        .collect();
    solve(conversions, seed_ranges)
}

#[test]
fn default() {
    let input = get_input(23, 5).unwrap();
    let input = parse_input(&input);
    assert_eq!(379811651, part1(&input));
    assert_eq!(27992443, part2(&input));
}

// Input parsed (50μs)
// 1. 379811651 (12μs)
// 2. 27992443 (37μs)
// Total: 102μs