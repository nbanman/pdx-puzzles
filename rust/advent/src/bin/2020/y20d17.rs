use std::collections::HashSet;
use std::ops::Range;
use advent::utilities::get_input::get_input;
use utilities::structs::coord::Coord;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};
use utilities::structs::str_grid::StrGrid;

type Input<'a> = StrGrid<'a>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(20, 17).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &'_ str) -> Input<'_> {
    StrGrid::new(input).unwrap()
}

fn get_cubes<const DIMENSIONS: usize>(start: &StrGrid) -> Output {
    let mut space: HashSet<Coord<i64, DIMENSIONS>> = start.s.iter().enumerate()
        .filter(|&(_, &b)| b != b'.' && b != b'\n')
        .map(|(idx, _)| {
            let pos = start.idx_to_coord(&idx);
            let coordinates: [i64; DIMENSIONS] = std::array::from_fn(|dimension| {
                match dimension {
                    0 => pos.x() as i64,
                    1 => pos.y() as i64,
                    _ => 0,
                }
            });
            Coord::new(coordinates)
        })
        .collect();

    let mut bounds: [Range<i64>; DIMENSIONS] = std::array::from_fn(|dimension| {
        match dimension {
            0 => 0..start.width as i64 - 1,
            1 => 0..start.height as i64,
            _ => 0..1,
        }
    });

    for _ in 0..6 {
        let mut new_space: HashSet<Coord<i64, DIMENSIONS>> = HashSet::new();

        // expand bounds by one in each direction in each dimension
        for dimension in 0..DIMENSIONS {
            let bound = &bounds[dimension];
            bounds[dimension] = Range {start: bound.start - 1, end: bound.end + 1 };
        }

        let mut bounds_iter = bounds.iter();
        let init: Vec<Vec<i64>> = bounds_iter
            .next()
            .map(|rng| rng.clone().map(|it| vec![it]).collect())
            .unwrap();
        let coordinates = bounds_iter.fold(init, |acc, rng| {
            acc.into_iter()
                .flat_map(|int_list| {
                    rng.clone().map(move |new| {
                        int_list.iter().cloned().chain(std::iter::once(new)).collect()
                    })
                })
                .collect()
        });
        for coordinate in coordinates {
            let pos: Coord<i64, DIMENSIONS> = Coord::new(coordinate.try_into().unwrap());
            let neighbors = pos.get_neighbors()
                .filter(|neighbor| space.contains(neighbor))
                .count();
            if neighbors == 3 || (neighbors == 2 && space.contains(&pos)) {
                new_space.insert(pos);
            }
        }
        space = new_space;
    }
    space.len()
}

fn part1(start: &Input) -> Output {
    get_cubes::<3>(start)
}

fn part2(start: &Input) -> Output {
    get_cubes::<4>(start)
}

#[test]
fn default() {
    let input = get_input(20, 17).unwrap();
    let input = parse_input(&input);
    assert_eq!(346, part1(&input));
    assert_eq!(1632, part2(&input));
}

// Input parsed (30μs)
// 1. 346 (11ms)
// 2. 1632 (356ms)
// Total: 367ms