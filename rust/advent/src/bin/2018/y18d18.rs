use std::iter::successors;

use advent::utilities::get_input::get_input;
use indexmap::IndexSet;
use utilities::structs::{grid::{Grid2, GridIterator}, stopwatch::{ReportDuration, Stopwatch}};

type Output = usize;
type CollectionArea = Grid2<char>;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(18, 18).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> impl Iterator<Item = CollectionArea> + Clone {
    let initial = Grid2::try_from(input).unwrap();
    successors(Some(initial), |prev| {
        prev.into_iter().enumerate()
            .map(|(index, &acre)| {
                let neighbors: Vec<char> = prev
                    .adjacent(index, true)
                    .unwrap()
                    .map(|it| *it.value)
                    .collect();
                match acre {
                    '.' => {
                        if neighbors.iter()
                            .filter(|&&it| it == '|')
                            .count() >= 3 {
                            '|'
                        } else {
                            acre
                        }
                    },
                    '|' => {
                        if neighbors.iter()
                            .filter(|&&it| it == '#')
                            .count() >= 3 {
                            '#'
                        } else {
                            acre
                        }
                    },
                    _ => {
                        if neighbors.contains(&'#') && neighbors.contains(&'|') {
                            '#'
                        } else {
                            '.'
                        }
                    },
                }
            })
            .try_collect_grid(prev.width())
            .ok()
    })
}

fn resource_value(collection_area: &CollectionArea) -> usize {
    let mut pipes = 0;
    let mut pounds = 0;
    for &c in collection_area.iter() {
        if c == '|' {
            pipes += 1;
        } else if c == '#' {
            pounds +=1;
        }
    }
    pipes * pounds
}

fn part1(generator: impl Iterator<Item = CollectionArea>) -> Output {
    resource_value(&generator.take(11).last().unwrap())
}

fn part2(generator: impl Iterator<Item = CollectionArea>) -> Output {
    let mut cache: IndexSet<CollectionArea> = IndexSet::default();
    for collection_area in generator {
        if let Some(repeat_index) = cache.get_index_of(&collection_area) {
            let loop_index = repeat_index + (1_000_000_000 - repeat_index) % (cache.len() - repeat_index);
            return resource_value(&cache.get_index(loop_index).unwrap())
        }
        cache.insert(collection_area);
    }
    unreachable!()
}

#[test]
fn default() {
    let input = get_input(18, 18).unwrap();
    let input = parse_input(&input);
    assert_eq!(605154, part1(input.clone()));
    assert_eq!(200364, part2(input));
}

// Input parsed (44μs)
// 1. 605154 (4ms)
// 2. 200364 (190ms)
// Total: 194ms
