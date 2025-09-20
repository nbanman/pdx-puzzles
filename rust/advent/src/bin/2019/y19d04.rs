use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(19, 4).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}
fn get_passwords(input: Input) -> Vec<String> {
    input
        .split_once('-')
        .map(|(start, end)| start.parse::<usize>().unwrap()..=end.trim_end().parse().unwrap())
        .unwrap()
        .map(|i| i.to_string())
        .filter(|password| {
            let zipped_pass_string: Vec<(u8, u8)> = password
                .as_bytes()
                .iter()
                .copied()
                .tuple_windows()
                .collect();
            !zipped_pass_string.iter().any(|(prev, next)| prev > next)
                && zipped_pass_string.iter().any(|(prev, next)| prev == next)
        })
        .collect()
}

fn part1(input: Input) -> Output {
    get_passwords(input).len()
}

fn part2(input: Input) -> Output {
    get_passwords(input)
        .into_iter()
        .filter(|password| {
            let password = password.as_bytes();
            let first = &password[0..3];
            if first[0] == first[1] && first[1] != first[2] {
                true
            } else {
                let last = &password[password.len() - 3..];
                if last[0] != last[1] && last[1] == last[2] {
                    true
                } else {
                    password
                        .iter()
                        .tuple_windows()
                        .any(|(a, b, c, d)| a != b && b == c && c != d)
                }
            }
        })
        .count()
}

#[test]
fn default() {
    let input = get_input(19, 4).unwrap();
    assert_eq!(466, part1(&input));
    assert_eq!(292, part2(&input));
}

// Input parsed (15μs)
// 1. 466 (14ms)
// 2. 292 (14ms)
// Total: 28ms
