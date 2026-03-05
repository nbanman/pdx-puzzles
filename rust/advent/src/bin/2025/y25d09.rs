use advent::utilities::get_input::get_input;
use indexmap::IndexSet;
use itertools::Itertools;
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
    let x_map: IndexSet<usize> = red_tiles.iter()
        .map(|tile| tile.x())
        .sorted_unstable()
        .collect();
    let y_map: IndexSet<usize> = red_tiles.iter()
        .map(|tile| tile.y())
        .sorted_unstable()
        .collect();
    let (x_lo, x_hi) = red_tiles
        .iter()
        .map(|pos| pos.x())
        .minmax()
        .into_option()
        .unwrap();
    let (y_lo, y_hi) = red_tiles
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

    let possible = red_tiles
        .iter()
        .filter(|&pos| {
            pos.x() != x_lo && pos.x() != x_hi && pos.y() != y_lo && pos.y() != y_hi
        })
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
                && green_y
                    .get(&ymin)
                    .map(|it| it.iter().all(|&x| x <= xmin || x >= xmax))
                    .unwrap_or(true)
                && green_y
                    .get(&ymax)
                    .map(|it| it.iter().all(|&x| x <= xmin || x >= xmax))
                    .unwrap_or(true)
        })
        .collect_vec();

    possible.into_iter()
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

// 2877792786 too high
// 2445498515
// 2080559052
// 1603439684 (supposed answer)
