use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::{coord::Coord, stopwatch::{ReportDuration, Stopwatch}}};

type Solution = (usize, usize);
type Output = usize;
type Pos = Coord<i64, 3>;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(21, 19).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    let solution = solve(&input);
    println!("1. {} ({})", part1(&solution), stopwatch.lap().report());
    println!("2. {} ({})", part2(&solution), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Scanner {
    beacons: Vec<Pos>,
    locations: Vec<Pos>,
    morphisms: Vec<[Pos; 24]>,
    mighty_morphisms: [Vec<Pos>; 24],
    pos_pairs: FxHashMap<(i64, i64, i64), (usize, usize)>
}

impl Scanner {
    fn new(beacons: Vec<Pos>, locations: Vec<Pos>) -> Self {
        let morphisms = beacons.iter()
            .map(|beacon| {
                let b = beacon.0;
                [
                    Pos::from((b[0], b[1], b[2])),
                    Pos::from((b[1], -b[0], b[2])),
                    Pos::from((-b[0], -b[1], b[2])),
                    Pos::from((-b[1], b[0], b[2])),
                    Pos::from((b[2], b[1], -b[0])),
                    Pos::from((b[2], -b[0], -b[1])),
                    Pos::from((b[2], -b[1], b[0])),
                    Pos::from((b[2], b[0], b[1])),
                    Pos::from((-b[0], b[1], -b[2])),
                    Pos::from((-b[1], -b[0], -b[2])),
                    Pos::from((b[0], -b[1], -b[2])),
                    Pos::from((b[1], b[0], -b[2])),
                    Pos::from((-b[2], b[1], b[0])),
                    Pos::from((-b[2], -b[0], b[1])),
                    Pos::from((-b[2], -b[1], -b[0])),
                    Pos::from((-b[2], b[0], -b[1])),
                    Pos::from((b[0], b[2], -b[1])),
                    Pos::from((b[1], b[2], b[0])),
                    Pos::from((-b[0], b[2], b[1])),
                    Pos::from((-b[1], b[2], -b[0])),
                    Pos::from((b[0], -b[2], b[1])),
                    Pos::from((b[1], -b[2], -b[0])),
                    Pos::from((-b[0], -b[2], -b[1])),
                    Pos::from((-b[1], -b[2], b[0])),
                ]
            })
            .collect_vec();
        
        let mighty_morphisms = std::array::from_fn(|i| {
            (0..morphisms.len()).map(|j| morphisms[j][i]).collect_vec()
        });

        let pos_pairs = beacons.iter().enumerate()
            .tuple_combinations()
            .map(|((ai, &a), (bi, &b))| {
                let key: (i64, i64, i64) = (a - b).0.into_iter()
                    .map(|it| it.abs())
                    .sorted_unstable()
                    .collect_tuple()
                    .unwrap();
                (key, (ai, bi))
            })
            .collect();

        Self { beacons, locations, morphisms, mighty_morphisms, pos_pairs }
    }
}

impl From<&str> for Scanner {
    fn from(s: &str) -> Self {
        let beacons = s.get_numbers::<i64>()
            .skip(1)
            .tuples::<(_, _, _)>()
            .map(|it| Pos::from(it))
            .collect();
        Self::new(beacons, Vec::new())
    }
}

#[derive(Debug, Clone)]
struct SharedSets<'a> {
    master: &'a Scanner,
    b: &'a Scanner,
    matches: Vec<(i64, i64, i64)>,
}

impl<'a> SharedSets<'a> {
    fn new(master: &'a Scanner, scanners: &'a [Scanner]) -> Self {
        scanners
            .iter()
            .map(|b| {
                let matches = master.pos_pairs.keys()
                    .filter(|&key| b.pos_pairs.contains_key(key))
                    .copied()
                    .collect();
                SharedSets { master, b, matches }               
            })
            .find(|it| it.matches.len() >= 66)
            .unwrap()
    }

    fn merge(self) -> Scanner {
        // Pick the first matches and use them to align and find offset
        let match_set = self.matches[0];
        let master_index_pair = self.master.pos_pairs.get(&match_set).unwrap();
        let b_index_pair = self.b.pos_pairs.get(&match_set).unwrap();
        let m1 = self.master.beacons[master_index_pair.0];
        let m2= self.master.beacons[master_index_pair.1];
        let m_diff = m1 - m2;
        let (match_idx, match_value) = self.b.morphisms[b_index_pair.0].iter().copied()
            .zip(self.b.morphisms[b_index_pair.1].iter().copied())
            .enumerate()
            .filter_map(|(idx, (b1, b2))| {
                if b1 - b2 == m_diff {
                    Some((idx, b1))
                } else if b2 - b1 == m_diff {
                    Some((idx, b2))
                } else {
                    None
                }
            })
            .next()
            .unwrap();
        
        let offset = m1 - match_value;
        let rotated_offset_beacons = self.b.mighty_morphisms[match_idx].iter()
            .map(|&it| it + offset);
        let beacons = self.master.beacons.iter().copied()
            .chain(rotated_offset_beacons)
            .unique()
            .collect();
        let mut locations = self.master.locations.clone();
        locations.push(offset);
        Scanner::new(beacons, locations)
    }
}

fn solve(input: &str) -> Solution {
    let mut scanners = input.split("\n\n").map(Scanner::from);
    let mut master = scanners.next().unwrap();
    let mut scanners = scanners.collect_vec();

    while !scanners.is_empty() {
        let shared_sets = SharedSets::new(&master, &scanners);
        let b_index = scanners.iter().position(|it| it == shared_sets.b).unwrap();
        master = shared_sets.merge();
        scanners.remove(b_index);
    }

    let furthest_dist = master.locations.iter().tuple_combinations()
        .map(|(a, &b)| a.manhattan_distance(b))
        .max()
        .unwrap();

    (master.beacons.len(), furthest_dist)
}

fn part1(input: &Solution) -> Output {
    input.0
}

fn part2(input: &Solution) -> Output {
    input.1
}

#[test]
fn default() {
    let input = get_input(21, 19).unwrap();
    let input = solve(&input);
    assert_eq!(378, part1(&input));
    assert_eq!(13148, part2(&input));
}

// Input parsed (23μs)
// 1. 378 (43ms)
// 2. 13148 (7μs)
// Total: 43ms