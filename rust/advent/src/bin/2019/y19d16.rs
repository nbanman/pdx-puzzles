use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<i8>;
type Output = i32;
const PHASES: usize = 100;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(19, 16).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.as_bytes().trim_ascii_end().iter().map(|&b| (b - 48) as i8).collect()
}

fn part1(numbers: &Input) -> Output {
    let start_pattern = [0, 1, 0, -1];
    let mut fft = numbers.clone();
    for _ in 0..PHASES {
        fft = (0..fft.len())
            .map(|index| {
                let calc = fft.iter().enumerate().fold(0, |acc, (index2, &i)| {
                    let ai = ((index2 + 1) / (index + 1)) % 4;
                    acc + i as i32 * start_pattern[ai]
                });
                (calc % 10).abs() as i8
            })
            .collect();
    }
    fft.into_iter()
        .take(8)
        .fold(0, |acc, i| acc * 10 + i as Output)
}

fn part2(numbers: &Input) -> Output {
    let offset = numbers[..7].iter().fold(0, |acc, &i| acc * 10 + i as usize);
    let array_len = numbers.len() * 10_000 - offset;
    let offset_mod = offset % numbers.len();
    let mut numbers_i: Vec<i8> = (0..array_len)
        .map(|i| numbers[(offset_mod + i) % numbers.len()])   
        .collect();
    for _ in 0..PHASES {
        for i in (0..numbers_i.len() - 1).rev() {
            numbers_i[i] = (numbers_i[i] + numbers_i[i + 1]) % 10;
        }
    }
    numbers_i.into_iter()
        .take(8)
        .fold(0, |acc, i| acc * 10 + i as Output)
}

#[test]
fn default() {
    let input = get_input(19, 16).unwrap();
    let input = parse_input(&input);
    assert_eq!(52611030, part1(&input));
    assert_eq!(52541026, part2(&input));
}

