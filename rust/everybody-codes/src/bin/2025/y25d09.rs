#![feature(portable_simd)]
use core::simd::{Simd, u64x4};
use std::ops::{BitAnd, BitOr};

use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use utilities::{
    minmax::minmax,
    structs::stopwatch::{ReportDuration, Stopwatch},
};

type Input<'a> = &'a str;
#[cfg(feature = "simd")]
#[derive(Debug, Clone, Copy)]
struct Dna([u64x4; 2]);

#[cfg(not(feature = "simd"))]
#[derive(Debug, Clone, Copy)]
struct Dna([u64; 8]);

impl Dna {
    fn array(value: &str) -> [u64; 8] {
        let bytes = value.as_bytes();
        std::array::from_fn(|i| {
            let bytes = &bytes[i * 16..i * 16 + 16];
            bytes.iter().fold(0u64, |acc, &b| {
                (acc << 4) | match b {
                    b'A' => 1,
                    b'T' => 2,
                    b'C' => 4,
                    b'G' => 8,
                    b => panic!("{} not a valid symbol", b as char),
                }
            })
        })
    }
}

impl BitAnd for Dna {
    type Output = Dna;

    #[cfg(feature = "simd")]
    fn bitand(self, rhs: Self) -> Self::Output {
        let inner: [u64x4; 2] = std::array::from_fn(|i| {
            self.0[i] & rhs.0[i]
        });
        Self(inner)
    }

    #[cfg(not(feature = "simd"))]
    fn bitand(self, rhs: Self) -> Self::Output {
        let inner: [u64; 8] = std::array::from_fn(|i| {
            self.0[i] & rhs.0[i]
        });
        Self(inner)
    }

}

impl BitOr for Dna {
    type Output = Dna;

    #[cfg(feature = "simd")]
    fn bitor(self, rhs: Self) -> Self::Output {
        let inner: [u64x4; 2] = std::array::from_fn(|i| {
            self.0[i] | rhs.0[i]
        });
        Self(inner)
    }

    #[cfg(not(feature = "simd"))]
    fn bitor(self, rhs: Self) -> Self::Output {
        let inner: [u64; 8] = std::array::from_fn(|i| {
            self.0[i] | rhs.0[i]
        });
        Self(inner)
    }

}

impl From<&str> for Dna {
    #[cfg(feature = "simd")]
    fn from(value: &str) -> Self {
        let arr = Self::array(value);
        Dna([
            u64x4::from_slice(&arr[..4]),
            u64x4::from_slice(&arr[4..]),
        ])
    }

    #[cfg(not(feature = "simd"))]
    fn from(value: &str) -> Self {
        Dna(Self::array(value))
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

fn parse(input: Input<'_>) -> Vec<&'_ str> {
    input
        .lines()
        .map(|line| {
            let (_, dna) = line.split_once(':').unwrap();
            dna
        })
        .collect()
}

fn get_dna(input: Input) -> Vec<Dna> {
    input
        .lines()
        .map(|line| {
            let (_, dna) = line.split_once(':').unwrap();
            dna.into()
        })
        .collect()
}

fn part1(input: Input) -> usize {
    let (a, b, c) = input
        .lines()
        .map(|line| &line[2..])
        .collect_tuple()
        .unwrap();
    let dna: Vec<[char; 3]> = a
        .chars()
        .zip(b.chars())
        .zip(c.chars())
        .map(|((x, y), z)| [x, y, z])
        .collect();

    let mut sim = vec![Some([0usize; 2]); 3];
    for pos in dna {
        for scale in 0..3 {
            if let Some(similars) = &mut sim[scale] {
                let a = pos[scale];
                let b = pos[(scale + 1) % 3];
                let c = pos[(scale + 2) % 3];
                let mut matching = false;
                if a == b {
                    similars[0] += 1;
                    matching = true;
                }
                if a == c {
                    similars[1] += 1;
                    matching = true;
                }
                if !matching {
                    sim[scale] = None;
                }
            }
        }
    }
    sim.into_iter().flatten().map(|it| it[0] * it[1]).sum()
}

fn part2(input: Input) -> usize {
    let dna = parse(input);
    let mut sim_sum = 0;
    'outer: for scale in 0..dna.len() {
        let aa = dna[scale];
        'mid: for (p1, p2) in (0..dna.len())
            .tuple_combinations()
            .filter(|&(a, b)| a != scale && b != scale)
        {
            let bb = dna[p1];
            let mut b_sim = 0;
            let cc = dna[p2];
            let mut c_sim = 0;
            for ((a, b), c) in aa.chars().zip(bb.chars()).zip(cc.chars()) {
                let mut matching = false;
                if a == b {
                    b_sim += 1;
                    matching = true;
                }
                if a == c {
                    c_sim += 1;
                    matching = true;
                }
                if !matching {
                    continue 'mid;
                }
            }
            sim_sum += b_sim * c_sim;
            continue 'outer;
        }
    }
    sim_sum
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
}

fn part3(input: Input) -> usize {
    let dna = get_dna(input);
    let nuclear_families: Vec<(usize, usize, usize)> = (0..dna.len())
        .into_par_iter()
        .filter_map(|child| {
            let child_dna = dna[child];
            (0..dna.len()).tuple_combinations()
                .filter(|&(a, b)| a != child && b != child)
                .find(|&(p1, p2)| {
                    let p1_dna = dna[p1];
                    let p2_dna = dna[p2];
                    let parent_dna = p1_dna | p2_dna;
                    child_dna.0 == (child_dna & parent_dna).0
                })
                .map(|(p1, p2)| (child, p1, p2))
        })
        .collect();
    
    let mut tree = UnionFind::new(500);
    for (a, b, c) in nuclear_families {
        tree.union(a, b);
        tree.union(a, c);
    }
    let largest_group = tree.size.iter().enumerate()
        .max_by_key(|&(_, len)| len)
        .map(|(index, _)| index)
        .unwrap();

    tree.parent.iter().enumerate()
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

// Input parsed (63μs)
// 1. 6478 (9μs)
// 2. 316671 (2.102ms)
// 3. 40905 (233.631ms)
// Total: 235.816ms
