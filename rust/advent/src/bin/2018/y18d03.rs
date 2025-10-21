use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::{coord::Coord2U, stopwatch::{ReportDuration, Stopwatch}}};

type Input = (Vec<usize>, Vec<(Pos, Pos)>, usize);
type Output = usize;
type Pos = Coord2U;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(18, 3).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let claims: Vec<(Pos, Pos)> = input.get_numbers()
        .tuples()
        .map(|(_, x, y, w, h)| {
            (Pos::new2d(x, y), Pos::new2d(x + w - 1, y + h - 1))
        })
        .collect();
    let width = claims.iter().map(|(_, br)| br.x()).max().unwrap() + 1;
    let height = claims.iter().map(|(_, br)| br.y()).max().unwrap() + 1;
    
    let mut skein = vec![0; width * height];
    for (tl, br) in claims.iter() {
        for y in tl.y()..=br.y() {
            for x in tl.x()..=br.x() {
                skein[y * width + x] += 1;
            }
        }
    }
    
    (skein, claims, width)
}

fn part1(input: &Input) -> Output {
    let (skein, _, _) = input;
    skein.iter().filter(|&&it| it > 1).count()
}

fn part2(input: &Input) -> Output {
    let (skein, claims, width) = input;
    1 + claims.iter()
        .position(|(tl, br)| {
            (tl.y()..=br.y()).cartesian_product(tl.x()..=br.x()).all (|(y, x)| {
                skein[y * width + x] == 1
            })
        })
        .unwrap()
}

#[test]
fn default() {
    let input = get_input(18, 3).unwrap();
    let input = parse_input(&input);
    assert_eq!(110891, part1(&input));
    assert_eq!(297, part2(&input));
}

// Input parsed (1ms)
// 1. 110891 (278μs)
// 2. 297 (28μs)
// Total: 2ms
