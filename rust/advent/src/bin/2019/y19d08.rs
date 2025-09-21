use advent::utilities::get_input::get_input;
use advent_ocr::ocr;
use itertools::Itertools;
use utilities::structs::grid::Grid2;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<Vec<Pixel>>;
type Output = usize;
const WIDTH: usize = 25;
const HEIGHT: usize = 6;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Pixel { Black, White, Transparent}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(19, 8).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let input = input.trim_end().as_bytes();
    input.iter()
        .map(|&b| {
            match b {
                b'0' => Pixel::Black,
                b'1' => Pixel::White,
                b'2' => Pixel::Transparent,
                _ => unreachable!()
            }
        })
        .chunks(WIDTH * HEIGHT)
        .into_iter()
        .map(|chunk| chunk.collect())
        .collect()
}

fn part1(layers: &Input) -> Output {
    layers.iter()
        .min_by_key(|layer| {
            layer.iter().filter(|&&pixel| pixel == Pixel::Black).count()
        })
        .map(|layer| {
            let whites = layer.iter().filter(|&&pixel| pixel == Pixel::White).count();
            let transparents= layer.iter().filter(|&&pixel| pixel == Pixel::Transparent).count();
            whites * transparents
        })
        .unwrap()
}

fn part2(layers: &Input) -> String {
    let screen = Grid2::new2d_with_fn(WIDTH, HEIGHT, |pos| {
        layers.iter()
            .map(|layer| layer[pos])
            .find(|&pixel| pixel != Pixel::Transparent)
            .map(|pixel| pixel == Pixel::White)
            .unwrap()
    });
    ocr(screen).unwrap()
}

#[test]
fn default() {
    let input = get_input(19, 8).unwrap();
    let input = parse_input(&input);
    assert_eq!(1088, part1(&input));
    assert_eq!("LGYHB".to_string(), part2(&input));
}

// Input parsed (94μs)
// 1. 1088 (10μs)
// 2. LGYHB (33μs)
// Total: 140μs