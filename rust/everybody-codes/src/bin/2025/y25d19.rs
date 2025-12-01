use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::stopwatch::{ReportDuration, Stopwatch},
};

type Input<'a> = &'a str;
type Bird = RangeInclusive<usize>;

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
    gaps: Vec<RangeInclusive<usize>>,
}

trait Flappy {
    fn altitude(self, x: usize, wall: &Wall) -> impl Iterator<Item = Self>;
}

impl Flappy for Bird {
    fn altitude(self, x: usize, wall: &Wall) -> impl Iterator<Item = Self> {
        let dist = wall.x - x;
        let min_alt = self.start().checked_sub(dist).unwrap_or_default();
        let max_alt = self.end() + dist;
        wall.gaps.iter().filter_map(move |gap| {
            let gap_min = max(min_alt, *gap.start());
            let gap_max = min(max_alt, *gap.end());
            if gap_min <= gap_max {
                Some(gap_min..=gap_max)
            } else {
                None
            }
        })
    }
}

fn build_walls(notes: Input) -> Vec<Wall> {
    notes
        .get_numbers::<usize>()
        .tuples::<(_, _, _)>()
        .chunk_by(|(x, _, _)| *x)
        .into_iter()
        .map(|chunk| {
            let x = chunk.0;
            let gaps = chunk
                .1
                .map(|(_, lo, size)| {
                    // note here that I'm narrowing the wall gap sizes to correspond to where the bird's altitude
                    // can be at the time of passing the wall. Walls whose x is even must have even tops and bottoms
                    // of gaps. Walls whose x is odd must have odd tops and bottoms of gaps.
                    let mut lo = lo;
                    let mut hi = lo + size - 1;
                    if lo & 1 != x & 1 {
                        lo += 1;
                    }
                    if hi & 1 != x & 1 {
                        hi -= 1;
                    }
                    lo..=hi
                })
                .sorted_unstable_by_key(|rng| *rng.start())
                .collect();
            Wall { x, gaps }
        })
        .collect()
}

fn min_flaps(notes: Input) -> usize {
    let walls = build_walls(notes);
    let mut birds = vec![0..=0];
    let mut x = 0;
    for wall in walls.iter() {
        birds = birds
            .into_iter()
            .flat_map(|bird| bird.altitude(x, wall))
            .collect();

        // combine any birds that have overlapping ranges
        for i in (0..birds.len() - 1).rev() {
            if birds[i].end() >= birds[i + 1].start() {
                birds[i] = *birds[i].start()..=*birds[i + 1].end();
                birds.remove(i + 1);
            }
        }
        x = wall.x;
    }

    (x + birds[0].start()) / 2
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
    assert_eq!(784, part2(&input2));
    assert_eq!(4542717, part3(&input3));
}

// Input parsed (34μs)
// 1. 51 (9μs)
// 2. 784 (15μs)
// 3. 4542717 (31μs)
// Total: 92μs
