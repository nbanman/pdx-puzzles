use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rayon::slice::ParallelSliceMut;
use rustc_hash::FxHashMap;
use utilities::minmax::minmax;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::{
        coord::Coord2U,
        stopwatch::{ReportDuration, Stopwatch},
    },
};

type Input = Vec<Pos>;
type Output = usize;
type Pos = Coord2U;

fn main() {
    let input = get_input(25, 9).unwrap();
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input
        .get_numbers()
        .tuples::<(_, _)>()
        .map(Pos::from)
        .collect()
}

fn rect_area(a: Pos, b: Pos) -> usize {
    (a.x().abs_diff(b.x()) + 1) * (a.y().abs_diff(b.y()) + 1)
}

fn part1(red_tiles: &Input) -> Output {
    red_tiles
        .iter()
        .tuple_combinations()
        .map(|(a, b)| rect_area(*a, *b))
        .max()
        .unwrap()
}

fn part2(red_tiles: &Input) -> Output {
    let (xmin, xmax) = red_tiles
        .iter()
        .map(|pos| pos.x())
        .minmax()
        .into_option()
        .unwrap();
    let (ymin, ymax) = red_tiles
        .iter()
        .map(|pos| pos.y())
        .minmax()
        .into_option()
        .unwrap();

    let mut green_x: FxHashMap<usize, Vec<usize>> = FxHashMap::default();
    let mut green_y = green_x.clone();

    for (&prev, &next) in red_tiles.iter().circular_tuple_windows() {
        let (&xmin, &xmax) = minmax(&prev.x(), &next.x());
        for x in xmin + 1..xmax {
            green_x.entry(x).or_insert(Vec::new()).push(prev.y());
        }
        let (&ymin, &ymax) = minmax(&prev.y(), &next.y());
        for y in ymin + 1..ymax {
            green_y.entry(y).or_insert(Vec::new()).push(prev.x());
        }
    }

    green_x.retain(|_, v| {
        if v.len() < 3 {
            false
        } else {
            v.sort();
            v.pop();
            v.remove(0);
            true
        }
    });
    green_y.retain(|_, v| {
        if v.len() < 3 {
            false
        } else {
            v.sort();
            v.pop();
            v.remove(0);
            true
        }
    });

    red_tiles
        .iter()
        .tuple_combinations()
        .filter(|&(&a, &b)| {
            let (&xmin, &xmax) = minmax(&a.x(), &b.x());
            let (&ymin, &ymax) = minmax(&a.y(), &b.y());
            green_x
                .get(&xmin)
                .map(|it| it.iter().all(|&y| y <= ymin || y >= ymax))
                .unwrap_or(true)
                && green_x
                    .get(&xmax)
                    .map(|it| it.iter().all(|&y| y <= ymin || y >= ymax))
                    .unwrap_or(true)
                && green_x
                    .get(&ymin)
                    .map(|it| it.iter().all(|&x| x <= xmin || x >= ymax))
                    .unwrap_or(true)
                && green_x
                    .get(&xmax)
                    .map(|it| it.iter().all(|&x| x <= xmin || x >= ymax))
                    .unwrap_or(true)
        })
        .map(|(a, b)| rect_area(*a, *b))
        .max()
        .unwrap()
}

#[test]
fn default() {
    let input = get_input(25, 9).unwrap();
    let input = parse_input(&input);
    assert_eq!(4759420470, part1(&input));
    // assert_eq!(YY, part2(&input));
}

// 2916129645 too high
