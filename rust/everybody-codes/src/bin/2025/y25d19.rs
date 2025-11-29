use std::ops::Range;

use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::{
        coord::Coord2U,
        stopwatch::{ReportDuration, Stopwatch},
    },
};

type Input<'a> = &'a str;
type Pos = Coord2U;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(25, 19);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, Clone)]
struct Wall {
    x: usize,
    gaps: Vec<Range<usize>>,
}

trait Flappy {
    fn altitude(self, wall: &Wall) -> impl Iterator<Item = usize>;
}

impl Flappy for Pos {
    fn altitude(self, wall: &Wall) -> impl Iterator<Item = usize> {
        let dist = self.x() - wall.x;
        let min = self.y().checked_sub(dist).unwrap_or_default();
        let even_or_odd = (self.y() & 1) ^ (dist & 1);
        (min..=self.y() + dist)
            .filter(move |&y| (y & 1) == even_or_odd)
            .filter(move |y| wall.gaps.iter().any(|it| it.contains(y)))
    }
}

fn build_walls(notes: Input) -> Vec<Wall> {
    let mut walls = vec![Wall { x: 0, gaps: vec![0..1] }];
    walls.extend(
        notes
            .get_numbers::<usize>()
            .tuples::<(_, _, _)>()
            .chunk_by(|(x, _, _)| *x)
            .into_iter()
            .map(|chunk| {
                let x = chunk.0;
                let gaps = chunk.1.map(|(_, lo, size)| lo..lo + size).collect();
                Wall { x, gaps }
            })
    );

    walls
}

fn min_flaps(notes: Input) -> usize {
    let mut walls = build_walls(notes);
    let start = walls.pop().unwrap();
    for gap in start.gaps {
        'outer: for y in gap {
            let mut birds = vec![Pos::from((start.x, y))];
            for wall in walls.iter().rev() {
                birds = birds
                    .into_iter()
                    .flat_map(|bird| bird.altitude(wall))
                    .unique()
                    .map(|y| Pos::from((wall.x, y)))
                    .collect();
                if birds.is_empty() {
                    continue 'outer;
                }
            }
            return (start.x + y) / 2;
        }
    }
    unreachable!()
}

fn part1(notes: Input) -> usize {
    min_flaps(notes)
}

fn part2(notes: Input) -> usize {
    min_flaps(notes)
}

fn part3(notes: Input) -> usize {
    min_flaps(notes)
}

#[test]
fn test1() {
    let notes = r"7,7,2
12,0,4
15,5,3
24,1,6
28,5,5
40,8,2";
    assert_eq!(24, part1(notes))
}

#[test]
fn test2() {
    let notes = r"7,7,2
7,1,3
12,0,4
15,5,3
24,1,6
28,5,5
40,3,3
40,8,2";
    assert_eq!(22, part2(notes));
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(25, 19);
    assert_eq!(51, part1(&input1));
    // assert_eq!(ZZ, part2(&input2));
    // assert_eq!(ZZ, part3(&input3));
}
