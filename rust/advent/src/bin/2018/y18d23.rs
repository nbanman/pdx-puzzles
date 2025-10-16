use std::{cmp::Reverse, collections::BinaryHeap};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::{coord::Coord, stopwatch::{ReportDuration, Stopwatch}}};

type Input = Vec<Nanobot>;
type Output = usize;
type Pos = Coord<i64, 3>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Nanobot {
    pos: Pos,
    radius: usize,
}

impl Nanobot {
    fn in_range_of(&self, other: Pos) -> bool {
        self.radius >= self.pos.manhattan_distance(other)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cube<'a> {
    pos: Pos,
    length: i64,
    nanobots: Vec<&'a Nanobot>,
}

impl<'a> Ord for Cube<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.length.cmp(&other.length).then(self.pos.cmp(&other.pos))
    }
}

impl<'a> PartialOrd for Cube<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Cube<'a> {
    fn new(pos: Pos, length: i64, parent_bots: Vec<&'a Nanobot>) -> Self {
        let nanobots: Vec<&Nanobot> = parent_bots.iter()
            .filter(|&it| Self::in_range_of(it, pos, length))
            .copied()
            .collect();
        Self {
            pos,
            length,
            nanobots,
        }
    }

    fn cubify(self) -> [Cube<'a>; 8] {
        let length = self.length / 2;
        [
            Cube::new(self.pos, length, self.nanobots.clone()),
            Cube::new(
                Pos::new([self.pos.x() + length, self.pos.y(), self.pos.z()]),
                length,
                self.nanobots.clone()
            ),
            Cube::new( 
                Pos::new([self.pos.x(), self.pos.y() + length, self.pos.z()]),
                length,
                self.nanobots.clone()
            ),
            Cube::new( 
                Pos::new([self.pos.x() + length , self.pos.y() + length, self.pos.z()]),
                length,
                self.nanobots.clone()
            ),
            Cube::new( 
                Pos::new([self.pos.x(), self.pos.y(), self.pos.z() + length]),
                length,
                self.nanobots.clone()
            ),
            Cube::new( 
                Pos::new([self.pos.x() + length, self.pos.y(), self.pos.z() + length]),
                length,
                self.nanobots.clone()
            ),
            Cube::new( 
                Pos::new([self.pos.x(), self.pos.y() + length, self.pos.z() + length]),
                length,
                self.nanobots.clone()
            ),
            Cube::new( 
                Pos::new([self.pos.x() + length, self.pos.y() + length, self.pos.z() + length]),
                length,
                self.nanobots.clone()
            ),
        ]
    }

    fn in_range_of(nanobot: &Nanobot, pos: Pos, length: i64) -> bool {
        let cube_pos: Pos = Coord([
            match nanobot.pos.x() {
                x if x < pos.x() => pos.x(),
                x if x >= pos.x() + length => pos.x() + length - 1,
                _ => nanobot.pos.x()
            },
            match nanobot.pos.y() {
                y if y < pos.y() => pos.y(),
                y if y >= pos.y() + length => pos.y() + length - 1,
                _ => nanobot.pos.y()
            },
            match nanobot.pos.z() {
                z if z < pos.z() => pos.z(),
                z if z >= pos.z() + length => pos.z() + length - 1,
                _ => nanobot.pos.z()
            },
        ]);
        nanobot.in_range_of(cube_pos)
    }
}


fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(18, 23).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.get_numbers()
        .chunks(4)
        .into_iter()
        .map(|chunk| {
            let (x, y, z, radius) = chunk.collect_tuple().unwrap();
            Nanobot { pos: Pos::new3d(x, y, z), radius: radius as usize }
        })
        .collect()
}

fn part1(nanobots: &Input) -> Output {
    let strongest = nanobots.iter().max_by_key(|it| it.radius).unwrap();
    nanobots.iter()
        .filter(|&bot| strongest.in_range_of(bot.pos))
        .count()
}

fn part2(nanobots: &Input) -> Output {
    let (min, max) = Coord::min_max(nanobots.iter().map(|bot| &bot.pos));
    let (min, max) = min.0.into_iter().zip(max.0.into_iter()).max_by_key(|(min, max)| max - min).unwrap();
    let length = 2i64.pow((((max - min) as f32).ln() / 2f32.ln()).ceil() as u32);
    let initial_cube = Cube::new(Pos::origin(), length, nanobots.iter().collect_vec());
    let mut cubes = BinaryHeap::new();
    cubes.push(Reverse(initial_cube));   
    let mut current = Cube { pos: Pos::origin(), length: 1, nanobots: Vec::new() };
    while let Some(Reverse(next)) = cubes.pop() {
        if current.nanobots.len() >= next.nanobots.len() { continue; }
        let next = next.cubify();
        for cube in next {
            if cube.length == 1 && cube.nanobots.len() > current.nanobots.len() {
                current = cube.clone();
            }
            cubes.push(Reverse(cube));
        }
    }
    current.pos.manhattan_distance(Pos::origin())
}

#[test]
fn default() {
    let input = get_input(18, 23).unwrap();
    let input = parse_input(&input);
    assert_eq!(481, part1(&input));
    assert_eq!(47141479, part2(&input));
}

// Input parsed (152μs)
// 1. 481 (6μs)
// 2. 47141479 (557ms)
// Total: 557ms
