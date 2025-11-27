use std::collections::BinaryHeap;
use std::hash::Hash;

use everybody_codes::utilities::inputs::get_event_inputs;
use rustc_hash::FxHashSet;
use utilities::structs::grid::{Grid2, GridAdjacent, GridIterator};
use utilities::{
    enums::cardinals::Cardinal,
    structs::{
        coord::Coord2U,
        stopwatch::{ReportDuration, Stopwatch},
        str_grid::StrGrid,
    },
};

type Input<'a> = &'a str;
type Pos = Coord2U;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(25, 17);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

trait InRange {
    fn in_range(&self, other: &Self, radius: usize) -> bool;
}

impl InRange for Pos {
    fn in_range(&self, other: &Self, radius: usize) -> bool {
        let x = self.x().abs_diff(other.x());
        let y = self.y().abs_diff(other.y());
        let p = x * x + y * y;
        p <= radius * radius
    }
}

fn part1(notes: Input) -> usize {
    let volcano: StrGrid = notes.into();
    let center = volcano.s.iter().position(|&b| b == b'@').unwrap();
    let center = Pos::from_index(center, volcano.width).unwrap();
    volcano
        .s
        .iter()
        .enumerate()
        .filter(|&(idx, &b)| {
            b != b'@'
                && Pos::from_index(idx, volcano.width)
                    .unwrap()
                    .in_range(&center, 10)
        })
        .map(|(_, &b)| (b - b'0') as usize)
        .sum()
}

fn part2(notes: Input) -> usize {
    let volcano: StrGrid = notes.into();
    let center = volcano.s.iter().position(|&b| b == b'@').unwrap();
    let center = Pos::from_index(center, volcano.width).unwrap();
    let r2: Vec<_> = (0..center.x()).map(|it| it * it).collect();
    let mut destruction = vec![0usize; r2.len()];
    for (idx, &b) in volcano.s.iter().enumerate() {
        if !b.is_ascii_digit() {
            continue;
        }
        let pos = Pos::from_index(idx, volcano.width).unwrap();
        let x = center.x().abs_diff(pos.x());
        let y = center.y().abs_diff(pos.y());
        let p = x * x + y * y;
        let d = (b - b'0') as usize;
        let r = r2.binary_search(&p).unwrap_or_else(|r| r);
        if r < destruction.len() {
            destruction[r] += d;
        }
    }

    destruction
        .iter()
        .enumerate()
        .max_by_key(|&(_, &destruction)| destruction)
        .map(|(radius, &destruction)| radius * destruction)
        .unwrap()
}

fn part3(notes: Input) -> usize {
    let volcano: StrGrid = notes.into();
    let center = volcano.s.iter().position(|&b| b == b'@').unwrap();
    let center = Pos::from_index(center, volcano.width).unwrap();
    let start = volcano.s.iter().position(|&b| b == b'S').unwrap();
    let start = Pos::from_index(start, volcano.width).unwrap();
    let r2: Vec<_> = (0..center.x()).map(|it| it * it).collect();
    let volcano = notes
        .as_bytes()
        .iter()
        .enumerate()
        .filter(|&(_, &b)| b != b'\n')
        .map(|(idx, &b)| {
            let pos = Pos::from_index(idx, volcano.width).unwrap();
            let x = center.x().abs_diff(pos.x());
            let y = center.y().abs_diff(pos.y());
            let p = x * x + y * y;
            let seconds = if b == b'S' {
                0
            } else {
                (b - b'0') as usize
            };
            let r = r2.binary_search(&p).unwrap_or_else(|r| r);
            (r, seconds)
        })
        .try_collect_grid(volcano.width - 1)
        .unwrap();

    let mut min_seconds = 0;

    for radius in 10..(volcano.width() - 1) / 2 - 1 {
        if min_seconds <= 30 * (radius + 1) - 1 {
            match a_star(&volcano, start, center, radius) {
                Ok(seconds) => {
                    return seconds * radius
                },
                Err(seconds) => { min_seconds = seconds; },
            }
        }
    }
    unreachable!()
}

