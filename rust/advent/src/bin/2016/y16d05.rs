use advent::utilities::get_input::get_input;
use itertools::Itertools;
use md5::{Digest, Md5};
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Output = String;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(16, 5).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(seed: &str) -> impl Iterator<Item = [u8; 16]> + Clone {
    let mut hasher = Md5::new();
    (0..)
        .map(move |i| {
            let data = format!("{}{}", seed, i);
            Digest::update(&mut hasher, &data);
            Digest::finalize_reset(&mut hasher).into()
        })
        .filter(|digest: &[u8; 16]| digest[0] == 0 && digest[1] == 0 && digest[2] < 16)
}

fn part1(hashes: impl Iterator<Item = [u8; 16]>) -> Output {
    hashes
        .map(|hash| {
            let b = hash[2] & 15;
            (if b < 10 { b + 48 } else { b + 87 }) as char
        })
        .take(8)
        .collect()
}

fn part2(hashes: impl Iterator<Item = [u8; 16]>) -> Output {
    hashes
        .filter(|hash| hash[2] & 15 < 8)
        .unique_by(|hash| hash[2] & 15)
        .take(8)
        .sorted_unstable_by_key(|hash| hash[2] & 15)
        .map(|hash| {
            let b = hash[3] >> 4;
            (if b < 10 { b + 48 } else { b + 87 }) as char
        })
        .collect()
}

#[test]
fn default() {
    let input = get_input(16, 5).unwrap();
    let input = parse_input(&input);
    assert_eq!("4543c154".to_string(), part1(input.clone()));
    assert_eq!("1050cbbd".to_string(), part2(input));
}

// Input parsed (18μs)
// 1. 4543c154 (1.468s)
// 2. 1050cbbd (3.932s)
// Total: 5.400s
