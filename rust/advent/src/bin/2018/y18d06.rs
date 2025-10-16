use std::cmp::max;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::{coord::{Coord2, Coord2U}, grid::Grid2, stopwatch::{ReportDuration, Stopwatch}}};

type Input = (Vec<Pos>, usize, usize);
type Output = usize;
type Pos = Coord2;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(18, 6).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    // "raw" coordinates before offset parsed from input
    let offset_pos: Vec<Pos> = input.get_numbers()
        .chunks(2)
        .into_iter()
        .map(|mut chunk| Pos::new2d(chunk.next().unwrap(), chunk.next().unwrap()))
        .collect();

    let mut x_min = i64::MAX;
    let mut y_min = i64::MAX;
    let mut x_max= i64::MIN;
    let mut y_max= i64::MIN;

    for &pos in offset_pos.iter() {
        let x = pos.x();
        let y = pos.y();
        if x < x_min {
            x_min = x;
        } else if x > x_max {
            x_max = x;
        }
        if y < y_min {
            y_min = y;
        } else if y > y_max {
            y_max = y;
        }
    }

    let offset = Pos::new2d(x_min, y_min);
    
    let positions = offset_pos.into_iter()
        .map(|pos| pos - offset)
        .collect();

    let width = (x_max - x_min + 1) as usize;
    let height = (y_max - y_min + 1) as usize;
    
    (positions, width, height)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Space {
    Unexplored,
    Equidistant,
    Mapped(usize),
}

fn part1(input: &Input) -> Output {
    let (positions, width, height) = input;
    let mut space = Grid2::new2d(vec![Space::Unexplored; width * height], *width).unwrap();
    for (index, &pos) in positions.iter().enumerate() {
        let pos = Coord2U::try_from(pos).unwrap();
        space[pos] = Space::Mapped(index);
    }

    let sides = [
        (Pos::new2d(0, -1), Pos::new2d(1, 1)),
        (Pos::new2d(1, 0), Pos::new2d(-1, 1)),
        (Pos::new2d(0, 1), Pos::new2d(-1, -1)),
        (Pos::new2d(-1, 0), Pos::new2d(1, -1)),
    ];

    let ring = |pos: Pos, t: i64| {
        sides.iter()
            .copied()
            .flat_map(move |(initial, dir)| {
                let initial = pos + Pos::new2d(initial.x() * t, initial.y() * t);
                let it = (2..=t).scan(initial, move |acc, _| {
                    *acc = *acc + dir;
                    Some(*acc)
                });
                std::iter::once(initial).chain(it)
            })
    };

    // for each distance unit assign unassigned spaces to the closest coordinate 
    for distance in 1..max(*width, *height) {
        // track any assignments because if another coordinate also reaches the space at the same distance then
        // the assignment must be undone
        let mut distance_log = FxHashSet::default();

        for (index, &pos) in positions.iter().enumerate() {
            for ring_pos in ring(pos, distance as i64) {
                let ring_pos = Coord2U::try_from(ring_pos)
                    .unwrap_or(Coord2U::new2d(99_999, 99_999));
                match space.get(ring_pos) {
                    None | Some(Space::Equidistant) => {},
                    Some(Space::Unexplored) => {
                        space[ring_pos] = Space::Mapped(index);
                        distance_log.insert(ring_pos);
                    },
                    _ => {
                        if distance_log.contains(&ring_pos) {
                            space[ring_pos] = Space::Equidistant;
                        }
                    },
                }
            }
        }
        if distance_log.is_empty() {
            break;
        }
    }

    let infinite: FxHashSet<_> = 
        space.row(0).unwrap()
            .chain(space.row(*height - 1).unwrap())
            .chain(space.column(0).unwrap())
            .chain(space.column(*width - 1).unwrap())
            .collect();

    space.iter()
        .filter(|&it| !infinite.contains(it))
        .counts()
        .iter()
        .map(|(_, &count)| count)
        .max()
        .unwrap()   
}

fn part2(input: &Input) -> Output {
    let (positions, width, height) = input;
    (0..width * height)
        .filter(|&idx| {
            let sum: usize = positions.iter()
                .map(|pos| {
                    pos.manhattan_distance(Pos::from_index(idx, *width).unwrap())
                })
                .sum();
            sum < 10_000
        })
        .count()
}

#[test]
fn default() {
    let input = get_input(18, 6).unwrap();
    let input = parse_input(&input);
    assert_eq!(5365, part1(&input));
    assert_eq!(42513, part2(&input));
}

// Input parsed (19μs)
// 1. 5365 (13ms)
// 2. 42513 (4ms)
// Total: 18ms
