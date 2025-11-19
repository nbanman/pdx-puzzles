#![feature(portable_simd)]

#[cfg(feature = "simd")]
use core::simd::u64x4;
#[cfg(feature = "simd")]
use std::simd::num::SimdUint;

use everybody_codes::utilities::inputs::get_event_inputs;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use std::ops::{BitAnd, BitOr};
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
#[cfg(feature = "simd")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Dna([u64x4; 2]);

#[cfg(not(feature = "simd"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Dna([u64; 8]);

impl Dna {
    fn array(value: &str) -> [u64; 8] {
        let (_, value) = value.split_once(':').unwrap();
        let bytes = value.as_bytes();
        std::array::from_fn(|i| {
            let bytes = &bytes[i * 16..i * 16 + 16];
            bytes.iter().fold(0u64, |acc, &b| {
                (acc << 4)
                    | match b {
                        b'A' => 1,
                        b'T' => 2,
                        b'C' => 4,
                        b'G' => 8,
                        b => panic!("{} not a valid symbol", b as char),
                    }
            })
        })
    }

    #[cfg(feature = "simd")]
    fn similarity(&self, other: &Dna) -> usize {
        let meld = *self & *other;
        meld.0
            .into_iter()
            .map(|it| it.count_ones().reduce_sum() as usize)
            .sum()
    }

    #[cfg(not(feature = "simd"))]
    fn similarity(&self, other: &Dna) -> usize {
        let meld = *self & *other;
        meld.0.into_iter().map(|it| it.count_ones() as usize).sum()
    }
}

impl BitAnd for Dna {
    type Output = Dna;

    #[cfg(feature = "simd")]
    fn bitand(self, rhs: Self) -> Self::Output {
        let inner: [u64x4; 2] = std::array::from_fn(|i| self.0[i] & rhs.0[i]);
        Self(inner)
    }

    #[cfg(not(feature = "simd"))]
    fn bitand(self, rhs: Self) -> Self::Output {
        let inner: [u64; 8] = std::array::from_fn(|i| self.0[i] & rhs.0[i]);
        Self(inner)
    }
}

impl BitOr for Dna {
    type Output = Dna;

    #[cfg(feature = "simd")]
    fn bitor(self, rhs: Self) -> Self::Output {
        let inner: [u64x4; 2] = std::array::from_fn(|i| self.0[i] | rhs.0[i]);
        Self(inner)
    }

    #[cfg(not(feature = "simd"))]
    fn bitor(self, rhs: Self) -> Self::Output {
        let inner: [u64; 8] = std::array::from_fn(|i| self.0[i] | rhs.0[i]);
        Self(inner)
    }
}

impl From<&str> for Dna {
    #[cfg(feature = "simd")]
    fn from(value: &str) -> Self {
        let arr = Self::array(value);
        Dna([u64x4::from_slice(&arr[..4]), u64x4::from_slice(&arr[4..])])
    }

    #[cfg(not(feature = "simd"))]
    fn from(value: &str) -> Self {
        Dna(Self::array(value))
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let x = self.find(x);
        let y = self.find(y);

        if x == y {
            return false;
        }
        if self.size[x] >= self.size[y] {
            self.parent[y] = x;
            self.size[x] += self.size[y];
        } else {
            self.parent[x] = y;
            self.size[y] += self.size[x];
        }
        true
    }

    fn update(&mut self) {
        for i in 0..self.parent.len() {
            self.find(i);
        }
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(25, 9);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn get_dna(input: Input) -> Vec<Dna> {
    input.lines().par_bridge().map(Dna::from).collect()
}

fn get_families(dna: &[Dna], child: usize, child_dna: &Dna) -> Option<(usize, usize)> {
    (0..dna.len())
        .filter(|&it| it != child)
        .map(|p1| {
            let p1_dna = &dna[p1];
            (p1, child_dna.similarity(p1_dna))
        })
        .filter(|&(_, p1_sim)| p1_sim > 60)
        .flat_map(|(p1, _)| {
            (0..dna.len())
                .filter(move |p2| *p2 != child && *p2 != p1)
                .map(move |p2| (p1, p2))
        })
        .find(|&(p1, p2)| {
            let p1_dna = dna[p1];
            let p2_dna = dna[p2];
            let parent_dna = p1_dna | p2_dna;
            *child_dna == (*child_dna & parent_dna)
        })
}

fn part1(input: Input) -> usize {
    let dna = get_dna(input);
    (0..dna.len())
        .filter_map(|i| {
            let child = dna[i];
            let p1 = dna[(i + 1) % dna.len()];
            let p2 = dna[(i + 2) % dna.len()];
            if child & (p1 | p2) == child {
                Some(child.similarity(&p1) * child.similarity(&p2))
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

fn part2(input: Input) -> usize {
    let dna = get_dna(input);
    (0..dna.len())
        .into_par_iter()
        .filter_map(|child| {
            let child_dna = dna[child];
            get_families(&dna, child, &child_dna)
                .map(|(p1, p2)| {
                    child_dna.similarity(&dna[p1]) * child_dna.similarity(&dna[p2])
                })
        })
        .sum()
}

fn part3(input: Input) -> usize {
    let dna = get_dna(input);
    let nuclear_families: Vec<(usize, usize, usize)> = (0..dna.len())
        .into_par_iter()
        .filter_map(|child| {
            let child_dna = dna[child];
            get_families(&dna, child, &child_dna).map(|(p1, p2)| (child, p1, p2))
        })
        .collect();

    let mut tree = UnionFind::new(500);
    for (a, b, c) in nuclear_families {
        tree.union(a, b);
        tree.union(a, c);
    }

    let largest_group = tree
        .size
        .iter()
        .enumerate()
        .max_by_key(|&(_, len)| len)
        .map(|(index, _)| index)
        .unwrap();

    tree.update();

    tree.parent
        .iter()
        .enumerate()
        .filter(|&(_, &parent)| parent == largest_group)
        .map(|(index, _)| index + 1)
        .sum()
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(25, 9);
    assert_eq!(6478, part1(&input1));
    assert_eq!(316671, part2(&input2));
    assert_eq!(40905, part3(&input3));
}

// Input parsed (60μs)
// 1. 6478 (541μs)
// 2. 316671 (90μs)
// 3. 40592 (419μs)
// Total: 1.116ms