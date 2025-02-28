use std::usize;

use advent::utilities::get_input::get_input;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use utilities::{
    enums::cardinals::Cardinal,
    structs::{
        coord::{Coord2, Coord2U},
        stopwatch::{ReportDuration, Stopwatch},
        str_grid::StrGrid,
    },
};

type Input<'a> = StrGrid<'a>;
type Output = usize;
type Pos = Coord2;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(23, 16).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Clone, Debug)]
struct State {
    pos: Pos,
    dir: Cardinal,
}

impl State {
    fn to_index(&self, grid: &Input) -> usize {
        ((self.pos.y() as usize * grid.width + self.pos.x() as usize) << 2) + self.dir.ordinal()
    }

    fn next(&self, grid: &Input) -> Option<impl Iterator<Item = Self>> {
        let moved = self.pos.move_direction(self.dir, 1).unwrap();

        let x: usize = moved.x().try_into().ok()?;
        let y: usize = moved.y().try_into().ok()?;

        let test = grid.try_get_coord(Coord2U::new2d(x, y))?;

        let next = match test {
            b'.' => vec![self.dir],
            b'|' => match self.dir {
                Cardinal::North | Cardinal::South => vec![self.dir],
                Cardinal::East | Cardinal::West => vec![Cardinal::North, Cardinal::South],
            },
            b'-' => match self.dir {
                Cardinal::North | Cardinal::South => vec![Cardinal::East, Cardinal::West],
                Cardinal::East | Cardinal::West => vec![self.dir],
            },
            b'/' => {
                if self.dir.ordinal() & 1 == 0 {
                    vec![self.dir.right()]
                } else {
                    vec![self.dir.left()]
                }
            }
            b'\\' => {
                if self.dir.ordinal() & 1 == 1 {
                    vec![self.dir.right()]
                } else {
                    vec![self.dir.left()]
                }
            }
            c => panic!("{} not recognized as space or mirror", c as char),
        }
        .into_iter()
        .map(move |dir| Self { pos: moved, dir });
        Some(next)
    }
}

fn light_beam(state: State, grid: &Input) -> usize {
    let mut visited = vec![false; grid.s.len() * 4];
    let mut q = Vec::new();
    q.push(state);

    while let Some(current) = q.pop() {
        if let Some(next) = current.next(grid) {
            next.filter(|next_state| {
                let index = next_state.to_index(grid);
                let is_visited = visited[index];
                visited[index] = true;
                !is_visited
            })
            .for_each(|next_state| q.push(next_state));
        }
    }

    let mut count = 0;
    for index in 0..grid.s.len() {
        for dir in 0..4 {
            if visited[(index << 2) + dir] {
                count += 1;
                break;
            }
        }
    }
    count
}

fn parse_input(input: &str) -> Input {
    StrGrid::new(input).unwrap()
}

fn part1(grid: &Input) -> Output {
    let start = State {
        pos: Pos::new2d(-1, 0),
        dir: Cardinal::East,
    };
    light_beam(start, grid)
}

fn part2(grid: &Input) -> Output {
    let mut states = Vec::new();
    states.extend((0..grid.width - 1).map(|x| State {
        pos: Pos::new2d(x as i64, -1),
        dir: Cardinal::South,
    }));
    states.extend((0..grid.width - 1).map(|x| State {
        pos: Pos::new2d(x as i64, grid.height as i64),
        dir: Cardinal::North,
    }));
    states.extend((0..grid.height).map(|y| State {
        pos: Pos::new2d(-1, y as i64),
        dir: Cardinal::East,
    }));
    states.extend((0..grid.height).map(|y| State {
        pos: Pos::new2d((grid.width - 1) as i64, y as i64),
        dir: Cardinal::West,
    }));

    states
        .par_iter()
        .map(|v| light_beam(v.clone(), grid))
        .max()
        .unwrap()
}

#[test]
fn default() {
    let input = get_input(23, 16).unwrap();
    let input = parse_input(&input);
    assert_eq!(7798, part1(&input));
    assert_eq!(8026, part2(&input));
}

// Input parsed (25μs)
// 1. 7798 (272μs)
// 2. 8026 (8ms)
// Total: 8ms
