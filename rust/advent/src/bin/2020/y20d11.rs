use std::iter::successors;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::structs::{
    coord::Coord,
    grid::{Grid, Grid2, GridIterator},
    stopwatch::{ReportDuration, Stopwatch},
};

type Input = Grid2<Seat>;
type Output = usize;
type Pos = Coord<i32, 2>;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(20, 11).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Seat {
    Occupied,
    Unoccupied,
    EmptySpace,
}
impl Seat {
    fn is_occupied(&self) -> bool {
        matches!(&self, Self::Occupied)
    }
    fn is_empty_place(&self) -> bool {
        matches!(&self, Self::EmptySpace)
    }
}

impl From<char> for Seat {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Occupied,
            'L' => Self::Unoccupied,
            '.' => Self::EmptySpace,
            c => {
                panic!("{c} is not a recognized character.");
            }
        }
    }
}

fn parse_input(input: &str) -> Input {
    Grid::new2d_map_str(input, Seat::from).unwrap()
}

fn solve<F>(layout: &Input, tolerance: Output, get_neighbors: F) -> Output
where
    F: Fn(&Input, Output) -> Output,
{
    // Create a sequence that starts with the original layout and provides successive seat layouts, one per turn
    // The sequence uses the part-specific tolerance parameter and getNeighbor function to generate new states.
    let width = layout.width();
    let new_states = successors(Some(layout.clone()), |layout| {
        let next_layout = layout
            .iter()
            .enumerate()
            .map(|(index, &seat)| {
                if seat.is_empty_place() {
                    Seat::EmptySpace
                } else {
                    let is_occupied = seat.is_occupied();
                    let neighbors = get_neighbors(&layout, index);
                    if (is_occupied && neighbors < tolerance) || (!is_occupied && neighbors == 0) {
                        Seat::Occupied
                    } else {
                        Seat::Unoccupied
                    }
                }
            })
            .try_collect_grid(width)
            .unwrap();
        Some(next_layout)
    });
    new_states
        .tuple_windows()
        .find(|(a, b)| a == b)
        .unwrap()
        .0
        .into_iter()
        .filter(Seat::is_occupied)
        .count()
}

fn part1(layout: &Input) -> Output {
    let get_neighbors = |layout: &Grid2<Seat>, index: Output| {
        layout
            .adjacent(index, true)
            .unwrap()
            .filter(|adj| adj.value.is_occupied())
            .count()
    };
    solve(layout, 4, get_neighbors)
}

fn part2(layout: &Input) -> Output {
    let width = layout.width();
    let get_neighbors = |layout: &Grid2<Seat>, index: Output| {
        let pos = Pos::new2d((index % width) as i32, (index / width) as i32);
        vec![
            Pos::new2d(-1, -1),
            Pos::new2d(0, -1),
            Pos::new2d(1, -1),
            Pos::new2d(-1, 0),
            Pos::new2d(1, 0),
            Pos::new2d(-1, 1),
            Pos::new2d(0, 1),
            Pos::new2d(1, 1),
        ]
        .into_iter()
        .filter(|&slope| {
            let mut new_pos = pos + slope;
            let mut keep = false;
            while let Some(seat) = layout.get(new_pos) {
                match seat {
                    Seat::Occupied => {
                        keep = true;
                        break;
                    }
                    Seat::Unoccupied => {
                        break;
                    }
                    Seat::EmptySpace => {
                        new_pos += slope;
                    }
                }
            }
            keep
        })
        .count()
    };
    solve(layout, 5, get_neighbors)
}

#[test]
fn default() {
    let input = get_input(20, 11).unwrap();
    let input = parse_input(&input);
    assert_eq!(2243, part1(&input));
    assert_eq!(2027, part2(&input));
}

// Input parsed (71μs)
// 1. 2243 (85ms)
// 2. 2027 (39ms)
// Total: 124ms
