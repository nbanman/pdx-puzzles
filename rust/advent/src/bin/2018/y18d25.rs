use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::{
        coord::Coord,
        stopwatch::{ReportDuration, Stopwatch},
    },
};

type Input<'a> = &'a str;
type Output = usize;
type Pos = Coord<i64, 4>;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(18, 25).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: Input) -> Output {
    let mut constellations: FxHashSet<Vec<Pos>> = FxHashSet::default();

    let points: Vec<Pos> = input
        .get_numbers()
        .chunks(4)
        .into_iter()
        .map(|chunk| chunk.into())
        .collect();

    for point in points {
        let in_range: Vec<Vec<Pos>> = constellations
            .iter()
            .filter(|constellation| {
                constellation
                    .iter()
                    .any(|it| point.manhattan_distance(it) <= 3)
            })
            .cloned()
            .collect();
        if in_range.is_empty() {
            constellations.insert(vec![point]);
        } else {
            let new_constellation: Vec<Pos> = std::iter::once(point)
                .chain(in_range.iter().flat_map(|it| it.iter().copied()))
                .collect();
            for it in in_range {
                constellations.remove(&it);
            }
            constellations.insert(new_constellation);
        }
    }
    constellations.len()
}

#[test]
fn default() {
    let input = get_input(18, 25).unwrap();
    assert_eq!(394, part1(&input));
}

// Input parsed (18μs)
// 1. 394 (2ms)
// Total: 2ms
