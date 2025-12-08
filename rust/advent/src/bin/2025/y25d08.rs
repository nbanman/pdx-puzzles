use advent::utilities::get_input::get_input;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::{
        coord::Coord,
        stopwatch::{ReportDuration, Stopwatch},
    },
};

const MAX_CONNECTIONS: usize = 1_000;

type Input = Vec<JBox>;
type Output = usize;
type JBox = Coord<usize, 3>;

fn main() {
    let input = get_input(25, 8).unwrap();
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!(
        "1. {} ({})",
        part1(&input, MAX_CONNECTIONS),
        stopwatch.lap().report()
    );
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

trait Junction {
    fn dist(&self, other: &Self) -> OrderedFloat<f32>;
}

impl Junction for JBox {
    fn dist(&self, other: &Self) -> OrderedFloat<f32> {
        let inner = self
            .0
            .iter()
            .zip(other.0)
            .map(|(a, b)| a.abs_diff(b).pow(2))
            .sum::<usize>();
        OrderedFloat((inner as f32).sqrt())
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
}

fn parse_input(input: &str) -> Input {
    input
        .get_numbers()
        .tuples()
        .map(|(x, y, z)| JBox::new([x, y, z]))
        .collect()
}

fn connections(junction_boxes: &Input) -> impl Iterator<Item = (usize, usize)> {
    junction_boxes
        .iter()
        .enumerate()
        .tuple_combinations()
        .map(|((a_idx, a), (b_idx, b))| (a.dist(b), a_idx, b_idx))
        .sorted_by_cached_key(|(dist, _, _)| *dist)
        .map(|(_, a, b)| (a, b))
}

fn part1(junction_boxes: &Input, max_connections: usize) -> Output {
    let mut lights = UnionFind::new(junction_boxes.len());

    for (a, b) in connections(junction_boxes).take(max_connections) {
        lights.union(a, b);
    }

    lights
        .size
        .iter()
        .sorted_unstable()
        .rev()
        .take(3)
        .product()
}

fn part2(junction_boxes: &Input) -> Output {
    let mut lights = UnionFind::new(junction_boxes.len());
    for (a, b) in connections(junction_boxes) {
        lights.union(a, b);
        let root_len = *lights
            .size
            .iter()
            .max()
            .unwrap();
        if root_len == junction_boxes.len() {
            return junction_boxes[a].x() * junction_boxes[b].x();
        }
    }
    unreachable!()
}

// Input parsed (52μs)
// 1. 181584 (55.922ms)
// 2. 8465902405 (56.054ms)
// Total: 112.041ms

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_INPUT: &str = r"
        162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689
    ";

    #[test]
    fn default() {
        let input = get_input(25, 8).unwrap();
        let input = parse_input(&input);
        assert_eq!(181584, part1(&input, MAX_CONNECTIONS));
        // assert_eq!(YY, part2(&input));
    }

    #[test]
    fn test1() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(40, part1(&input, 10));
    }

    #[test]
    fn test2() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(25272, part2(&input));
    }
}
