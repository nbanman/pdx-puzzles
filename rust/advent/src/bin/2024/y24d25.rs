use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::structs::grid::Grid2;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 25).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: Input) -> Output {
    let (locks, keys): (Vec<&str>, Vec<&str>) = input
        .split("\n\n")
        .partition(|s| s.starts_with('#'));

    let locks = locks.into_iter()
        .map(|lock| {
            Grid2::try_from(lock).unwrap()
                .columns()
                .map(|pin| pin.iter().filter(|&&&it| it == '#').count())
                .collect_vec()
        })
        .collect_vec();

    let keys = keys.into_iter()
        .map(|key| {
            Grid2::try_from(key).unwrap()
                .columns()
                .map(|pin| pin.iter().filter(|&&&it| it == '#').count())
                .collect_vec()
        })
        .collect_vec();

    let mut fits = 0;
    for lock in locks {
        'key_loop: for key in keys.iter() {
            for _ in 0..5 {
                for (lock_pin, key_pin) in lock.iter().zip(key.iter()) {
                    if lock_pin + key_pin > 7 { continue 'key_loop; }
                }
            }
            fits += 1;
        }
    }
    fits
}

#[test]
fn default() {
    let input = get_input(24, 25).unwrap();
    assert_eq!(3287, part1(&input));
}

// Input parsed (26μs)
// 1. 3287 (491μs)
// Total: 520μs