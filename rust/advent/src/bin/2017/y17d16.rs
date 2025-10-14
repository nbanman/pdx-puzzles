use advent::utilities::get_input::get_input;
use indexmap::IndexSet;
use itertools::Itertools;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = ([u8; 16], Vec<DanceMove>);
type Output = String;

#[derive(Debug, Clone, Copy)]
enum DanceMove {
    Spin(i32),
    Exchange(i32, i32),
    Partner(u8, u8),
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(17, 16).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let positions: [u8; 16] = "abcdefghijklmnop".as_bytes().into_iter().copied()
        .collect_vec()
        .try_into()
        .unwrap();

    let dance_moves = input.split(',')
        .map(|it| {
            let (dance_code, args) = it.split_at(1);
            match dance_code {
                "s" => DanceMove::Spin(args.parse().unwrap()),
                "x" => {
                    let (a, b) = args.split('/').map(|it| it.parse().unwrap()).collect_tuple().unwrap();
                    DanceMove::Exchange(a, b)
                },
                "p" => {
                    let (a, b) = args.split('/')
                        .map(|it| it.chars().next().unwrap() as u8)
                        .collect_tuple()
                        .unwrap();
                    DanceMove::Partner(a, b)
                },
                _ => unreachable!(),
            }
        })
        .collect();

    (positions, dance_moves)
}

fn dance_party(positions: &mut [u8], dance_moves: &[DanceMove]) {
    // rather than reorder entire array after each spin, just keep track of the offset and reverse it at the end
    let mut offset = 0;
    let len = positions.len() as i32;

    // execute each instruction
    for &dance_move in dance_moves {
        match dance_move {
            DanceMove::Spin(spin) => {
                offset = (spin + offset) % len as i32;
            },
            DanceMove::Exchange(a, b) => {
                let a = (a - offset).rem_euclid(len) as usize;
                let b = (b - offset).rem_euclid(len) as usize;
                positions.swap(a, b);
            },
            DanceMove::Partner(a, b) => {
                let a = positions.iter().position(|&it| it == a).unwrap();
                let b = positions.iter().position(|&it| it == b).unwrap();
                positions.swap(a, b);
            },
        }
    }

    // rotate to accommodate the spin
    positions.rotate_right(offset as usize);
}

fn part1(input: &Input) -> Output {
    let (positions, dance_moves) = input;
    let mut positions = *positions;
    dance_party(&mut positions, dance_moves);
    positions.into_iter().map(|b| b as char).collect()
}

fn part2(input: &Input) -> Output {
    let (positions, dance_moves) = input;
    let mut positions = *positions;
    let mut cache: IndexSet<[u8; 16]> = IndexSet::new();
    loop {
        if !cache.insert(positions.clone()) {
            let finale = cache.get_index(1_000_000_000 % cache.len()).unwrap();
            return finale.iter().map(|&b| b as char).collect();
        }
        dance_party(&mut positions, dance_moves);
    }
}

#[test]
fn default() {
    let input = get_input(17, 16).unwrap();
    let input = parse_input(&input);
    assert_eq!("hmefajngplkidocb".to_string(), part1(&input));
    assert_eq!("fbidepghmjklcnoa".to_string(), part2(&input));
}

// Input parsed (333μs)
// 1. hmefajngplkidocb (56μs)
// 2. fbidepghmjklcnoa (2ms)
// Total: 2ms
