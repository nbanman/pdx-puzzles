use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(18, 2).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(box_ids: Input) -> usize {
    let frequencies: Vec<FxHashSet<usize>> = box_ids.lines()
        .map(|id| id.chars().counts().values().copied().collect())
        .collect();
    
    frequencies.iter().filter(|it| it.contains(&2)).count() *
        frequencies.iter().filter(|it| it.contains(&3)).count()
        
}

fn part2(box_ids: Input) -> String {
    box_ids.lines()
        .combinations(2)
        .map(|combo| combo[0].chars().zip(combo[1].chars()))
        .find(|pair| pair.clone().filter(|(a, b)| a != b).count() == 1)
        .map(|pair| {
            pair
                .filter(|(a, b)| a == b)
                .map(|(a, _)| a)
                .collect::<String>()
        })
        .unwrap()
}

#[test]
fn default() {
    let input = get_input(18, 2).unwrap();
    assert_eq!(7688, part1(&input));
    assert_eq!("lsrivmotzbdxpkxnaqmuwcchj".to_string(), part2(&input));
}

// Input parsed (18μs)
// 1. 7688 (249μs)
// 2. lsrivmotzbdxpkxnaqmuwcchj (911μs)
// Total: 1ms
