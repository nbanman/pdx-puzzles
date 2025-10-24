use advent::utilities::get_input::get_input;
use md5::{Digest, Md5};
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(15, 4).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn solve(salt: Input, digit_length: usize) -> Output {
    let mut hasher = Md5::new();
    let mut buf = [0u8; 32];
    (1..)
        .find(|i| {
            let message = format!("{}{}", salt, i);
            hasher.update(&message);
            base16ct::lower::encode_str(&hasher.finalize_reset(), &mut buf)
                .unwrap()
                .as_bytes()
                .iter()
                .take(digit_length)
                .all(|b| *b == b'0')
        })
        .unwrap()

}
fn part1(salt: Input) -> Output {
    solve(salt, 5)
}

fn part2(salt: Input) -> Output {
    solve(salt, 6)
}

#[test]
fn default() {
    let input = get_input(15, 4).unwrap();
    assert_eq!(117946, part1(&input));
    assert_eq!(3938038, part2(&input));
}

    // Input parsed (16μs)
    // 1. 117946 (17ms)
    // 2. 3938038 (542ms)
    // Total: 559ms