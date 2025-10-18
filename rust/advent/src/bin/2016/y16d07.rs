use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{collation::Collate, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = Vec<Vec<Vec<&'a str>>>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(16, 7).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input<'_> {
    input.lines()
        .map(|line| line.split(['[', ']']).collate(2))
        .collect()
}

fn abba(net: &str) -> bool {
    net.as_bytes().iter().tuple_windows()
        .any(|(a, b, c, d)| a == d && a != b && b == c)
}

fn aba(net: &str) -> impl Iterator<Item = String> {
    net.chars().tuple_windows()
        .filter(|(a, b, c)| a == c && a != b)
        .map(|(a, b, _)| format!("{}{}{}", b, a, b))
}

fn part1(ips: &Input) -> Output {
    ips.iter()
        .filter(|&ip| {
            let supernets = &ip[0];
            let hypernets = &ip[1];
            supernets.iter().any(|&net| abba(net))
                && !hypernets.iter().any(|&net| abba(net))
        })
        .count()
}

fn part2(ips: &Input) -> Output {
    ips.iter()
        .filter(|&ip| {
            let supernets = &ip[0];
            let hypernets = &ip[1];
            supernets.iter()
                .flat_map(|&net| aba(net))
                .any(|aba| hypernets.iter().any(|&net| net.contains(&aba)))
        })
        .count()
}

#[test]
fn default() {
    let input = get_input(16, 7).unwrap();
    let input = parse_input(&input);
    assert_eq!(118, part1(&input));
    assert_eq!(260, part2(&input));
}

// Input parsed (718μs)
// 1. 118 (99μs)
// 2. 260 (767μs)
// Total: 1ms