fn a_star(
    volcano: &Grid2<(usize, usize)>,
    start: Pos,
    center: Pos,
    radius: usize,
) -> Result<usize, usize> {
    let heuristic = |pos: Pos, phase: Cardinal| {
        let mut pos = pos;
        let mut h = 0;
        if phase == Cardinal::East {
            let target = center.move_direction(Cardinal::East, radius + 1).unwrap();
            h += pos.manhattan_distance(target);
            pos = target;
        }
        if phase == Cardinal::East || phase == Cardinal::South {
            let target = center.move_direction(Cardinal::South, radius + 1).unwrap();
            h += pos.manhattan_distance(target);
            pos = target;
        }
        if phase != Cardinal::North {
            let target = center.move_direction(Cardinal::West, radius + 1).unwrap();
            h += pos.manhattan_distance(target);
            pos = target;
        }
        h + pos.manhattan_distance(start)
    };

    let mut open: BinaryHeap<State> = BinaryHeap::new();
    let initial_state = State {
        pos: start,
        seconds: 0,
        phase: Cardinal::East,
        f: heuristic(start, Cardinal::East),
    };
    open.push(initial_state);
    let mut closed: FxHashSet<(Pos, Cardinal)> = FxHashSet::default();

    while let Some(State {
        pos,
        seconds,
        phase,
        f: _,
    }) = open.pop()
    {
        if !closed.insert((pos, phase)) {
            continue;
        }

        if pos == start && phase == Cardinal::North {
            return if seconds <= 30 * (radius + 1) - 1 {
                Ok(seconds)
            } else {
                Err(seconds)
            };
        }
        for GridAdjacent {
            index: _,
            pos: adj_pos,
            dir: _,
            value: &(adj_rad, adj_sec),
        } in volcano.adjacent(pos, false).unwrap()
        {
            // abort case 1: ventured into burn area
            if adj_rad <= radius {
                continue;
            }

            // ac2: no significant backtracking once quadrant checkpoint reached
            if match phase {
                Cardinal::North => adj_pos.y() > center.y() + 10,
                Cardinal::East => adj_pos.x() < center.x() - 10,
                Cardinal::South => adj_pos.y() < center.y() - 10,
                Cardinal::West => adj_pos.x() > center.x() + 10,
            } {
                continue;
            }

            let adj_phase = match phase {
                Cardinal::East => if adj_pos.y() == center.y() {
                    Cardinal::South
                } else {
                    phase
                },
                Cardinal::South => {
                    if adj_pos.x() == center.x() {
                        Cardinal::West
                    } else {
                        phase
                    }
                },
                Cardinal::West => {
                    if adj_pos.y() == center.y() {
                        Cardinal::North
                    } else {
                        phase
                    }
                },
                Cardinal::North => phase,
            };

            // ac3: already visited
            if closed.contains(&(adj_pos, adj_phase)) {
                continue;
            }
            let adj_sec = seconds + adj_sec;
            let adj_f = adj_sec + heuristic(adj_pos, adj_phase);

            let adj_state = State {
                pos: adj_pos,
                seconds: adj_sec,
                phase: adj_phase,
                f: adj_f,
            };
            open.push(adj_state);
        }
    }
    Err(usize::MAX)
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct State {
    pos: Pos,
    seconds: usize,
    phase: Cardinal,
    f: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f.cmp(&self.f).then(other.seconds.cmp(&self.seconds))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(25, 17);
    assert_eq!(1584, part1(&input1));
    assert_eq!(66183, part2(&input2));
    assert_eq!(42069, part3(&input3));
}

// Input parsed (40μs)
// 1. 1584 (25μs)
// 2. 66183 (74μs)
// 3. 42069 (33.728ms)
// Total: 33.872ms