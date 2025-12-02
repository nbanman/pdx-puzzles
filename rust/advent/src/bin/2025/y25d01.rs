use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<i64>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(25, 1).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (dir, n) = line.split_at(1);
            n.parse::<i64>().unwrap() * if dir == "L" { -1 } else { 1 }
        })
        .collect()
}

fn part1(input: &Input) -> Output {
    input
        .iter()
        .scan(50i64, |state, &n| {
            *state = (*state + n).rem_euclid(100);
            Some(*state)
        })
        .filter(|&n| n == 0)
        .count()
}

fn part2(input: &Input) -> Output {
    let mut dial = 50;
    let mut clicks = 0;
    for &n in input {
        dial += n;
        if dial <= 0 && n != dial {
            clicks += 1;
        }
        clicks += dial.abs() / 100;
        dial = dial.rem_euclid(100);
    }
    clicks as usize
}

#[test]
fn test1() {
    let input = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
    assert_eq!(3, part1(&parse_input(input)));
}

#[test]
fn test2() {
    let input = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
    assert_eq!(6, part2(&parse_input(input)));
}

#[test]
fn default() {
    let input = get_input(25, 1).unwrap();
    let input = parse_input(&input);
    assert_eq!(1102, part1(&input));
    assert_eq!(6175, part2(&input));
}

// Input parsed (88μs)
// 1. 1102 (20μs)
// 2. 6175 (17μs)
// Total: 128μs
