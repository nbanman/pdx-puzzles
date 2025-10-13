use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = Vec<Vec<&'a str>>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(17, 4).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input<'_> {
    input.lines().map(|line| line.split(' ').collect()).collect()
}

fn count_unique<T: Eq + std::hash::Hash>(passphrases: &Vec<Vec<T>>) -> Output {
    passphrases.into_iter()
        .filter(|it| it.len() == it.iter().collect::<FxHashSet<&T>>().len())
        .count()
}

fn part1(passphrases: &Input) -> Output {
    count_unique(passphrases)
}

fn part2(passphrases: &Input) -> Output {
    let passphrases = passphrases.into_iter()
        .map(|phrase| {
            phrase.into_iter()
                .map(|&word| {
                    word.as_bytes().iter().copied().counts().into_iter().sorted_unstable().collect_vec()
                })
                .collect_vec()
        })
        .collect_vec();
    count_unique(&passphrases)
}

#[test]
fn default() {
    let input = get_input(17, 4).unwrap();
    let input = parse_input(&input);
    assert_eq!(455, part1(&input));
    assert_eq!(186, part2(&input));
}

// Input parsed (176μs)
// 1. 455 (56μs)
// 2. 186 (1ms)
// Total: 1ms
