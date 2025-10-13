use advent::utilities::get_input::get_input;
use utilities::structs::{hexagon::Hexagon, stopwatch::{ReportDuration, Stopwatch}};

type Input = Vec<Hexagon>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(17, 11).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.split(',')
        .map(|dir| dir.into())
        .scan(Hexagon::origin(), |acc, dir| {
            *acc = acc.hex_at(dir);
            Some(*acc)
        })
        .collect()
}

fn part1(path: &Input) -> Output {
    path.last().unwrap().distance(Hexagon::origin())
}

fn part2(path: &Input) -> Output {
    path.iter()
        .map(|hex| hex.distance(Hexagon::origin()))
        .max()
        .unwrap()
}

#[test]
fn default() {
    let input = get_input(17, 11).unwrap();
    let input = parse_input(&input);
    assert_eq!(747, part1(&input));
    assert_eq!(1544, part2(&input));
}

// Input parsed (718μs)
// 1. 747 (5μs)
// 2. 1544 (11μs)
// Total: 737μs
